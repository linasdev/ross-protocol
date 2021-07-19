use alloc::vec;
use bxcan::{ExtendedId, Frame};

use crate::ross_frame::{RossFrame, RossFrameId};

const FRAME_ID: u32 = 0x1405_5555;
const FRAME_DATA: [u8; 8] = [0x55; 8];
const ROSS_FRAME: RossFrame = RossFrame {
    not_error_flag: true,
    start_frame_flag: false,
    multi_frame_flag: true,
    frame_id: RossFrameId::CurrentFrameId(0x0555),
    device_address: 0x5555,
    data_len: 8,
    data: FRAME_DATA,
};

#[test]
fn from_bxcan_frame_test() {
    let bxcan_frame = Frame::new_data(ExtendedId::new(FRAME_ID).unwrap(), FRAME_DATA);
    let ross_frame = RossFrame::from_bxcan_frame(bxcan_frame).unwrap();

    assert_eq!(ross_frame, ROSS_FRAME);
}

#[test]
fn to_bxcan_frame_test() {
    let bxcan_frame = ROSS_FRAME.to_bxcan_frame();
    let bxcan_frame_expected = Frame::new_data(ExtendedId::new(FRAME_ID).unwrap(), FRAME_DATA);

    assert_eq!(bxcan_frame, bxcan_frame_expected);
}

#[test]
fn from_usart_frame_test() {
    let usart_frame = vec!(
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
    );

    let ross_frame = RossFrame::from_usart_frame(usart_frame).unwrap();

    assert_eq!(ross_frame, ROSS_FRAME);
}

#[test]
fn to_usart_frame_test() {
    let usart_frame = ROSS_FRAME.to_usart_frame();
    let usart_frame_expected = vec!(
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
    );

    assert_eq!(usart_frame, usart_frame_expected);
}
