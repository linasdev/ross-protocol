use bxcan::{Frame, Id, ExtendedId};

use crate::{RossFrameId, RossFrame};

const BXCAN_EXTENDED_IDENTIFIER: Id =
    Id::Extended(unsafe { ExtendedId::new_unchecked(0x0aaaaaaa) });

const BXCAN_DATA: [u8; 8] = [0xaa; 8];

#[test]
fn from_bxcan_frame_test() {
    let bxcan_frame = Frame::new_data(BXCAN_EXTENDED_IDENTIFIER, BXCAN_DATA);

    let frame = RossFrame::from_bxcan_frame(bxcan_frame).unwrap();

    assert_eq!(frame.not_error_flag, true);
    assert_eq!(frame.start_frame_flag, false);
    assert_eq!(frame.multi_frame_flag, true);
    assert_eq!(frame.frame_id, RossFrameId::CurrentFrameId(0x0aaa));
    assert_eq!(frame.device_address, 0xaaaa);
    assert_eq!(frame.data_len, 8);
    assert_eq!(frame.data, BXCAN_DATA);
}
