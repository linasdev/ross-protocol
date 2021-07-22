use alloc::vec;
use alloc::vec::Vec;

use crate::ross_frame::{RossFrame, RossFrameId};

#[derive(Debug, PartialEq)]
pub struct RossPacket {
    /// If this flag is set, the packet is considered to be an error packet
    pub is_error: bool,
    /// Transmitting device's address
    pub device_address: u16,
    /// Packet data
    pub data: Vec<u8>,
}

impl RossPacket {
    pub fn to_frames(&self) -> Vec<RossFrame> {
        if self.data.len() <= 8 {
            let mut data = [0; 8];

            for i in 0..self.data.len() {
                data[i] = self.data[i];
            }

            return vec![RossFrame {
                not_error_flag: !self.is_error,
                start_frame_flag: true,
                multi_frame_flag: false,
                frame_id: RossFrameId::LastFrameId(0),
                device_address: self.device_address,
                data_len: self.data.len() as u8,
                data,
            }];
        }

        let frame_count = (self.data.len() - 1) / 7 + 1;
        let mut frames = vec![];

        for i in 0..frame_count {
            let data_len = if i == frame_count - 1 {
                if self.data.len() % 7 == 0 {
                    8
                } else {
                    self.data.len() % 7 + 1
                }
            } else {
                8
            };

            let mut data = [0u8; 8];

            if i == 0 {
                data[0] = ((frame_count - 1) & 0xff) as u8;
            } else {
                data[0] = (i & 0xff) as u8;
            }

            for j in 0..(data_len - 1) {
                data[j + 1] = self.data[i * 7 + j];
            }

            frames.push(RossFrame {
                not_error_flag: !self.is_error,
                start_frame_flag: i == 0,
                multi_frame_flag: true,
                frame_id: if i == 0 {
                    RossFrameId::LastFrameId(frame_count as u16 - 1)
                } else {
                    RossFrameId::CurrentFrameId(i as u16)
                },
                device_address: self.device_address,
                data_len: data_len as u8,
                data,
            });
        }

        return frames;
    }
}

#[derive(Debug, PartialEq)]
pub enum RossPacketBuilderError {
    /// Frame supplied was not the next frame in the sequence+
    OutOfOrder,
    /// Expected a multi frame packet but a single frame packet was given
    SingleFramePacket,
    /// Expected less frames to be in the packet+
    TooManyFrames,
    /// Expected an error frame but a data frame was given or the other way around+
    WrongFrameType,
    /// The frame given was transmitted by a different device than the previous frames+
    DeviceAddressMismatch,
    /// Expected more frames+
    MissingFrames,
}

#[derive(Debug, PartialEq)]
pub struct RossPacketBuilder {
    is_error: bool,
    expected_frame_count: u16,
    device_address: u16,
    frames: Vec<RossFrame>,
}

impl RossPacketBuilder {
    pub fn expected_frame_count(&self) -> u16 {
        self.expected_frame_count
    }

    pub fn frame_count(&self) -> u16 {
        self.frames.len() as u16
    }

    pub fn frames_left(&self) -> u16 {
        self.expected_frame_count() - self.frame_count()
    }

    pub fn new(frame: RossFrame) -> Result<Self, RossPacketBuilderError> {
        if !frame.start_frame_flag {
            return Err(RossPacketBuilderError::OutOfOrder);
        }

        let expected_frame_count = if let RossFrameId::LastFrameId(last_frame_id) = frame.frame_id {
            last_frame_id + 1
        } else {
            return Err(RossPacketBuilderError::OutOfOrder);
        };

        Ok(RossPacketBuilder {
            is_error: !frame.not_error_flag,
            expected_frame_count,
            device_address: frame.device_address,
            frames: vec![frame],
        })
    }

    pub fn add_frame(&mut self, frame: RossFrame) -> Result<(), RossPacketBuilderError> {
        if !frame.not_error_flag != self.is_error {
            return Err(RossPacketBuilderError::WrongFrameType);
        }

        if frame.device_address != self.device_address {
            return Err(RossPacketBuilderError::DeviceAddressMismatch);
        }

        if frame.start_frame_flag {
            return Err(RossPacketBuilderError::OutOfOrder);
        }

        if !frame.multi_frame_flag {
            return Err(RossPacketBuilderError::SingleFramePacket);
        }

        if let RossFrameId::CurrentFrameId(frame_id) = frame.frame_id {
            if frame_id != self.frames.len() as u16 {
                return Err(RossPacketBuilderError::OutOfOrder);
            }

            if frame_id >= self.expected_frame_count {
                return Err(RossPacketBuilderError::TooManyFrames);
            }
        } else {
            return Err(RossPacketBuilderError::OutOfOrder);
        }

        self.frames.push(frame);

        Ok(())
    }

    pub fn build(&self) -> Result<RossPacket, RossPacketBuilderError> {
        if self.frames.len() != self.expected_frame_count as usize {
            return Err(RossPacketBuilderError::MissingFrames);
        }

        let mut data = vec![];

        for frame in self.frames.iter() {
            let start_index = if frame.multi_frame_flag { 1 } else { 0 };

            for i in start_index..frame.data_len {
                data.push(frame.data[i as usize]);
            }
        }

        Ok(RossPacket {
            is_error: self.is_error,
            device_address: self.device_address,
            data,
        })
    }
}
