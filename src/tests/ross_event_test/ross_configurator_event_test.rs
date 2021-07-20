use alloc::vec;
use alloc::vec::Vec;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_event::ross_configurator_event::*;
use crate::ross_packet::RossPacket;

const EVENT_PACKET: RossPacket = RossPacket {
    is_error: false,
    device_address: 0x0000,
    data: Vec::new(),
};

#[test]
fn try_from_packet_configurator_hello_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x05, // event code
    ];

    RossConfiguratorHelloEvent::try_from_packet(packet).unwrap();
}

#[test]
fn to_packet_configurator_hello_event_test() {
    let configurator_hello_event = RossConfiguratorHelloEvent {};

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        0x00, // event code
        0x05, // event code
    ];

    assert_eq!(configurator_hello_event.to_packet(), packet);
}
