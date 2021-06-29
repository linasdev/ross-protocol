use alloc::vec;
use alloc::vec::Vec;

use crate::ross_frame::{RossFrame, RossFrameId};

pub struct RossPacket {
    /// If this flag is set, the packet is considered to be an error packet
    pub is_error: bool,
    /// Number of frames in the packet
    pub frame_count: u16,
    /// Transmitting device's address
    pub device_address: u16,
    /// Packet data
    pub data: Vec<u8>,
}

#[derive(Debug)]
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

pub struct RossPacketBuilder {
    is_error: bool,
    expected_frame_count: u16,
    device_address: u16,
    frames: Vec<RossFrame>,
}

impl RossPacketBuilder {
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
            frame_count: self.expected_frame_count,
            device_address: self.device_address,
            data,
        })
    }
}
