use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::bootloader::*;
use crate::event::event_code::*;
use crate::packet::Packet;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_bootloader_hello_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // bootloader_address
        0x23,                                              // bootloader_address
    ];

    let event = BootloaderHelloEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.programmer_address, 0xabab);
    assert_eq!(event.bootloader_address, 0x0123);
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_size_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // bootloader_address
        0x23,                                              // bootloader_address
        0x00,                                              // extra byte
    ];

    BootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // bootloader_address
        0x23,                                              // bootloader_address
    ];
    packet.is_error = true;

    BootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_event_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0xab,                                              // programmer_address
        0xab,                                              // programmer_address
    ];

    BootloaderHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
fn to_packet_bootloader_hello_event_test() {
    let event = BootloaderHelloEvent {
        programmer_address: 0xabab,
        bootloader_address: 0x0123,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // bootloader_address
        0x23,                                              // bootloader_address
    ];

    assert_eq!(event.to_packet(), packet);
}
