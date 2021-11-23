use crate::frame::{Frame, FrameId};
use crate::packet::*;

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
fn new_test_out_of_order() {
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
fn add_frame_test_wrong_frame_type() {
    let mut error_frame = MULTI_FRAME_PACKET2;
    error_frame.not_error_flag = false;

    let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
    packet_builder.add_frame(error_frame).unwrap();
}

#[test]
#[should_panic]
fn add_frame_test_device_address_mismatch() {
    let mut wrong_device_frame = MULTI_FRAME_PACKET2;
    wrong_device_frame.device_address = 0xffff;

    let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
    packet_builder.add_frame(wrong_device_frame).unwrap();
}

#[test]
#[should_panic]
fn add_frame_test_single_frame_packet() {
    let mut packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
    packet_builder.add_frame(SINGLE_FRAME_PACKET).unwrap();
}

#[test]
#[should_panic]
fn add_frame_test_too_many_frames() {
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
fn build_test_missing_frames() {
    let packet_builder = PacketBuilder::new(MULTI_FRAME_PACKET1).unwrap();
    packet_builder.build().unwrap();
}
