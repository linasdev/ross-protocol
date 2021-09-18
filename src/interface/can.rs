use bxcan::{Can as BxCan, Instance};
use nb::block;

use crate::frame::*;
use crate::interface::*;
use crate::packet::*;

#[derive(Debug, PartialEq)]
pub enum CanError {
    BufferOverrun,
    MailboxFull,
}

pub struct Can<I: Instance> {
    can: BxCan<I>,
    packet_builder: Option<PacketBuilder>,
}

impl<I: Instance> Can<I> {
    pub fn new(can: BxCan<I>) -> Self {
        Can {
            can,
            packet_builder: None,
        }
    }
}

impl<I: Instance> Interface for Can<I> {
    fn try_get_packet(&mut self) -> Result<Packet, InterfaceError> {
        loop {
            match self.can.receive() {
                Ok(frame) => {
                    let ross_frame = match Frame::from_bxcan_frame(frame) {
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
                Err(_) => break,
            }
        }

        Err(InterfaceError::NoPacketReceived)
    }

    fn try_send_packet(&mut self, packet: &Packet) -> Result<(), InterfaceError> {
        let frames = packet.to_frames();

        for frame in frames {
            if let Ok(Some(_)) = block!(self.can.transmit(&frame.to_bxcan_frame())) {
                return Err(InterfaceError::CanError(CanError::MailboxFull));
            }
        }

        Ok(())
    }
}
