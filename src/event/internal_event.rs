use alloc::vec;
use core::convert::TryInto;

use crate::protocol::BROADCAST_ADDRESS;
use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::event_packet::EventPacketError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct InternalButtonPressedEvent {
    pub index: u8,
}

impl ConvertPacket<InternalButtonPressedEvent> for InternalButtonPressedEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 3 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != INTERNAL_BUTTON_PRESSED_EVENT_CODE {
            return Err(ConvertPacketError::EventPacket(
                EventPacketError::WrongEventType,
            ));
        }

        let index = packet.data[2];

        Ok(InternalButtonPressedEvent {
            index,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(INTERNAL_BUTTON_PRESSED_EVENT_CODE).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        Packet {
            is_error: false,
            device_address: BROADCAST_ADDRESS,
            data,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InternalButtonReleasedEvent {
    pub index: u8,
}

impl ConvertPacket<InternalButtonReleasedEvent> for InternalButtonReleasedEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 3 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != INTERNAL_BUTTON_RELEASED_EVENT_CODE {
            return Err(ConvertPacketError::EventPacket(
                EventPacketError::WrongEventType,
            ));
        }

        let index = packet.data[2];

        Ok(InternalButtonReleasedEvent {
            index,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(INTERNAL_BUTTON_RELEASED_EVENT_CODE).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        Packet {
            is_error: false,
            device_address: BROADCAST_ADDRESS,
            data,
        }
    }
}