use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct ButtonPressedEvent {
    pub receiver_address: u16,
    pub button_address: u16,
    pub index: u8,
}

impl ConvertPacket<ButtonPressedEvent> for ButtonPressedEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 5 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BUTTON_PRESSED_EVENT_CODE {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let button_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];

        Ok(ButtonPressedEvent {
            receiver_address,
            button_address,
            index,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BUTTON_PRESSED_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.button_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ButtonReleasedEvent {
    pub receiver_address: u16,
    pub button_address: u16,
    pub index: u8,
}

impl ConvertPacket<ButtonReleasedEvent> for ButtonReleasedEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 5 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BUTTON_RELEASED_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let button_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];

        Ok(ButtonReleasedEvent {
            receiver_address,
            button_address,
            index,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BUTTON_RELEASED_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.button_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        Packet {
            is_error: false,
            device_address: self.receiver_address,
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
    fn pressed_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                            // button address
            0x23,                                            // button address
            0x45,                                            // index
        ];

        let event = ButtonPressedEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.button_address, 0x0123);
        assert_eq!(event.index, 0x45);
    }

    #[test]
    fn pressed_to_packet_test() {
        let event = ButtonPressedEvent {
            receiver_address: 0xabab,
            button_address: 0x0123,
            index: 0x45,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                            // button address
            0x23,                                            // button address
            0x45,                                            // index
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn released_try_from_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // button address
            0x23,                                             // button address
            0x45,                                             // index
        ];

        let event = ButtonReleasedEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.button_address, 0x0123);
        assert_eq!(event.index, 0x45);
    }

    #[test]
    fn released_to_packet_test() {
        let event = ButtonReleasedEvent {
            receiver_address: 0xabab,
            button_address: 0x0123,
            index: 0x45,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // button address
            0x23,                                             // button address
            0x45,                                             // index
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
