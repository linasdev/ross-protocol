use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;
use crate::protocol::BROADCAST_ADDRESS;

#[derive(Debug, PartialEq)]
pub struct ProgrammerHelloEvent {
    pub programmer_address: u16,
    pub firmware_version: u32,
}

impl ConvertPacket<ProgrammerHelloEvent> for ProgrammerHelloEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 8 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != PROGRAMMER_HELLO_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let firmware_version = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());

        Ok(ProgrammerHelloEvent {
            programmer_address,
            firmware_version,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_version).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: BROADCAST_ADDRESS,
            data,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ProgrammerStartUploadEvent {
    pub receiver_address: u16,
    pub programmer_address: u16,
    pub new_firmware_version: u32,
    pub firmware_size: u32,
}

impl ConvertPacket<ProgrammerStartUploadEvent> for ProgrammerStartUploadEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 12 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != PROGRAMMER_START_UPLOAD_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let new_firmware_version = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());
        let firmware_size = u32::from_be_bytes(packet.data[8..=11].try_into().unwrap());

        Ok(ProgrammerStartUploadEvent {
            receiver_address,
            programmer_address,
            new_firmware_version,
            firmware_size,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_START_UPLOAD_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.new_firmware_version).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_size).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}
