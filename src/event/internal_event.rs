use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct SystemTickEvent {
    pub receiver_address: u16,
}

impl ConvertPacket<SystemTickEvent> for SystemTickEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 2 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != INTERNAL_SYSTEM_TICK_EVENT_CODE {
            return Err(ConvertPacketError::Event(
                EventError::WrongEventType,
            ));
        }

        let receiver_address = packet.device_address;

        Ok(SystemTickEvent {
            receiver_address,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(INTERNAL_SYSTEM_TICK_EVENT_CODE).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}
