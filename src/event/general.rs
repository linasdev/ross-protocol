use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
