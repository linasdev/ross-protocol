use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub struct BootloaderHelloEvent {
    pub programmer_address: u16,
    pub bootloader_address: u16,
    pub firmware_version: u32,
}

impl ConvertPacket<BootloaderHelloEvent> for BootloaderHelloEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 8 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BOOTLOADER_HELLO_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(
                EventError::WrongEventType,
            ));
        }

        let programmer_address = packet.device_address;
        let bootloader_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let firmware_version = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());

        Ok(BootloaderHelloEvent {
            programmer_address,
            bootloader_address,
            firmware_version,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(BOOTLOADER_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.bootloader_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_version).iter() {
            data.push(*byte);
        }

        Packet {
            is_error: false,
            device_address: self.programmer_address,
            data,
        }
    }
}
