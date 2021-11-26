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

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != INTERNAL_SYSTEM_TICK_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;

        Ok(SystemTickEvent { receiver_address })
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
            ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        ];
    
        let event = SystemTickEvent::try_from_packet(&packet).unwrap();
    
        assert_eq!(event.receiver_address, 0xabab);
    }
    
    #[test]
    fn to_packet_test() {
        let event = SystemTickEvent {
            receiver_address: 0xabab,
        };
    
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        ];
    
        assert_eq!(event.to_packet(), packet);
    }    
}
