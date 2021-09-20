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
            return Err(ConvertPacketError::Event(
                EventError::WrongEventType,
            ));
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

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BUTTON_RELEASED_EVENT_CODE {
            return Err(ConvertPacketError::Event(
                EventError::WrongEventType,
            ));
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
