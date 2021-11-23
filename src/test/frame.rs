use alloc::vec;
use bxcan::{ExtendedId, Frame as BxFrame};

use crate::frame::{Frame, FrameId};

const FRAME_ID: u32 = 0x1405_5555;
const FRAME_DATA: [u8; 8] = [0x55; 8];
const FRAME: Frame = Frame {
    not_error_flag: true,
    start_frame_flag: false,
    multi_frame_flag: true,
    frame_id: FrameId::CurrentFrameId(0x0555),
    device_address: 0x5555,
    data_len: 8,
    data: FRAME_DATA,
};

#[test]
fn from_bxcan_frame_test() {
    let bxcan_frame = BxFrame::new_data(ExtendedId::new(FRAME_ID).unwrap(), FRAME_DATA);
    let ross_frame = Frame::from_bxcan_frame(bxcan_frame).unwrap();

    assert_eq!(ross_frame, FRAME);
}

#[test]
fn to_bxcan_frame_test() {
    let bxcan_frame = FRAME.to_bxcan_frame();
    let bxcan_frame_expected = BxFrame::new_data(ExtendedId::new(FRAME_ID).unwrap(), FRAME_DATA);

    assert_eq!(bxcan_frame, bxcan_frame_expected);
}

#[test]
fn from_usart_frame_test() {
    let usart_frame = vec![
        0x0e, // COBS
        0xa5, // byte 0
        0x55, // FRAME_ID
        0x55, // DEVICE_ADDRESS
        0x55, // DEVICE_ADDRESS
        0x08, // DATA_LEN
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
    ];

    let ross_frame = Frame::from_usart_frame(usart_frame).unwrap();

    assert_eq!(ross_frame, FRAME);
}

#[test]
fn to_usart_frame_test() {
    let usart_frame = FRAME.to_usart_frame();
    let usart_frame_expected = vec![
        0x0e, // COBS
        0xa5, // byte 0
        0x55, // FRAME_ID
        0x55, // DEVICE_ADDRESS
        0x55, // DEVICE_ADDRESS
        0x08, // DATA_LEN
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
        0x55, // DATA
    ];

    assert_eq!(usart_frame, usart_frame_expected);
}
