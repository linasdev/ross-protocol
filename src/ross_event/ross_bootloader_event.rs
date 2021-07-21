use alloc::vec;
use core::convert::TryInto;

use crate::ross_convert_packet::{RossConvertPacket, RossConvertPacketError};
use crate::ross_event::ross_event_code::*;
use crate::ross_event::ross_event_packet::RossEventPacketError;
use crate::ross_packet::RossPacket;

#[derive(Debug, PartialEq)]
pub struct RossBootloaderHelloEvent {
    pub device_address: u16,
    pub programmer_address: u16,
    pub firmware_version: u32,
}

impl RossConvertPacket<RossBootloaderHelloEvent> for RossBootloaderHelloEvent {
    fn try_from_packet(packet: &RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.data.len() != 8 {
            return Err(RossConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_BOOTLOADER_HELLO_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        let device_address = packet.device_address;
        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let firmware_version = u32::from_be_bytes(packet.data[4..=7].try_into().unwrap());

        Ok(RossBootloaderHelloEvent {
            device_address,
            programmer_address,
            firmware_version,
        })
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec![];

        for byte in u16::to_be_bytes(ROSS_BOOTLOADER_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
            data.push(*byte);
        }

        for byte in u32::to_be_bytes(self.firmware_version).iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: self.device_address,
            data,
        }
    }
}
