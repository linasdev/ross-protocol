use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::event_packet::EventPacketError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct BcmChangeBrightnessEvent {
    pub bcm_address: u16,
    pub channel: u8,
    pub brightness: u8,
}

impl ConvertPacket<BcmChangeBrightnessEvent> for BcmChangeBrightnessEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 4 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BCM_CHANGE_BRIGHTNESS_EVENT_CODE
        {
            return Err(ConvertPacketError::EventPacket(
                EventPacketError::WrongEventType,
            ));
        }

        let bcm_address = packet.device_address;
        let channel = packet.data[2];
        let brightness = packet.data[3];

        Ok(BcmChangeBrightnessEvent {
            bcm_address,
            channel,
            brightness,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BCM_CHANGE_BRIGHTNESS_EVENT_CODE).iter() {
            data.push(*byte);
        }

        data.push(self.channel);
        data.push(self.brightness);

        Packet {
            is_error: false,
            device_address: self.bcm_address,
            data,
        }
    }
}
