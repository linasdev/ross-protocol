use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GatewayDiscoverEvent {
    pub device_address: u16,
    pub gateway_address: u16,
}

impl ConvertPacket<GatewayDiscoverEvent> for GatewayDiscoverEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 4 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != GATEWAY_DISCOVER_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let device_address = packet.device_address;
        let gateway_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());

        Ok(Self {
            device_address,
            gateway_address,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(GATEWAY_DISCOVER_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.gateway_address).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.device_address,
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
    fn discover_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((GATEWAY_DISCOVER_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((GATEWAY_DISCOVER_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                              // gateway address
            0x00,                                              // gateway address
        ];

        let event = GatewayDiscoverEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.device_address, 0xabab);
        assert_eq!(event.gateway_address, 0x0000);
    }

    #[test]
    fn discover_to_packet_test() {
        let event = GatewayDiscoverEvent {
            device_address: 0xabab,
            gateway_address: 0x0000,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((GATEWAY_DISCOVER_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((GATEWAY_DISCOVER_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                              // gateway address
            0x00,                                              // gateway address
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
