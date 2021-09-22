use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::configurator_event::*;
use crate::event::event_code::*;
use crate::packet::Packet;
use crate::protocol::BROADCAST_ADDRESS;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0x0000,
    data: Vec::new(),
};

#[test]
fn try_from_packet_configurator_hello_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((CONFIGURATOR_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((CONFIGURATOR_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
    ];

    ConfiguratorHelloEvent::try_from_packet(&packet).unwrap();
}

#[test]
fn to_packet_configurator_hello_event_test() {
    let configurator_hello_event = ConfiguratorHelloEvent {};

    let mut packet = EVENT_PACKET;
    packet.device_address = BROADCAST_ADDRESS;
    packet.data = vec![
        ((CONFIGURATOR_HELLO_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((CONFIGURATOR_HELLO_EVENT_CODE >> 0) & 0xff) as u8, // event code
    ];

    assert_eq!(configurator_hello_event.to_packet(), packet);
}
