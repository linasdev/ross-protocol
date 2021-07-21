use alloc::vec;
use core::convert::TryInto;

use crate::ross_convert_packet::{RossConvertPacket, RossConvertPacketError};
use crate::ross_event::ross_event_packet::RossEventPacketError;
use crate::ross_event::ross_event_code::*;
use crate::ross_packet::RossPacket;

#[derive(Debug, PartialEq)]
pub struct RossConfiguratorHelloEvent {
}

impl RossConvertPacket<RossConfiguratorHelloEvent> for RossConfiguratorHelloEvent {
    fn try_from_packet(packet: RossPacket) -> Result<Self, RossConvertPacketError> {
        if packet.data.len() != 2 {
            return Err(RossConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(RossConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != ROSS_CONFIGURATOR_HELLO_EVENT_CODE
        {
            return Err(RossConvertPacketError::EventPacket(
                RossEventPacketError::WrongEventType,
            ));
        }

        Ok(RossConfiguratorHelloEvent {})
    }

    fn to_packet(&self) -> RossPacket {
        let mut data = vec!();

        for byte in u16::to_be_bytes(ROSS_CONFIGURATOR_HELLO_EVENT_CODE).iter() {
            data.push(*byte);
        }

        RossPacket {
            is_error: false,
            device_address: 0x0000,
            data,
        }
    }
}
