use alloc::vec;
use core::convert::TryInto;

use crate::convert_packet::{ConvertPacket, ConvertPacketError};
use crate::event::event_code::*;
use crate::event::EventError;
use crate::packet::Packet;
use crate::protocol::BROADCAST_ADDRESS;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ProgrammerSetDeviceAddressEvent {
    pub receiver_address: u16,
    pub programmer_address: u16,
    pub new_address: u16,
}

impl ConvertPacket<ProgrammerSetDeviceAddressEvent> for ProgrammerSetDeviceAddressEvent {
    fn try_from_packet(packet: &Packet) -> Result<Self, ConvertPacketError> {
        if packet.data.len() != 6 {
            return Err(ConvertPacketError::WrongSize);
        }

        if packet.is_error {
            return Err(ConvertPacketError::WrongType);
        }

        if u16::from_be_bytes(packet.data[0..=1].try_into().unwrap())
            != PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE
        {
            return Err(ConvertPacketError::Event(EventError::WrongEventType));
        }

        let receiver_address = packet.device_address;
        let programmer_address = u16::from_be_bytes(packet.data[2..=3].try_into().unwrap());
        let new_address = u16::from_be_bytes(packet.data[4..=5].try_into().unwrap());

        Ok(ProgrammerSetDeviceAddressEvent {
            receiver_address,
            programmer_address,
            new_address,
        })
    }

    fn to_packet(&self) -> Packet {
        let mut data = vec![];

        for byte in u16::to_be_bytes(PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.programmer_address).iter() {
            data.push(*byte);
        }

        for byte in u16::to_be_bytes(self.new_address).iter() {
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
    fn hello_try_from_packet_test() {
        let mut packet = EVENT_PACKET;

        packet.data = vec![
            ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // programmer address
            0x23,                                              // programmer address
        ];

        let event = ProgrammerHelloEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.programmer_address, 0x0123);
    }

    #[test]
    #[should_panic]
    fn hello_try_from_packet_wrong_size_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // programmer address
            0x34,                                              // programmer address
            0x00,                                              // extra byte
        ];

        ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
    }

    #[test]
    #[should_panic]
    fn hello_try_from_packet_wrong_type_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // programmer address
            0x34,                                              // programmer address
        ];
        packet.is_error = true;

        ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
    }

    #[test]
    #[should_panic]
    fn hello_try_from_packet_wrong_event_type_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // bootloader address
            0x23,                                              // bootloader address
        ];

        ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
    }

    #[test]
    fn hello_to_packet_test() {
        let event = ProgrammerHelloEvent {
            programmer_address: 0x0123,
        };

        let mut packet = EVENT_PACKET;
        packet.device_address = BROADCAST_ADDRESS;
        packet.data = vec![
            ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                              // programmer address
            0x23,                                              // programmer address
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn start_firmware_upgrade_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // programmer address
            0x23, // programmer address
            0x01, // firmware size
            0x23, // firmware size
            0x45, // firmware size
            0x67, // firmware size
        ];

        let event = ProgrammerStartFirmwareUpgradeEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.programmer_address, 0x0123);
        assert_eq!(event.firmware_size, 0x01234567);
    }

    #[test]
    fn start_firmware_upgrade_to_packet_test() {
        let event = ProgrammerStartFirmwareUpgradeEvent {
            receiver_address: 0xabab,
            programmer_address: 0x0123,
            firmware_size: 0x01234567,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_START_FIRMWARE_UPGRADE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01, // programmer address
            0x23, // programmer address
            0x01, // firmware size
            0x23, // firmware size
            0x45, // firmware size
            0x67, // firmware size
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn start_config_upgrade_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                             // programmer address
            0x23,                                                             // programmer address
            0x01,                                                             // data len
            0x23,                                                             // data len
            0x45,                                                             // data len
            0x67,                                                             // data len
        ];

        let event = ProgrammerStartConfigUpgradeEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.programmer_address, 0x0123);
        assert_eq!(event.config_size, 0x01234567);
    }

    #[test]
    fn start_config_upgrade_to_packet_test() {
        let event = ProgrammerStartConfigUpgradeEvent {
            receiver_address: 0xabab,
            programmer_address: 0x0123,
            config_size: 0x01234567,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_START_CONFIG_UPGRADE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                             // programmer address
            0x23,                                                             // programmer address
            0x01,                                                             // data len
            0x23,                                                             // data len
            0x45,                                                             // data len
            0x67,                                                             // data len
        ];

        assert_eq!(event.to_packet(), packet);
    }

    #[test]
    fn set_device_address_try_from_packet_test() {
        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                           // programmer address
            0x23,                                                           // programmer address
            0x01,                                                           // new address
            0x23,                                                           // new address
        ];

        let event = ProgrammerSetDeviceAddressEvent::try_from_packet(&packet).unwrap();

        assert_eq!(event.receiver_address, 0xabab);
        assert_eq!(event.programmer_address, 0x0123);
        assert_eq!(event.new_address, 0x0123);
    }

    #[test]
    fn set_device_address_to_packet_test() {
        let event = ProgrammerSetDeviceAddressEvent {
            receiver_address: 0xabab,
            programmer_address: 0x0123,
            new_address: 0x0123,
        };

        let mut packet = EVENT_PACKET;
        packet.data = vec![
            ((PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((PROGRAMMER_SET_DEVICE_ADDRESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                           // programmer address
            0x23,                                                           // programmer address
            0x01,                                                           // new address
            0x23,                                                           // new address
        ];

        assert_eq!(event.to_packet(), packet);
    }
}
