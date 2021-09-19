use alloc::vec;
use alloc::vec::Vec;

use crate::protocol::BROADCAST_ADDRESS;
use crate::convert_packet::ConvertPacket;
use crate::event::internal_event::*;
use crate::event::event_code::*;
use crate::packet::Packet;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: BROADCAST_ADDRESS,
    data: Vec::new(),
};

#[test]
fn try_from_packet_internal_button_pressed_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                     // index
    ];

    let internal_button_pressed_event = InternalButtonPressedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(internal_button_pressed_event.index, 0x01);
}

#[test]
fn to_packet_internal_button_pressed_event_test() {
    let internal_button_pressed_event = InternalButtonPressedEvent {
        index: 0x01,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                     // index
    ];

    assert_eq!(internal_button_pressed_event.to_packet(), packet);
}

#[test]
fn try_from_packet_internal_button_released_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                      // index
    ];

    let internal_button_released_event = InternalButtonReleasedEvent::try_from_packet(&packet).unwrap();

    assert_eq!(internal_button_released_event.index, 0x01);
}

#[test]
fn to_packet_internal_button_released_event_test() {
    let internal_button_released_event = InternalButtonReleasedEvent {
        index: 0x01,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                      // index
    ];

    assert_eq!(internal_button_released_event.to_packet(), packet);
}
