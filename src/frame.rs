use alloc::vec;
use alloc::vec::Vec;
use cobs::{decode, encode, max_encoding_length};

use bxcan::{Data, ExtendedId, Frame as BxFrame, Id};

/// Frame id for packets with more than one frame
#[derive(Debug, PartialEq)]
pub enum FrameId {
    /// Last frame id inside current packet (12 bits)
    LastFrameId(u16),
    /// Current frame id (12 bits)
    CurrentFrameId(u16),
}

#[derive(Debug, PartialEq)]
pub enum FrameError {
    /// Received a standard frame instead of an extended one
    FrameIsStandard,
    /// Received a remote frame instead of a data one
    FrameIsRemote,
    /// Part of the frame id is missing
    FrameIdMissing,
    /// Frame has a different size than expected
    WrongSize,
    // COBS decoding error
    CobsError,
}

/// Ross compatible representation of a CAN frame
#[derive(Debug, PartialEq)]
pub struct Frame {
    /// If this bit is low, the frame is considered to be an error frame
    pub not_error_flag: bool,
    /// If this bit is high, the frame is considered to be the first frame of a packet
    pub start_frame_flag: bool,
    /// If this bit is high, the frame is considered to be only a part of a packet
    pub multi_frame_flag: bool,
    /// Either the last or the current frame id inside current packet, depending on `start_frame_flag`
    pub frame_id: FrameId,
    /// Transmitting device's address
    pub device_address: u16,
    /// Length of frame data
    pub data_len: u8,
    /// Frame data
    pub data: [u8; 8],
}

impl Frame {
    /// Converts a bxcan frame to a ross frame
    ///
    /// This is the extended id structure for a ross frame:
    /// bit 0:          NOT_ERROR_FLAG (if this bit is low, the frame is considered to be an error frame)
    /// bit 1:          START_FRAME_FLAG (if this bit is high, the frame is considered to be the first frame of a packet)
    /// bit 2:          MULTI_FRAME_FLAG (if this bit is high, the frame is considered to be only a part of a packet)
    /// bits 3 - 7:     RESERVED (reserved for future use)
    /// bits 8 - 11:    LAST_FRAME_ID (most significant nibble (0xf00) of the last frame id)
    ///                 FRAME_ID (most significant nibble (0xf00) of the current frame id)
    /// bits 12 - 27    DEVICE_ADDRESS (transmitting device's address)
    ///
    pub fn from_bxcan_frame(frame: BxFrame) -> Result<Self, FrameError> {
        if let Id::Extended(id) = frame.id() {
            let id = id.as_raw();

            let not_error_flag = ((id >> 28) & 0x0001) != 0;
            let start_frame_flag = ((id >> 27) & 0x0001) != 0;
            let multi_frame_flag = ((id >> 26) & 0x0001) != 0;
            let frame_id_nibble = ((id >> 16) & 0x000f) as u16;
            let device_address = ((id >> 0) & 0xffff) as u16;

            if let Some(frame_data) = frame.data() {
                let data_len = frame.dlc();
                let mut data = [0u8; 8];

                for i in 0..(data_len as usize) {
                    data[i] = frame_data[i];
                }

                if multi_frame_flag {
                    if data_len == 0 {
                        return Err(FrameError::FrameIdMissing);
                    }

                    let frame_id = if start_frame_flag {
                        FrameId::LastFrameId((frame_id_nibble << 8) | data[0] as u16)
                    } else {
                        FrameId::CurrentFrameId((frame_id_nibble << 8) | data[0] as u16)
                    };

                    Ok(Frame {
                        not_error_flag,
                        start_frame_flag,
                        multi_frame_flag,
                        frame_id,
                        device_address,
                        data_len,
                        data,
                    })
                } else {
                    let start_frame_flag = true;
                    let frame_id = FrameId::LastFrameId(0x00);

                    Ok(Frame {
                        not_error_flag,
                        start_frame_flag,
                        multi_frame_flag,
                        frame_id,
                        device_address,
                        data_len,
                        data,
                    })
                }
            } else {
                Err(FrameError::FrameIsRemote)
            }
        } else {
            Err(FrameError::FrameIsStandard)
        }
    }

    /// Converts a ross frame to a bxcan frame
    pub fn to_bxcan_frame(&self) -> BxFrame {
        let mut id = 0x00;
        id |= (self.not_error_flag as u32) << 28;
        id |= (self.start_frame_flag as u32) << 27;
        id |= (self.multi_frame_flag as u32) << 26;
        match self.frame_id {
            FrameId::LastFrameId(frame_id) => id |= ((frame_id & 0x0f00) as u32 >> 8) << 16,
            FrameId::CurrentFrameId(frame_id) => id |= ((frame_id & 0x0f00) as u32 >> 8) << 16,
        }
        id |= (self.device_address & 0xffff) as u32;

        BxFrame::new_data(
            ExtendedId::new(id).unwrap(),
            Data::new(&self.data[0..self.data_len as usize]).unwrap(),
        )
    }

