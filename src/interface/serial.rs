use serialport::SerialPort;
use std::io::Error as IOError;

use crate::frame::*;
use crate::interface::*;
use crate::packet::*;

#[derive(Debug)]
pub enum SerialError {
    ReadError(IOError),
    WriteError(IOError),
    BuilderError(PacketBuilderError),
    FrameError(FrameError),
}

pub struct Serial {
    port: Box<dyn SerialPort>,
    packet_builder: Option<PacketBuilder>,
}

impl Serial {
    pub fn new(port: Box<dyn SerialPort>) -> Self {
        Serial {
            port,
            packet_builder: None,
        }
    }
}

impl Interface for Serial {
    fn try_get_packet(&mut self) -> Result<Packet, InterfaceError> {
        loop {
            let mut buf = [0x00; 1];

            match self.port.read_exact(&mut buf[..]) {
                Ok(_) => {
                    if buf[0] == 0x00 {
                        let expected_length = match self.port.read_exact(&mut buf[..]) {
                            Ok(_) => buf[0],
                            Err(err) => {
                                return Err(InterfaceError::SerialError(SerialError::ReadError(
                                    err,
                                )))
                            }
                        };

                        let mut frame = vec![0x00; expected_length as usize];

                        if let Err(err) = self.port.read_exact(&mut frame[..]) {
                            return Err(InterfaceError::SerialError(SerialError::ReadError(err)));
                        }

                        let ross_frame = match Frame::from_usart_frame(frame) {
                            Ok(frame) => frame,
                            Err(err) => {
                                return Err(InterfaceError::FrameError(err));
                            }
                        };

                        if let Some(ref mut packet_builder) = self.packet_builder {
                            if let Err(err) = packet_builder.add_frame(ross_frame) {
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
                Err(err) => return Err(InterfaceError::NoPacketReceived),
            }
        }
    }

    fn try_send_packet(&mut self, packet: &Packet) -> Result<(), InterfaceError> {
        for frame in packet.to_frames().iter() {
            let frame_buf = frame.to_usart_frame();

            let buf = [0x00; 1];
            if let Err(err) = self.port.write(&buf) {
                return Err(InterfaceError::SerialError(SerialError::WriteError(err)));
            }

            let buf = [frame_buf.len() as u8; 1];
            if let Err(err) = self.port.write(&buf) {
                return Err(InterfaceError::SerialError(SerialError::WriteError(err)));
            }

            if let Err(err) = self.port.write(&frame_buf) {
                return Err(InterfaceError::SerialError(SerialError::WriteError(err)));
            }
        }

        if let Err(err) = self.port.flush() {
            Err(InterfaceError::SerialError(SerialError::WriteError(err)))
        } else {
            Ok(())
        }
    }
}
