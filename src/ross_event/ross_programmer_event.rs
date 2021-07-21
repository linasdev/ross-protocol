use alloc::vec;
use core::convert::TryInto;

use crate::ross_convert_packet::{RossConvertPacket, RossConvertPacketError};
use crate::ross_event::ross_event_code::*;
use crate::ross_event::ross_event_packet::RossEventPacketError;
use crate::ross_packet::RossPacket;

#[derive(Debug, PartialEq)]
pub struct RossProgrammerHelloEvent {
    pub programmer_address: u16,
    pub firmware_version: u32,
}

impl RossConvertPacket<RossProgrammerHelloEvent> for RossProgrammerHelloEvent {
    fn try_from_packet(packet: &RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.data.len() != 6 {
            return Err(RossConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_PROGRAMMER_HELLO_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        let programmer_address = packet.device_address;
        let firmware_version = u32::from_be_bytes(packet.data[2..=5].try_into().unwrap());

        Ok(RossProgrammerHelloEvent {
            programmer_address,
            firmware_version,
        })
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec![];

        for byte in u16::to_be_bytes(ROSS_PROGRAMMER_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_version).iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: self.programmer_address,
            data,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RossProgrammerStartUploadEvent {
    pub programmer_address: u16,
    pub device_address: u16,
    pub new_firmware_version: u32,
    pub firmware_size: u32,
}

impl RossConvertPacket<RossProgrammerStartUploadEvent> for RossProgrammerStartUploadEvent {
    fn try_from_packet(packet: &RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.data.len() != 12 {
            return Err(RossConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        let programmer_address = packet.device_address;
        let device_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let new_firmware_version = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());
        let firmware_size = u32::from_be_bytes(packet.data[8..=11].try_into().unwrap());

        Ok(RossProgrammerStartUploadEvent {
            programmer_address,
            device_address,
            new_firmware_version,
            firmware_size,
        })
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec![];

        for byte in u16::to_be_bytes(ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.device_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.new_firmware_version).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_size).iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: self.programmer_address,
            data,
        }
    }
}
