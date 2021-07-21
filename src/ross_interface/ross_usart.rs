use alloc::vec;
use embedded_hal::serial::{Read, Write};
use nb::block;

use crate::ross_frame::*;
use crate::ross_packet::*;

#[derive(Debug, PartialEq)]
pub enum RossUsartError {
    NoPacketReceived,
    ReadError,
    BuilderError(RossPacketBuilderError),
    FrameError(RossFrameError),
}

pub struct RossUsart<S: Read<u8> + Write<u8>> {
    serial: S,
    packet_builder: Option<RossPacketBuilder>,
}

impl<S: Read<u8> + Write<u8>> RossUsart<S> {
    pub fn new(serial: S) -> Self {
        RossUsart {
            serial,
            packet_builder: None,
        }
    }

    pub fn try_get_packet(&mut self) -> Result<RossPacket, RossUsartError> {
        loop {
            match self.serial.read() {
                Ok(frame_start) => {
                    if frame_start == 0x00 {
                        let mut frame = vec![];

                        let expected_length = match block!(self.serial.read()) {
                            Ok(length) => length,
                            Err(_) => return Err(RossUsartError::ReadError),
                        };

                        loop {
                            match block!(self.serial.read()) {
                                Ok(byte) => frame.push(byte),
                                Err(_) => return Err(RossUsartError::ReadError),
                            }

                            if frame.len() == expected_length as usize {
                                break;
                            }
                        }

                        let ross_frame = match RossFrame::from_usart_frame(frame) {
                            Ok(frame) => frame,
                            Err(err) => return Err(RossUsartError::FrameError(err)),
                        };

                        if let Some(ref mut packet_builder) = self.packet_builder {
                            if let Err(err) = packet_builder.add_frame(ross_frame) {
                                self.packet_builder = None;

                                return Err(RossUsartError::BuilderError(err));
                            }
                        } else {
                            self.packet_builder = match RossPacketBuilder::new(ross_frame) {
                                Ok(builder) => Some(builder),
                                Err(err) => return Err(RossUsartError::BuilderError(err)),
                            };
                        }

                        if let Some(ref mut packet_builder) = self.packet_builder {
                            if packet_builder.frames_left() == 0 {
                                let packet = match packet_builder.build() {
                                    Ok(packet) => packet,
                                    Err(err) => return Err(RossUsartError::BuilderError(err)),
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

        Err(RossUsartError::NoPacketReceived)
    }

    pub fn try_send_packet(&mut self, packet: &RossPacket) -> Result<(), RossUsartError> {
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
