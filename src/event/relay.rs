use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, Eq, PartialEq)]
pub struct RelaySetStateEvent {
    pub relay_address: u16,
    pub transmitter_address: u16,
    pub index: u8,
    pub state: bool,
}

impl ConvertPacket<RelaySetStateEvent> for RelaySetStateEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 6 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != RELAY_SET_STATE_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let relay_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];
        let state = packet.data[5] != 0x00;

        Ok(Self {
            relay_address,
            transmitter_address,
            index,
            state,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(RELAY_SET_STATE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);
        data.push(if self.state { 0x01 } else { 0x00 });

        Packet {
            is_error: false,
            device_address: self.relay_address,
            data,
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct RelayFlipStateEvent {
    pub relay_address: u16,
    pub transmitter_address: u16,
    pub index: u8,
}

impl ConvertPacket<RelayFlipStateEvent> for RelayFlipStateEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 5 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != RELAY_FLIP_STATE_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let relay_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];

        Ok(Self {
            relay_address,
            transmitter_address,
            index,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(RELAY_FLIP_STATE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);

        Packet {
            is_error: false,
            device_address: self.relay_address,
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
    fn set_state_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((RELAY_SET_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // transmitter address
            0x23,                                             // transmitter address
            0x45,                                             // index
            0x01,                                             // state
        ];

        let event = RelaySetStateEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.relay_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
        assert_eq!(event.index, 0x45);
        assert_eq!(event.state, true);
    }

    #[test]
    fn set_state_to_packet_test() {
        let event = RelaySetStateEvent {
            relay_address: 0xabab,
            transmitter_address: 0x0123,
            index: 0x45,
            state: true,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((RELAY_SET_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // transmitter address
            0x23,                                             // transmitter address
            0x45,                                             // index
            0x01,                                             // state
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn flip_state_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((RELAY_FLIP_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_FLIP_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // transmitter address
            0x23,                                              // transmitter address
            0x45,                                              // index
        ];

        let event = RelayFlipStateEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.relay_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
        assert_eq!(event.index, 0x45);
    }

    #[test]
    fn flip_state_to_packet_test() {
        let event = RelayFlipStateEvent {
            relay_address: 0xabab,
            transmitter_address: 0x0123,
            index: 0x45,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((RELAY_FLIP_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_FLIP_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // transmitter address
            0x23,                                              // transmitter address
            0x45,                                              // index
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
