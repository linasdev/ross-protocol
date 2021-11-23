use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::event_code::*;
use crate::event::internal::*;
use crate::packet::Packet;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_ack_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 0) & 0xff) as u8, // event code
    ];

    let event = SystemTickEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.receiver_address, 0xabab);
}

#[test]
fn to_packet_ack_event_test() {
    let event = SystemTickEvent {
        receiver_address: 0xabab,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((INTERNAL_SYSTEM_TICK_EVENT_CODE >> 0) & 0xff) as u8, // event code
    ];

    assert_eq!(event.to_packet(), packet);
}
