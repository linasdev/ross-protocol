use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct BcmChangeBrightnessEvent {
    pub bcm_address: u16,
    pub transmitter_address: u16,
    pub channel: u8,
    pub brightness: u8,
}

impl ConvertPacket<BcmChangeBrightnessEvent> for BcmChangeBrightnessEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 6 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != BCM_CHANGE_BRIGHTNESS_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let bcm_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let channel = packet.data[4];
        let brightness = packet.data[5];

        Ok(BcmChangeBrightnessEvent {
            bcm_address,
            transmitter_address,
            channel,
            brightness,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BCM_CHANGE_BRIGHTNESS_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    const EVENT_PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: vec![],
    };
    
    #[test]
    fn try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // transmitter address
            0x23, // transmitter address
            0x45, // channel
            0x67, // brightness
        ];
    
        let event = BcmChangeBrightnessEvent::try_from_packet(&packet).unwrap();
    
        assert_eq!(event.bcm_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
        assert_eq!(event.channel, 0x45);
        assert_eq!(event.brightness, 0x67);
    }
    
    #[test]
    fn to_packet_test() {
        let event = BcmChangeBrightnessEvent {
            bcm_address: 0xabab,
            transmitter_address: 0x0123,
            channel: 0x45,
            brightness: 0x67,
        };
    
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // transmitter address
            0x23, // transmitter address
            0x45, // channel
            0x67, // brightness
        ];
    
        assert_eq!(event.to_packet(), packet);
    }
}