    /// Converts a USART frame to a ross frame
    ///
    /// This is the structure for a USART frame:
    /// byte 0:
    ///     bit 0:      NOT_ERROR_FLAG (if this bit is low, the frame is considered to be an error frame)
    ///     bit 1:      START_FRAME_FLAG (if this bit is high, the frame is considered to be the first frame of a packet)
    ///     bit 2:      MULTI_FRAME_FLAG (if this bit is high, the frame is considered to be only a part of a packet)s
    ///     bit 3:      RESERVED (reserved for future use)
    ///     bits 4 - 7: LAST_FRAME_ID (most significant nibble (0xf00) of the last frame id)
    ///                 FRAME_ID (most significant nibble (0xf00) of the current frame id)
    ///
    /// byte 1:         LAST_FRAME_ID (least significant byte (0x0ff) of the last frame id)
    ///                 FRAME_ID (least significant byte (0x0ff) of the current frame id)
    ///
    /// byte 2:         DEVICE_ADDRESS (most significant byte (0xff00) of the device address)
    /// byte 3:         DEVICE_ADDRESS (least significant byte (0x00ff) of the device address)
    ///
    /// byte 4:         DATA_LEN (length of frame data)
    /// bytes 5 - 12:   DATA (frame data)
    pub fn from_usart_frame(encoded: Vec<u8>) -> Result<Self, FrameError> {
        let mut frame = vec![0; encoded.len()];
        match decode(&encoded[..], &mut frame[..]) {
            Ok(n) => frame.truncate(n),
            Err(_) => return Err(FrameError::CobsError),
        }

        if frame.len() < 5 || frame.len() != frame[4] as usize + 5 {
            return Err(FrameError::WrongSize);
        }

        let not_error_flag = ((frame[0] >> 7) & 0x01) != 0;
        let start_frame_flag = ((frame[0] >> 6) & 0x01) != 0;
        let multi_frame_flag = ((frame[0] >> 5) & 0x01) != 0;

        let frame_id = if start_frame_flag {
            FrameId::LastFrameId((((frame[0] & 0x0f) as u16) << 8) | frame[1] as u16)
        } else {
            FrameId::CurrentFrameId((((frame[0] & 0x0f) as u16) << 8) | frame[1] as u16)
        };

        let device_address = ((frame[2] as u16) << 8) | frame[3] as u16;
        let data_len = frame[4];
        let mut data = [0u8; 8];

        for i in 0..data_len as usize {
            data[i] = frame[i + 5];
        }

        Ok(Frame {
            not_error_flag,
            start_frame_flag,
            multi_frame_flag,
            frame_id,
            device_address,
            data_len,
            data,
        })
    }

    /// Converts a ross frame to a USART frame
    pub fn to_usart_frame(&self) -> Vec<u8> {
        let mut frame = vec![0x00u8; self.data_len as usize + 5];

        // byte 0
        frame[0] |= (self.not_error_flag as u8) << 7;
        frame[0] |= (self.start_frame_flag as u8) << 6;
        frame[0] |= (self.multi_frame_flag as u8) << 5;

        match self.frame_id {
            FrameId::LastFrameId(frame_id) => frame[0] |= ((frame_id & 0x0f00) >> 8) as u8,
            FrameId::CurrentFrameId(frame_id) => frame[0] |= ((frame_id & 0x0f00) >> 8) as u8,
        }

        // byte 1
        match self.frame_id {
            FrameId::LastFrameId(frame_id) => frame[1] |= (frame_id & 0x00ff) as u8,
            FrameId::CurrentFrameId(frame_id) => frame[1] |= (frame_id & 0x00ff) as u8,
        }

        // bytes 2 & 3
        frame[2] = ((self.device_address & 0xff00) >> 8) as u8;
        frame[3] = (self.device_address & 0x00ff) as u8;

        // byte 4
        frame[4] = self.data_len;

        // bytes 5 - 12
        for i in 0..self.data_len as usize {
            frame[i + 5] = self.data[i];
        }

        let mut encoded = vec![0; max_encoding_length(frame.len())];
        let encoded_len = encode(&frame[..], &mut encoded[..]);
        encoded.truncate(encoded_len);
        return encoded;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let bxcan_frame_expected =
            BxFrame::new_data(ExtendedId::new(FRAME_ID).unwrap(), FRAME_DATA);

        assert_eq!(bxcan_frame, bxcan_frame_expected);
    }

    #[test]
    fn from_usart_frame_test() {
        let usart_frame = vec![
            0x0e, // cobs
            0xa5, // byte 0
            0x55, // frame id
            0x55, // device address
            0x55, // device address
            0x08, // data len
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
        ];

        let ross_frame = Frame::from_usart_frame(usart_frame).unwrap();

        assert_eq!(ross_frame, FRAME);
    }

    #[test]
    fn to_usart_frame_test() {
        let usart_frame = FRAME.to_usart_frame();
        let usart_frame_expected = vec![
            0x0e, // cobs
            0xa5, // byte 0
            0x55, // frame id
            0x55, // device address
            0x55, // device address
            0x08, // data len
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
            0x55, // data
        ];

        assert_eq!(usart_frame, usart_frame_expected);
    }
}
