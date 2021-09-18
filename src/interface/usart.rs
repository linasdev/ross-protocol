use alloc::vec;
use embedded_hal::serial::{Read, Write};
use nb::block;

use crate::frame::*;
use crate::interface::*;
use crate::packet::*;

#[derive(Debug, PartialEq)]
pub enum UsartError {
    ReadError,
}

pub struct Usart<S: Read<u8> + Write<u8>> {
    serial: S,
    packet_builder: Option<PacketBuilder>,
}

impl<S: Read<u8> + Write<u8>> Usart<S> {
    pub fn new(serial: S) -> Self {
        Usart {
            serial,
            packet_builder: None,
        }
    }
}

impl<S: Read<u8> + Write<u8>> Interface for Usart<S> {
    fn try_get_packet(&mut self) -> Result<Packet, InterfaceError> {
        loop {
            match self.serial.read() {
                Ok(frame_start) => {
                    if frame_start == 0x00 {
                        let mut frame = vec![];

                        let expected_length = match block!(self.serial.read()) {
                            Ok(length) => length,
                            Err(_) => {
                                return Err(InterfaceError::UsartError(UsartError::ReadError))
                            }
                        };

                        loop {
                            match block!(self.serial.read()) {
                                Ok(byte) => frame.push(byte),
                                Err(_) => {
                                    return Err(InterfaceError::UsartError(UsartError::ReadError))
                                }
                            }

                            if frame.len() == expected_length as usize {
                                break;
                            }
                        }

                        let ross_frame = match Frame::from_usart_frame(frame) {
                            Ok(frame) => frame,
                            Err(err) => return Err(InterfaceError::FrameError(err)),
                        };

                        if let Some(ref mut packet_builder) = self.packet_builder {
                            if let Err(err) = packet_builder.add_frame(ross_frame) {
                                self.packet_builder = None;

                                return Err(InterfaceError::BuilderError(err));
                            }
                        } else {
                            self.packet_builder = match PacketBuilder::new(ross_frame) {
                                Ok(builder) => Some(builder),
                                Err(err) => return Err(InterfaceError::BuilderError(err)),
                            };
                        }

                        if let Some(ref mut packet_builder) = self.packet_builder {
                            if packet_builder.frames_left() == 0 {
                                let packet = match packet_builder.build() {
                                    Ok(packet) => packet,
                                    Err(err) => return Err(InterfaceError::BuilderError(err)),
                                };

                                self.packet_builder = None;

                                return Ok(packet);
                            }
                        }
                    }
                }
                Err(_) => break,
            }
        }

        Err(InterfaceError::NoPacketReceived)
    }

    fn try_send_packet(&mut self, packet: &Packet) -> Result<(), InterfaceError> {
        let frames = packet.to_frames();

        for frame in frames {
            let _ = block!(self.serial.write(0x00));

            let usart_frame = frame.to_usart_frame();

            let _ = block!(self.serial.write(usart_frame.len() as u8));

            for byte in frame.to_usart_frame().iter() {
                let _ = block!(self.serial.write(*byte));
            }
        }

        Ok(())
    }
}
