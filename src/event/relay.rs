use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RelayValue {
    Single(bool),
    DoubleExclusive(RelayDoubleExclusiveValue),
}

impl RelayValue {
    pub(crate) fn serialize(self) -> Vec<u8> {
        match self {
            Self::Single(value) => vec![if value { 0x00 } else { 0x01 }],
            Self::DoubleExclusive(RelayDoubleExclusiveValue::FirstChannelOn) => vec![0x02],
            Self::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn) => vec![0x03],
            Self::DoubleExclusive(RelayDoubleExclusiveValue::NoChannelOn) => vec![0x04],
        }
    }

    pub(crate) fn deserialize(data: &[u8]) -> Result<Self, ConvertPacketError> {
        if data.len() != 1 {
            return Err(ConvertPacketError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Self::Single(true)),
            0x01 => Ok(Self::Single(false)),
            0x02 => Ok(Self::DoubleExclusive(
                RelayDoubleExclusiveValue::FirstChannelOn,
            )),
            0x03 => Ok(Self::DoubleExclusive(
                RelayDoubleExclusiveValue::SecondChannelOn,
            )),
            0x04 => Ok(Self::DoubleExclusive(
                RelayDoubleExclusiveValue::NoChannelOn,
            )),
            _ => Err(ConvertPacketError::UnknownEnumVariant),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RelayDoubleExclusiveValue {
    FirstChannelOn,
    SecondChannelOn,
    NoChannelOn,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RelaySetValueEvent {
    pub relay_address: u16,
    pub transmitter_address: u16,
    pub index: u8,
    pub value: RelayValue,
}

impl ConvertPacket<RelaySetValueEvent> for RelaySetValueEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 6 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != RELAY_SET_VALUE_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let relay_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let index = packet.data[4];
        let value = RelayValue::deserialize(&packet.data[5..])?;

        Ok(Self {
            relay_address,
            transmitter_address,
            index,
            value,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(RELAY_SET_VALUE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        data.push(self.index);
        data.append(&mut self.value.serialize());

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
            ((RELAY_SET_VALUE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_VALUE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // transmitter address
            0x23,                                             // transmitter address
            0x45,                                             // index
            0x03,                                             // value
        ];

        let event = RelaySetValueEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.relay_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
        assert_eq!(event.index, 0x45);
        assert_eq!(
            event.value,
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn)
        );
    }

    #[test]
    fn set_state_to_packet_test() {
        let event = RelaySetValueEvent {
            relay_address: 0xabab,
            transmitter_address: 0x0123,
            index: 0x45,
            value: RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn),
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((RELAY_SET_VALUE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_VALUE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // transmitter address
            0x23,                                             // transmitter address
            0x45,                                             // index
            0x03,                                             // value
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
