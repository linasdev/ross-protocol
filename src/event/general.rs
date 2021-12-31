use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::bcm::BcmValue;
use crate::event::relay::RelayValue;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AckEvent {
    pub receiver_address: u16,
    pub transmitter_address: u16,
}

impl ConvertPacket<AckEvent> for AckEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 4 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != ACK_EVENT_CODE {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());

        Ok(AckEvent {
            receiver_address,
            transmitter_address,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(ACK_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DataEvent {
    pub receiver_address: u16,
    pub transmitter_address: u16,
    pub data_len: u16,
    pub data: Vec<u8>,
}

impl ConvertPacket<DataEvent> for DataEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != DATA_EVENT_CODE {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let data_len = u16::from_be_bytes(packet.data[4..=5].try_into().unwrap());

        if packet.data.len() != data_len as usize + 6 {
            return Err(ConvertPacketError::WrongSize);
        }

        let mut data = vec![0; data_len as usize];

        for i in 0..data_len as usize {
            data[i] = packet.data[i + 6];
        }

        Ok(DataEvent {
            receiver_address,
            transmitter_address,
            data_len,
            data,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(DATA_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.data_len).iter() {
            data.push(*byte);
        }

        for byte in self.data.iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PeripheralValue {
    Bcm(BcmValue),
    Relay(RelayValue),
}

impl PeripheralValue {
    fn serialize(self) -> Vec<u8> {
        match self {
            Self::Bcm(value) => {
                let mut data = vec![0x00];
                data.append(&mut value.serialize());
                data
            },
            Self::Relay(value) => {
                let mut data = vec![0x01];
                data.append(&mut value.serialize());
                data
            },
        }
    }

    fn deserialize(data: &[u8]) -> Result<Self, ConvertPacketError> {
        if data.len() < 2 {
            return Err(ConvertPacketError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Self::Bcm(BcmValue::deserialize(&data[1..])?)),
            0x01 => Ok(Self::Relay(RelayValue::deserialize(&data[1..])?)),
            _ => Err(ConvertPacketError::UnknownEnumVariant),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PeripheralDataEvent {
    pub receiver_address: u16,
    pub peripheral_address: u16,
    pub peripheral_index: u8,
    pub peripheral_value: PeripheralValue,
}

impl ConvertPacket<PeripheralDataEvent> for PeripheralDataEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() < 7 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != PERIPHERAL_DATA_EVENT_CODE {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let peripheral_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let peripheral_index = packet.data[4];
        let peripheral_value = PeripheralValue::deserialize(&packet.data[5..])?;

        Ok(Self {
            receiver_address,
            peripheral_address,
            peripheral_index,
            peripheral_value,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PERIPHERAL_DATA_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.peripheral_address).iter() {
            data.push(*byte);
        }

        data.push(self.peripheral_index);
        data.append(&mut self.peripheral_value.serialize());

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
    fn ack_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                 // transmitter address
            0x23,                                 // transmitter address
        ];

        let event = AckEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
    }

    #[test]
    fn ack_to_packet_test() {
        let event = AckEvent {
            receiver_address: 0xabab,
            transmitter_address: 0x0123,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                 // transmitter address
            0x23,                                 // transmitter address
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn data_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                  // transmitter address
            0x23,                                  // transmitter address
            0x00,                                  // data len
            0x05,                                  // data len
            0x00,                                  // data
            0x01,                                  // data
            0x02,                                  // data
            0x03,                                  // data
            0x04,                                  // data
        ];

        let event = DataEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0123);
        assert_eq!(event.data_len, 0x0005);
        assert_eq!(event.data, vec!(0x00, 0x01, 0x02, 0x03, 0x04));
    }

    #[test]
    fn data_to_packet_test() {
        let event = DataEvent {
            receiver_address: 0xabab,
            transmitter_address: 0x0123,
            data_len: 0x0005,
            data: vec![0x00, 0x01, 0x02, 0x03, 0x04],
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                  // transmitter address
            0x23,                                  // transmitter address
            0x00,                                  // data len
            0x05,                                  // data len
            0x00,                                  // data
            0x01,                                  // data
            0x02,                                  // data
            0x03,                                  // data
            0x04,                                  // data
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn peripheral_data_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PERIPHERAL_DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PERIPHERAL_DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                  // peripheral address
            0x23,                                  // peripheral address
            0x45,                                  // peripheral index
            0x00,                                  // peripheral value
            0x00,                                  // peripheral value
            0xff,                                  // peripheral value
        ];

        let event = PeripheralDataEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.peripheral_address, 0x0123);
        assert_eq!(event.peripheral_index, 0x45);
        assert_eq!(event.peripheral_value, PeripheralValue::Bcm(BcmValue::Single(0xff)));
    }

    #[test]
    fn peripheral_data_to_packet_test() {
        let event = PeripheralDataEvent {
            receiver_address: 0xabab,
            peripheral_address: 0x0123,
            peripheral_index: 0x45,
            peripheral_value: PeripheralValue::Bcm(BcmValue::Single(0xff)),
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PERIPHERAL_DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PERIPHERAL_DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                  // peripheral address
            0x23,                                  // peripheral address
            0x45,                                  // peripheral index
            0x00,                                  // peripheral value
            0x00,                                  // peripheral value
            0xff,                                  // peripheral value
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
