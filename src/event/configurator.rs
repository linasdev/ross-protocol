use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;
use crate::protocol::BROADCAST_ADDRESS;

#[derive(Debug, PartialEq)]
pub struct ConfiguratorHelloEvent {}

impl ConvertPacket<ConfiguratorHelloEvent> for ConfiguratorHelloEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 2 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != CONFIGURATOR_HELLO_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        Ok(ConfiguratorHelloEvent {})
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(CONFIGURATOR_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: BROADCAST_ADDRESS,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EVENT_PACKET: Packet = Packet {
        is_error: false,
        device_address: 0x0000,
        data: vec![],
    };
    
    #[test]
    fn try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((CONFIGURATOR_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((CONFIGURATOR_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        ];
    
        ConfiguratorHelloEvent::try_from_packet(&packet).unwrap();
    }
    
    #[test]
    fn to_packet_test() {
        let event = ConfiguratorHelloEvent {};
    
        let mut packet = EVENT_PACKET;
        packet.device_address = BROADCAST_ADDRESS;
        packet.data = vec![
            ((CONFIGURATOR_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((CONFIGURATOR_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        ];
    
        assert_eq!(event.to_packet(), packet);
    }  
}
