use alloc::vec;
use alloc::vec::Vec;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_event::ross_bootloader_event::*;
use crate::ross_event::ross_event_code::*;
use crate::ross_packet::RossPacket;

const EVENT_PACKET: RossPacket = RossPacket {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_bootloader_hello_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // bootloader_address
        0x23,                                                   // bootloader_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];

    let bootloader_hello_event = RossBootloaderHelloEvent::try_from_packet(&packet).unwrap();

    assert_eq!(bootloader_hello_event.programmer_address, 0xabab);
    assert_eq!(bootloader_hello_event.bootloader_address, 0x0123);
    assert_eq!(bootloader_hello_event.firmware_version, 0x01234567);
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_size_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // bootloader_address
        0x23,                                                   // bootloader_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
        0x00,                                                   // extra byte
    ];

    RossBootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // bootloader_address
        0x23,                                                   // bootloader_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];
    packet.is_error = true;

    RossBootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_event_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0xab,                                                   // programmer_address
        0xab,                                                   // programmer_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];

    RossBootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
fn to_packet_bootloader_hello_event_test() {
    let bootloader_hello_event = RossBootloaderHelloEvent {
        programmer_address: 0xabab,
        bootloader_address: 0x0123,
        firmware_version: 0x01234567,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // bootloader_address
        0x23,                                                   // bootloader_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];

    assert_eq!(bootloader_hello_event.to_packet(), packet);
}
