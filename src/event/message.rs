use alloc::vec;
use serde::{Serialize, Deserialize};
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MessageValue {
    U8(u8),
    U16(u16),
    U32(u32),
}

#[derive(Debug, PartialEq)]
pub struct MessageEvent {
    pub receiver_address: u16,
    pub transmitter_address: u16,
    pub code: u16,
    pub value: MessageValue,
}

impl ConvertPacket<MessageEvent> for MessageEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 11 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != MESSAGE_EVENT_CODE {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let transmitter_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let code = u16::from_be_bytes(packet.data[4..=5].try_into().unwrap());
        let value = postcard::from_bytes(&packet.data[6..=10]).map_err(|err| ConvertPacketError::PostcardError(err))?;

        Ok(Self {
            receiver_address,
            transmitter_address,
            code,
            value,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(MESSAGE_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.transmitter_address).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.code).iter() {
            data.push(*byte);
        }

        let mut value_buffer = [0u8; 5];
        postcard::to_slice(&self.value, &mut value_buffer).unwrap();
        for byte in value_buffer.iter() {
            data.push(*byte);
        }

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
    fn try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((MESSAGE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((MESSAGE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                     // transmitter address
            0x00,                                     // transmitter address
            0x01,                                     // code
            0x23,                                     // code
            0x02,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
        ];

        let event = MessageEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.transmitter_address, 0x0000);
        assert_eq!(event.code, 0x0123);
    }

    #[test]
    fn to_packet_test() {
        let event = MessageEvent {
            receiver_address: 0xabab,
            transmitter_address: 0x0000,
            code: 0x0123,
            value: MessageValue::U32(0xffff_ffff),
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((MESSAGE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((MESSAGE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                     // transmitter address
            0x00,                                     // transmitter address
            0x01,                                     // code
            0x23,                                     // code
            0x02,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
