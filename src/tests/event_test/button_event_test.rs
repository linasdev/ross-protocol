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
fn try_from_packet_internal_button_pressed_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                            // index
    ];

    let internal_button_pressed_event = ButtonPressedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(internal_button_pressed_event.device_address, 0xabab);
    assert_eq!(internal_button_pressed_event.index, 0x01);
}

#[test]
fn to_packet_internal_button_pressed_event_test() {
    let internal_button_pressed_event = ButtonPressedEvent {
        device_address: 0xabab,
        index: 0x01,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                            // index
    ];

    assert_eq!(internal_button_pressed_event.to_packet(), packet);
}

#[test]
fn try_from_packet_internal_button_released_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                             // index
    ];

    let internal_button_released_event = ButtonReleasedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(internal_button_released_event.device_address, 0xabab);
    assert_eq!(internal_button_released_event.index, 0x01);
}

#[test]
fn to_packet_internal_button_released_event_test() {
    let internal_button_released_event = ButtonReleasedEvent {
        device_address: 0xabab,
        index: 0x01,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                             // index
    ];

    assert_eq!(internal_button_released_event.to_packet(), packet);
}
