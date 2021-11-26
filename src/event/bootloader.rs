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
}

impl ConvertPacket<BootloaderHelloEvent> for BootloaderHelloEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 4 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()) != BOOTLOADER_HELLO_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let programmer_address = packet.device_address;
        let bootloader_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());

        Ok(BootloaderHelloEvent {
            programmer_address,
            bootloader_address,
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

        Packet {
            is_error: false,
            device_address: self.programmer_address,
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
            ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // bootloader address
            0x23, // bootloader address
        ];
    
        let event = BootloaderHelloEvent::try_from_packet(&packet).unwrap();
    
        assert_eq!(event.programmer_address, 0xabab);
        assert_eq!(event.bootloader_address, 0x0123);
    }
    
    #[test]
    #[should_panic]
    fn try_from_packet_wrong_size_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // bootloader address
            0x23, // bootloader address
            0x00, // extra byte
        ];
    
        BootloaderHelloEvent::try_from_packet(&packet).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn try_from_packet_wrong_type_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // bootloader address
            0x23, // bootloader address
        ];
        packet.is_error = true;
    
        BootloaderHelloEvent::try_from_packet(&packet).unwrap();
    }
    
    #[test]
    #[should_panic]
    fn try_from_packet_wrong_event_type_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0xab, // programmer address
            0xab, // programmer address
        ];
    
        BootloaderHelloEvent::try_from_packet(&packet).unwrap();
    }
    
    #[test]
    fn to_packet_test() {
        let event = BootloaderHelloEvent {
            programmer_address: 0xabab,
            bootloader_address: 0x0123,
        };
    
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // bootloader address
            0x23, // bootloader address
        ];
    
        assert_eq!(event.to_packet(), packet);
    }    
}
