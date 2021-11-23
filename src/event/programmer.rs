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
}

impl ConvertPacket<ProgrammerHelloEvent> for ProgrammerHelloEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 4 {
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

        Ok(ProgrammerHelloEvent { programmer_address })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
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
pub struct ProgrammerStartFirmwareUpgradeEvent {
    pub receiver_address: u16,
    pub programmer_address: u16,
    pub firmware_size: u32,
}

impl ConvertPacket<ProgrammerStartFirmwareUpgradeEvent> for ProgrammerStartFirmwareUpgradeEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 8 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let firmware_size = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());

        Ok(ProgrammerStartFirmwareUpgradeEvent {
            receiver_address,
            programmer_address,
            firmware_size,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
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

#[derive(Debug, PartialEq)]
pub struct ProgrammerStartConfigUpgradeEvent {
    pub receiver_address: u16,
    pub programmer_address: u16,
    pub config_size: u32,
}

impl ConvertPacket<ProgrammerStartConfigUpgradeEvent> for ProgrammerStartConfigUpgradeEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 8 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let config_size = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());

        Ok(ProgrammerStartConfigUpgradeEvent {
            receiver_address,
            programmer_address,
            config_size,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.config_size).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.receiver_address,
            data,
        }
    }
}
