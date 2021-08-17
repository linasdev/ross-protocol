use alloc::vec;
use alloc::vec::Vec;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_event::ross_event_code::*;
use crate::ross_event::ross_programmer_event::*;
use crate::ross_packet::RossPacket;
use crate::ross_protocol::BROADCAST_ADDRESS;

const EVENT_PACKET: RossPacket = RossPacket {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_programmer_hello_event_test() {
    let mut packet = EVENT_PACKET;

    packet.data = vec![
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // programmer_address
        0x23,                                                   // programmer_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];

    let programmer_hello_event = RossProgrammerHelloEvent::try_from_packet(&packet).unwrap();

    assert_eq!(programmer_hello_event.programmer_address, 0x0123);
    assert_eq!(programmer_hello_event.firmware_version, 0x01234567);
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_size_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // programmer_address
        0x34,                                                   // programmer_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
        0x00,                                                   // extra byte
    ];

    RossProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // programmer_address
        0x34,                                                   // programmer_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];
    packet.is_error = true;

    RossProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_event_type_test() {
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

    RossProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
fn to_packet_programmer_hello_event_test() {
    let programmer_hello_event = RossProgrammerHelloEvent {
        programmer_address: 0x0123,
        firmware_version: 0x01234567,
    };

    let mut packet = EVENT_PACKET;
    packet.device_address = BROADCAST_ADDRESS;
    packet.data = vec![
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // programmer_address
        0x23,                                                   // programmer_address
        0x01,                                                   // firmware_version
        0x23,                                                   // firmware_version
        0x45,                                                   // firmware_version
        0x67,                                                   // firmware_version
    ];

    assert_eq!(programmer_hello_event.to_packet(), packet);
}

#[test]
fn try_from_packet_programmer_start_upload_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                          // programmer_address
        0x23,                                                          // programmer_address
        0x01,                                                          // new_firmware_version
        0x23,                                                          // new_firmware_version
        0x45,                                                          // new_firmware_version
        0x67,                                                          // new_firmware_version
        0x01,                                                          // firmware_size
        0x23,                                                          // firmware_size
        0x45,                                                          // firmware_size
        0x67,                                                          // firmware_size
    ];

    let programmer_start_upload_event =
        RossProgrammerStartUploadEvent::try_from_packet(&packet).unwrap();

    assert_eq!(programmer_start_upload_event.receiver_address, 0xabab);
    assert_eq!(programmer_start_upload_event.programmer_address, 0x0123);
    assert_eq!(
        programmer_start_upload_event.new_firmware_version,
        0x01234567
    );
    assert_eq!(programmer_start_upload_event.firmware_size, 0x01234567);
}

#[test]
fn to_packet_programmer_start_upload_event_test() {
    let programmer_start_upload_event = RossProgrammerStartUploadEvent {
        receiver_address: 0xabab,
        programmer_address: 0x0123,
        new_firmware_version: 0x01234567,
        firmware_size: 0x01234567,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_PROGRAMMER_START_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                          // programmer_address
        0x23,                                                          // programmer_address
        0x01,                                                          // new_firmware_version
        0x23,                                                          // new_firmware_version
        0x45,                                                          // new_firmware_version
        0x67,                                                          // new_firmware_version
        0x01,                                                          // firmware_size
        0x23,                                                          // firmware_size
        0x45,                                                          // firmware_size
        0x67,                                                          // firmware_size
    ];

    assert_eq!(programmer_start_upload_event.to_packet(), packet);
}
