use alloc::vec;
use alloc::vec::Vec;

use crate::frame::{Frame, FrameId};

#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    const FRAME_DATA: [u8; 8] = [0x01; 8];
    const SINGLE_FRAME_PACKET: Frame = Frame {
        not_error_flag: true,
        start_frame_flag: true,
        multi_frame_flag: false,
        frame_id: FrameId::LastFrameId(0x00),
        device_address: 0x0101,
        data_len: 8,
        data: FRAME_DATA,
    };

    const MULTI_FRAME_PACKET_DATA: [u8; 14] = [0x01; 14];
    const MULTI_FRAME_PACKET1: Frame = Frame {
        not_error_flag: true,
        start_frame_flag: true,
        multi_frame_flag: true,
        frame_id: FrameId::LastFrameId(0x01),
        device_address: 0x0101,
        data_len: 8,
        data: FRAME_DATA,
    };
    const MULTI_FRAME_PACKET2: Frame = Frame {
        not_error_flag: true,
        start_frame_flag: false,
        multi_frame_flag: true,
        frame_id: FrameId::CurrentFrameId(0x01),
        device_address: 0x0101,
        data_len: 8,
        data: FRAME_DATA,
    };

    #[test]
    fn to_frames_test() {
        let packet = Packet {
            is_error: !MULTI_FRAME_PACKET1.not_error_flag,
            device_address: MULTI_FRAME_PACKET1.device_address,
            data: [0x01; 14].to_vec(),
        };

        let frames = packet.to_frames();

        assert_eq!(frames.len(), 2);
        assert_eq!(frames[0], MULTI_FRAME_PACKET1);
        assert_eq!(frames[1], MULTI_FRAME_PACKET2);
    }

    #[test]
    fn new_test() {
        let packet_builder = PacketBuilder::new(SINGLE_FRAME_PACKET).unwrap();
        let packet = packet_builder.build().unwrap();

        assert_eq!(packet.is_error, !SINGLE_FRAME_PACKET.not_error_flag);
        assert_eq!(packet.device_address, SINGLE_FRAME_PACKET.device_address);
        assert_eq!(packet.data, FRAME_DATA);
    }

    #[test]
    #[should_panic]
    fn new_out_of_order_test() {
        PacketBuilder::new(MULTI_FRAME_PACKET2).unwrap();
    }

    #[test]
    fn add_frame_test() {
        let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.add_frame(MULTI_FRAME_PACKET2).unwrap();
        let packet = packet_builder.build().unwrap();

        assert_eq!(packet.is_error, !MULTI_FRAME_PACKET1.not_error_flag);
        assert_eq!(packet.device_address, MULTI_FRAME_PACKET1.device_address);
        assert_eq!(packet.data, MULTI_FRAME_PACKET_DATA);
    }

    #[test]
    #[should_panic]
    fn add_frame_wrong_frame_type_test() {
        let mut error_frame = MULTI_FRAME_PACKET2;
        error_frame.not_error_flag = false;

        let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.add_frame(error_frame).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_frame_device_address_mismatch_rwar() {
        let mut wrong_device_frame = MULTI_FRAME_PACKET2;
        wrong_device_frame.device_address = 0xffff;

        let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.add_frame(wrong_device_frame).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_frame_single_frame_packet_test() {
        let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.add_frame(SINGLE_FRAME_PACKET).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_frame_too_many_frames_test() {
        let extra_frame = Frame {
            not_error_flag: true,
            start_frame_flag: false,
            multi_frame_flag: true,
            frame_id: FrameId::CurrentFrameId(0x02),
            device_address: 0x0101,
            data_len: 8,
            data: FRAME_DATA,
        };

        let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.add_frame(MULTI_FRAME_PACKET2).unwrap();
        packet_builder.add_frame(extra_frame).unwrap();
    }

    #[test]
    #[should_panic]
    fn build_missing_frames_test() {
        let packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
        packet_builder.build().unwrap();
    }
}
