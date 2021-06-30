use alloc::vec;
use alloc::vec::Vec;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_event::ross_bootloader_event::*;
use crate::ross_packet::RossPacket;

const EVENT_PACKET: RossPacket = RossPacket {
    is_error: false,
    frame_count: 1,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_bootloader_hello_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x00, // event code
        0x01, // programmer_address
        0x23, // programmer_address
        0x01, // firmware_version
        0x23, // firmware_version
        0x45, // firmware_version
        0x67, // firmware_version
    ];

    let bootloader_hello_event = RossBootloaderHelloEvent::try_from_packet(packet).unwrap();

    assert_eq!(bootloader_hello_event.device_address, 0xabab);
    assert_eq!(bootloader_hello_event.programmer_address, 0x0123);
    assert_eq!(bootloader_hello_event.firmware_version, 0x01234567);
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_size_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x00, // event code
        0x01, // programmer_address
        0x23, // programmer_address
        0x01, // firmware_version
        0x23, // firmware_version
        0x45, // firmware_version
        0x67, // firmware_version
        0x00, // extra byte
    ];

    RossBootloaderHelloEvent::try_from_packet(packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x00, // event code
        0x01, // programmer_address
        0x23, // programmer_address
        0x01, // firmware_version
        0x23, // firmware_version
        0x45, // firmware_version
        0x67, // firmware_version
    ];
    packet.is_error = true;

    RossBootloaderHelloEvent::try_from_packet(packet).unwrap();
}

#[test]
#[should_panic]
fn try_from_packet_bootloader_hello_event_wrong_event_type_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x01, // event code
        0x01, // programmer_address
        0x23, // programmer_address
    ];
    packet.is_error = true;

    RossBootloaderHelloEvent::try_from_packet(packet).unwrap();
}

#[test]
fn try_from_packet_bootloader_start_upload_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x01, // event code
        0x01, // programmer_address
        0x23, // programmer_address
    ];

    let bootloader_start_upload_event =
        RossBootloaderStartUploadEvent::try_from_packet(packet).unwrap();

    assert_eq!(bootloader_start_upload_event.device_address, 0xabab);
    assert_eq!(bootloader_start_upload_event.programmer_address, 0x0123);
}
