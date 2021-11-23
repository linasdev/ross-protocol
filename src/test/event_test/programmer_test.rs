use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::event_code::*;
use crate::event::programmer::*;
use crate::packet::Packet;
use crate::protocol::BROADCAST_ADDRESS;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_programmer_hello_event_test() {
    let mut packet = EVENT_PACKET;

    packet.data = vec![
        ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // programmer_address
        0x23,                                              // programmer_address
    ];

    let event = ProgrammerHelloEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.programmer_address, 0x0123);
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_size_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // programmer_address
        0x34,                                              // programmer_address
        0x00,                                              // extra byte
    ];

    ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // programmer_address
        0x34,                                              // programmer_address
    ];
    packet.is_error = true;

    ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_programmer_hello_event_wrong_event_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BOOTLOADER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BOOTLOADER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // bootloader_address
        0x23,                                              // bootloader_address
    ];

    ProgrammerHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
fn to_packet_programmer_hello_event_test() {
    let event = ProgrammerHelloEvent {
        programmer_address: 0x0123,
    };

    let mut packet = EVENT_PACKET;
    packet.device_address = BROADCAST_ADDRESS;
    packet.data = vec![
        ((PROGRAMMER_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                              // programmer_address
        0x23,                                              // programmer_address
    ];

    assert_eq!(event.to_packet(), packet);
}

#[test]
fn try_from_packet_programmer_start_upload_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_START_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_START_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                     // programmer_address
        0x23,                                                     // programmer_address
        0x01,                                                     // firmware_size
        0x23,                                                     // firmware_size
        0x45,                                                     // firmware_size
        0x67,                                                     // firmware_size
    ];

    let event = ProgrammerStartUploadEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.receiver_address, 0xabab);
    assert_eq!(event.programmer_address, 0x0123);
    assert_eq!(event.firmware_size, 0x01234567);
}

#[test]
fn to_packet_programmer_start_upload_event_test() {
    let event = ProgrammerStartUploadEvent {
        receiver_address: 0xabab,
        programmer_address: 0x0123,
        firmware_size: 0x01234567,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_START_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_START_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                     // programmer_address
        0x23,                                                     // programmer_address
        0x01,                                                     // firmware_size
        0x23,                                                     // firmware_size
        0x45,                                                     // firmware_size
        0x67,                                                     // firmware_size
    ];

    assert_eq!(event.to_packet(), packet);
}

#[test]
fn try_from_packet_programmer_start_config_upload_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_START_CONFIG_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_START_CONFIG_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                            // programmer_address
        0x23,                                                            // programmer_address
        0x01,                                                            // data_len
        0x23,                                                            // data_len
        0x45,                                                            // data_len
        0x67,                                                            // data_len
    ];

    let event = ProgrammerStartConfigUploadEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.receiver_address, 0xabab);
    assert_eq!(event.programmer_address, 0x0123);
    assert_eq!(event.data_len, 0x01234567);
}

#[test]
fn to_packet_programmer_start_config_upload_event_test() {
    let event = ProgrammerStartConfigUploadEvent {
        receiver_address: 0xabab,
        programmer_address: 0x0123,
        data_len: 0x01234567,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((PROGRAMMER_START_CONFIG_UPLOAD_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((PROGRAMMER_START_CONFIG_UPLOAD_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                            // programmer_address
        0x23,                                                            // programmer_address
        0x01,                                                            // data_len
        0x23,                                                            // data_len
        0x45,                                                            // data_len
        0x67,                                                            // data_len
    ];

    assert_eq!(event.to_packet(), packet);
}
