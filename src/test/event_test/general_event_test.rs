use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::event_code::*;
use crate::event::general_event::*;
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
        ((ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                 // transmitter_address
        0x23,                                 // transmitter_address
    ];

    let event = AckEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.receiver_address, 0xabab);
    assert_eq!(event.transmitter_address, 0x0123);
}

#[test]
fn to_packet_ack_event_test() {
    let event = AckEvent {
        receiver_address: 0xabab,
        transmitter_address: 0x0123,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((ACK_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((ACK_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                 // transmitter_address
        0x23,                                 // transmitter_address
    ];

    assert_eq!(event.to_packet(), packet);
}

#[test]
fn try_from_packet_data_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                  // transmitter_address
        0x23,                                  // transmitter_address
        0x00,                                  // data_len
        0x05,                                  // data_len
        0x00,                                  // data
        0x01,                                  // data
        0x02,                                  // data
        0x03,                                  // data
        0x04,                                  // data
    ];

    let event = DataEvent::try_from_packet(&packet).unwrap();

    assert_eq!(event.receiver_address, 0xabab);
    assert_eq!(event.transmitter_address, 0x0123);
    assert_eq!(event.data_len, 0x0005);
    assert_eq!(event.data, vec!(0x00, 0x01, 0x02, 0x03, 0x04));
}

#[test]
fn to_packet_data_event_test() {
    let event = DataEvent {
        receiver_address: 0xabab,
        transmitter_address: 0x0123,
        data_len: 0x0005,
        data: vec![0x00, 0x01, 0x02, 0x03, 0x04],
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((DATA_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((DATA_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                  // transmitter_address
        0x23,                                  // transmitter_address
        0x00,                                  // data_len
        0x05,                                  // data_len
        0x00,                                  // data
        0x01,                                  // data
        0x02,                                  // data
        0x03,                                  // data
        0x04,                                  // data
    ];

    assert_eq!(event.to_packet(), packet);
}
