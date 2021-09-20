use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::button_event::*;
use crate::event::event_code::*;
use crate::packet::Packet;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_button_pressed_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                            // button_address
        0x23,                                            // button_address
        0x45,                                            // index
    ];

    let button_pressed_event = ButtonPressedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(button_pressed_event.receiver_address, 0xabab);
    assert_eq!(button_pressed_event.button_address, 0x0123);
    assert_eq!(button_pressed_event.index, 0x45);
}

#[test]
fn to_packet_button_pressed_event_test() {
    let button_pressed_event = ButtonPressedEvent {
        receiver_address: 0xabab,
        button_address: 0x0123,
        index: 0x45,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                            // button_address
        0x23,                                            // button_address
        0x45,                                            // index
    ];

    assert_eq!(button_pressed_event.to_packet(), packet);
}

#[test]
fn try_from_packet_button_released_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                             // button_address
        0x23,                                             // button_address
        0x45,                                             // index
    ];

    let button_released_event = ButtonReleasedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(button_released_event.receiver_address, 0xabab);
    assert_eq!(button_released_event.button_address, 0x0123);
    assert_eq!(button_released_event.index, 0x45);
}

#[test]
fn to_packet_button_released_event_test() {
    let button_released_event = ButtonReleasedEvent {
        receiver_address: 0xabab,
        button_address: 0x0123,
        index: 0x45,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                             // button_address
        0x23,                                             // button_address
        0x45,                                             // index
    ];

    assert_eq!(button_released_event.to_packet(), packet);
}
