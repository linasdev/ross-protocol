use bxcan::{Frame, ExtendedId};

use crate::ross_frame::{RossFrameId, RossFrame};

const FRAME_ID: u32 = 0x15555555;
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
