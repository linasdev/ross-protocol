use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BcmValue {
    Single(u8),
    Rgb(u8, u8, u8),
    Rgbw(u8, u8, u8, u8),
}

impl BcmValue {
    fn serialize(self) -> Vec<u8> {
        match self {
            Self::Single(value) => vec![0x00, value],
            Self::Rgb(red, green, blue) => vec![0x01, red, green, blue],
            Self::Rgbw(red, green, blue, white) => vec![0x02, red, green, blue, white],
        }
    }

    fn deserialize(data: &[u8]) -> Result<Self, ConvertPacketError> {
        if data.len() < 2 {
            return Err(ConvertPacketError::WrongSize);
        }

        match data[0] {
            0x00 => {
                if data.len() != 2 {
                    return Err(ConvertPacketError::WrongSize);
                }

                Ok(Self::Single(data[1]))
            }
            0x01 => {
                if data.len() != 4 {
                    return Err(ConvertPacketError::WrongSize);
                }

                Ok(Self::Rgb(data[1], data[2], data[3]))
            }
            0x02 => {
                if data.len() != 5 {
                    return Err(ConvertPacketError::WrongSize);
                }

                Ok(Self::Rgbw(data[1], data[2], data[3], data[4]))
            }
            _ => Err(ConvertPacketError::UnknownEnumVariant),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BcmChangeBrightnessEvent {
    pub bcm_address: u16,
    pub transmitter_address: u16,
    pub index: u8,
    pub value: BcmValue,
}

impl ConvertPacket<BcmChangeBrightnessEvent> for BcmChangeBrightnessEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() < 7 {
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
        let index = packet.data[4];
        let value = BcmValue::deserialize(&packet.data[5..])?;

        Ok(BcmChangeBrightnessEvent {
            bcm_address,
            transmitter_address,
            index,
            value,
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

        data.push(self.index);
        data.append(&mut self.value.serialize());

        Packet {
            is_error: false,
            device_address: self.bcm_address,
            data,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct BcmAnimateBrightnessEvent {
    pub bcm_address: u16,
    pub transmitter_address: u16,
    pub index: u8,
    pub duration: u32,
    pub target_value: BcmValue,
}

impl ConvertPacket<BcmAnimateBrightnessEvent> for BcmAnimateBrightnessEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() < 11 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != BCM_ANIMATE_BRIGHTNESS_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let bcm_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];
        let duration = u32::from_be_bytes(packet.data[5..=8].try_into().unwrap());
        let target_value = BcmValue::deserialize(&packet.data[9..])?;

        Ok(Self {
            bcm_address,
            transmitter_address,
            index,
            duration,
            target_value,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BCM_ANIMATE_BRIGHTNESS_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        for byte in u32::to_be_bytes(self.duration).iter() {
            data.push(*byte);
        }

        data.append(&mut self.target_value.serialize());

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
    fn change_brightness_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                   // transmitter address
            0x00,                                                   // transmitter address
            0x01,                                                   // index
            0x01,                                                   // value
            0x23,                                                   // value
            0x45,                                                   // value
            0x67,                                                   // value
        ];

        let event = BcmChangeBrightnessEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.bcm_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0000);
        assert_eq!(event.index, 0x01);
        assert_eq!(event.value, BcmValue::Rgb(0x23, 0x45, 0x67));
    }

    #[test]
    fn change_brightness_to_packet_test() {
        let event = BcmChangeBrightnessEvent {
            bcm_address: 0xabab,
            transmitter_address: 0x0000,
            index: 0x01,
            value: BcmValue::Rgb(0x23, 0x45, 0x67),
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                   // transmitter address
            0x00,                                                   // transmitter address
            0x01,                                                   // index
            0x01,                                                   // value
            0x23,                                                   // value
            0x45,                                                   // value
            0x67,                                                   // value
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn animate_brightness_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                    // transmitter address
            0x00,                                                    // transmitter address
            0x01,                                                    // index
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0x01,                                                    // target value
            0x23,                                                    // target value
            0x45,                                                    // target value
            0x67,                                                    // target value
        ];

        let event = BcmAnimateBrightnessEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.bcm_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0000);
        assert_eq!(event.index, 0x01);
        assert_eq!(event.duration, 0xabab_abab);
        assert_eq!(event.target_value, BcmValue::Rgb(0x23, 0x45, 0x67));
    }

    #[test]
    fn animate_brightness_to_packet_test() {
        let event = BcmAnimateBrightnessEvent {
            bcm_address: 0xabab,
            transmitter_address: 0x0000,
            index: 0x01,
            duration: 0xabab_abab,
            target_value: BcmValue::Rgb(0x23, 0x45, 0x67),
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                    // transmitter address
            0x00,                                                    // transmitter address
            0x01,                                                    // index
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0x01,                                                    // target value
            0x23,                                                    // target value
            0x45,                                                    // target value
            0x67,                                                    // target value
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
