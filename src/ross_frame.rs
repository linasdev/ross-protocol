use bxcan::{Data, ExtendedId, Frame, Id};

/// Frame id for packets with more than one frame
#[derive(Debug, PartialEq)]
pub enum RossFrameId {
    /// Last frame id inside current packet (12 bits)
    LastFrameId(u16),
    /// Current frame id (12 bits)
    CurrentFrameId(u16),
}

#[derive(Debug)]
pub enum RossFrameError {
    /// Received a standard frame instead of an extended one
    FrameIsStandard,
    /// Received a remote frame instead of a data one
    FrameIsRemote,
    // Part of the frame id is missing
    FrameIdMissing,
}

/// Ross compatible representation of a CAN frame
#[derive(Debug, PartialEq)]
pub struct RossFrame {
    /// If this bit is low, the frame is considered to be an error frame
    pub not_error_flag: bool,
    /// If this bit is high, the frame is considered to be the first frame of a packet
    pub start_frame_flag: bool,
    /// If this bit is high, the frame is considered to be only a part of a packet
    pub multi_frame_flag: bool,
    /// Either the last or the current frame id inside current packet, depending on `start_frame_flag`
    pub frame_id: RossFrameId,
    /// Transmitting device's address
    pub device_address: u16,
    /// Length of frame data
    pub data_len: u8,
    /// Frame data
    pub data: [u8; 8],
}

impl RossFrame {
    /// Converts from a bxcan frame to a ross frame
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
    pub fn from_bxcan_frame(frame: Frame) -> Result<Self, RossFrameError> {
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
                        return Err(RossFrameError::FrameIdMissing);
                    }

                    let frame_id = if start_frame_flag {
                        RossFrameId::LastFrameId((frame_id_nibble << 8) | data[0] as u16)
                    } else {
                        RossFrameId::CurrentFrameId((frame_id_nibble << 8) | data[0] as u16)
                    };

                    Ok(RossFrame {
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
                    let frame_id = RossFrameId::LastFrameId(0x00);

                    Ok(RossFrame {
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
                Err(RossFrameError::FrameIsRemote)
            }
        } else {
            Err(RossFrameError::FrameIsStandard)
        }
    }

    pub fn to_bxcan_frame(&self) -> Frame {
        let mut id = 0x00;
        id |= (self.not_error_flag as u32) << 28;
        id |= (self.start_frame_flag as u32) << 27;
        id |= (self.multi_frame_flag as u32) << 26;
        match self.frame_id {
            RossFrameId::LastFrameId(frame_id) => id |= ((frame_id & 0x0fff) as u32) << 16,
            RossFrameId::CurrentFrameId(frame_id) => id |= ((frame_id & 0x0fff) as u32) << 16,
        }
        id |= (self.device_address & 0xffff) as u32;

        Frame::new_data(
            ExtendedId::new(id).unwrap(),
            Data::new(&self.data[0..self.data_len as usize]).unwrap(),
        )
    }
}
