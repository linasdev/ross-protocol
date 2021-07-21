use bxcan::{Can, Instance};
use nb::block;

use crate::ross_frame::*;
use crate::ross_packet::*;

#[derive(Debug, PartialEq)]
pub enum RossCanError {
    BufferOverrun,
    NoPacketReceived,
    MailboxFull,
    BuilderError(RossPacketBuilderError),
    FrameError(RossFrameError),
}

pub struct RossCan<I: Instance> {
    can: Can<I>,
    packet_builder: Option<RossPacketBuilder>,
}

impl<I: Instance> RossCan<I> {
    pub fn new(can: Can<I>) -> Self {
        RossCan {
            can,
            packet_builder: None,
        }
    }

    pub fn try_get_packet(&mut self) -> Result<RossPacket, RossCanError> {
        loop {
            match self.can.receive() {
                Ok(frame) => {
                    let ross_frame = match RossFrame::from_bxcan_frame(frame) {
                        Ok(frame) => frame,
                        Err(err) => return Err(RossCanError::FrameError(err)),
                    };

                    if let Some(ref mut packet_builder) = self.packet_builder {
                        if let Err(err) = packet_builder.add_frame(ross_frame) {
                            self.packet_builder = None;

                            return Err(RossCanError::BuilderError(err));
                        }
                    } else {
                        self.packet_builder = match RossPacketBuilder::new(ross_frame) {
                            Ok(builder) => Some(builder),
                            Err(err) => return Err(RossCanError::BuilderError(err)),
                        };
                    }

                    if let Some(ref mut packet_builder) = self.packet_builder {
                        if packet_builder.frames_left() == 0 {
                            let packet = match packet_builder.build() {
                                Ok(packet) => packet,
                                Err(err) => return Err(RossCanError::BuilderError(err)),
                            };

                            self.packet_builder = None;

                            return Ok(packet);
                        }
                    }
                }
                Err(_) => break,
            }
        }

        Err(RossCanError::NoPacketReceived)
    }

    pub fn try_send_packet(&mut self, packet: &RossPacket) -> Result<(), RossCanError> {
        let frames = packet.to_frames();

        for frame in frames {
            if let Ok(Some(_)) = block!(self.can.transmit(&frame.to_bxcan_frame())) {
                return Err(RossCanError::MailboxFull);
            }
        }

        Ok(())
    }
}
