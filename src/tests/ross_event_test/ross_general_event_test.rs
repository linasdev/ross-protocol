use alloc::vec;
use alloc::vec::Vec;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_event::ross_event_code::*;
use crate::ross_event::ross_general_event::*;
use crate::ross_packet::RossPacket;

const EVENT_PACKET: RossPacket = RossPacket {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_ack_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                      // transmitter_address
        0x23,                                      // transmitter_address
    ];

    let ack_event = RossAckEvent::try_from_packet(&packet).unwrap();

    assert_eq!(ack_event.device_address, 0xabab);
    assert_eq!(ack_event.transmitter_address, 0x0123);
}

#[test]
fn to_packet_ack_event_test() {
    let ack_event = RossAckEvent {
        device_address: 0xabab,
        transmitter_address: 0x0123,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                      // transmitter_address
        0x23,                                      // transmitter_address
    ];

    assert_eq!(ack_event.to_packet(), packet);
}

#[test]
fn try_from_packet_data_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                       // receiver_address
        0x23,                                       // receiver_address
        0x00,                                       // data_len
        0x05,                                       // data_len
        0x00,                                       // data
        0x01,                                       // data
        0x02,                                       // data
        0x03,                                       // data
        0x04,                                       // data
    ];

    let data_event = RossDataEvent::try_from_packet(&packet).unwrap();

    assert_eq!(data_event.transmitter_address, 0xabab);
    assert_eq!(data_event.receiver_address, 0x0123);
    assert_eq!(data_event.data_len, 0x0005);
    assert_eq!(data_event.data, vec!(0x00, 0x01, 0x02, 0x03, 0x04));
}

#[test]
fn to_packet_data_event_test() {
    let data_event = RossDataEvent {
        transmitter_address: 0xabab,
        receiver_address: 0x0123,
        data_len: 0x0005,
        data: vec![0x00, 0x01, 0x02, 0x03, 0x04],
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ROSS_DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ROSS_DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                       // receiver_address
        0x23,                                       // receiver_address
        0x00,                                       // data_len
        0x05,                                       // data_len
        0x00,                                       // data
        0x01,                                       // data
        0x02,                                       // data
        0x03,                                       // data
        0x04,                                       // data
    ];

    assert_eq!(data_event.to_packet(), packet);
}
