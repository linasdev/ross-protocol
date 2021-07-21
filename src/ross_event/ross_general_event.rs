use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use crate::ross_convert_packet::{RossConvertPacket, RossConvertPacketError};
use crate::ross_event::ross_event_packet::RossEventPacketError;
use crate::ross_event::ross_event_code::*;
use crate::ross_packet::RossPacket;

#[derive(Debug, PartialEq)]
pub struct RossAckEvent {
    pub device_address: u16,
    pub transmitter_address: u16,
}

impl RossConvertPacket<RossAckEvent> for RossAckEvent {
    fn try_from_packet(packet: RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.data.len() != 4 {
            return Err(RossConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_ACK_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        let device_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());

        Ok(RossAckEvent {
            device_address,
            transmitter_address,
        })
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec!();

        for byte in u16::to_be_bytes(ROSS_ACK_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: self.device_address,
            data,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RossDataEvent {
    pub device_address: u16,
    pub data_len: u16,
    pub data: Vec<u8>,
}

impl RossConvertPacket<RossDataEvent> for RossDataEvent {
    fn try_from_packet(packet: RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_DATA_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        let device_address = packet.device_address;
        let data_len = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());

        if packet.data.len() != data_len as usize + 4 {
            return Err(RossConvertPacketError::WrongSize);
        }

        let mut data = vec![0; data_len as usize];

        for i in 0..data_len as usize {
            data[i] = packet.data[i + 4];
        }

        Ok(RossDataEvent {
            device_address,
            data_len,
            data
        })
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec!();

        for byte in u16::to_be_bytes(ROSS_DATA_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.data_len).iter() {
            data.push(*byte);
        }

        for byte in self.data.iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: self.device_address,
            data,
        }
    }
}
