use alloc::vec;
use alloc::vec::Vec;

use crate::frame::{Frame, FrameId};

#[derive(Debug, PartialEq)]
pub struct Packet {
    /// If this flag is set, the packet is considered to be an error packet
    pub is_error: bool,
    /// Transmitting device's address
    pub device_address: u16,
    /// Packet data
    pub data: Vec<u8>,
}

impl Packet {
    pub fn to_frames(&self) -> Vec<Frame> {
        if self.data.len() <= 8 {
            let mut data = [0; 8];

            for i in 0..self.data.len() {
                data[i] = self.data[i];
            }

            return vec![Frame {
                not_error_flag: !self.is_error,
                start_frame_flag: true,
                multi_frame_flag: false,
                frame_id: FrameId::LastFrameId(0),
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

            frames.push(Frame {
                not_error_flag: !self.is_error,
                start_frame_flag: i == 0,
                multi_frame_flag: true,
                frame_id: if i == 0 {
                    FrameId::LastFrameId(frame_count as u16 - 1)
                } else {
                    FrameId::CurrentFrameId(i as u16)
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
pub enum PacketBuilderError {
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
pub struct PacketBuilder {
    is_error: bool,
    expected_frame_count: u16,
    device_address: u16,
    frames: Vec<Frame>,
}

impl PacketBuilder {
    pub fn expected_frame_count(&self) -> u16 {
        self.expected_frame_count
    }

    pub fn frame_count(&self) -> u16 {
        self.frames.len() as u16
    }

    pub fn frames_left(&self) -> u16 {
        self.expected_frame_count() - self.frame_count()
    }

    pub fn new(frame: Frame) -> Result<Self, PacketBuilderError> {
        if !frame.start_frame_flag {
            return Err(PacketBuilderError::OutOfOrder);
        }

        let expected_frame_count = if let FrameId::LastFrameId(last_frame_id) = frame.frame_id {
            last_frame_id + 1
        } else {
            return Err(PacketBuilderError::OutOfOrder);
        };

        Ok(PacketBuilder {
            is_error: !frame.not_error_flag,
            expected_frame_count,
            device_address: frame.device_address,
            frames: vec![frame],
        })
    }

    pub fn add_frame(&mut self, frame: Frame) -> Result<(), PacketBuilderError> {
        if !frame.not_error_flag != self.is_error {
            return Err(PacketBuilderError::WrongFrameType);
        }

        if frame.device_address != self.device_address {
            return Err(PacketBuilderError::DeviceAddressMismatch);
        }

        if frame.start_frame_flag {
            return Err(PacketBuilderError::OutOfOrder);
        }

        if !frame.multi_frame_flag {
            return Err(PacketBuilderError::SingleFramePacket);
        }

        if let FrameId::CurrentFrameId(frame_id) = frame.frame_id {
            if frame_id != self.frames.len() as u16 {
                return Err(PacketBuilderError::OutOfOrder);
            }

            if frame_id >= self.expected_frame_count {
                return Err(PacketBuilderError::TooManyFrames);
            }
        } else {
            return Err(PacketBuilderError::OutOfOrder);
        }

        self.frames.push(frame);

        Ok(())
    }

    pub fn build(&self) -> Result<Packet, PacketBuilderError> {
        if self.frames.len() != self.expected_frame_count as usize {
            return Err(PacketBuilderError::MissingFrames);
        }

        let mut data = vec![];

        for frame in self.frames.iter() {
            let start_index = if frame.multi_frame_flag { 1 } else { 0 };

            for i in start_index..frame.data_len {
                data.push(frame.data[i as usize]);
            }
        }

        Ok(Packet {
            is_error: self.is_error,
            device_address: self.device_address,
            data,
        })
    }
}
