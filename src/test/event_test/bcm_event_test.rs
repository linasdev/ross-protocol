use alloc::vec;
use alloc::vec::Vec;

use crate::convert_packet::ConvertPacket;
use crate::event::bcm_event::*;
use crate::event::event_code::*;
use crate::packet::Packet;

const EVENT_PACKET: Packet = Packet {
    is_error: false,
    device_address: 0xabab,
    data: Vec::new(),
};

#[test]
fn try_from_packet_bcm_change_brightness_event_test() {
    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // transmitter_address
        0x23,                                                   // transmitter_address
        0x45,                                                   // channel
        0x67,                                                   // brightness
    ];

    let bcm_change_brightness_event = BcmChangeBrightnessEvent::try_from_packet(&packet).unwrap();

    assert_eq!(bcm_change_brightness_event.bcm_address, 0xabab);
    assert_eq!(bcm_change_brightness_event.transmitter_address, 0x0123);
    assert_eq!(bcm_change_brightness_event.channel, 0x45);
    assert_eq!(bcm_change_brightness_event.brightness, 0x67);
}

#[test]
fn to_packet_bcm_change_brightness_event_test() {
    let bootloader_hello_event = BcmChangeBrightnessEvent {
        bcm_address: 0xabab,
        transmitter_address: 0x0123,
        channel: 0x45,
        brightness: 0x67,
    };

    let mut packet = EVENT_PACKET;
    packet.data = vec![
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
        ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
        0x01,                                                   // transmitter_address
        0x23,                                                   // transmitter_address
        0x45,                                                   // channel
        0x67,                                                   // brightness
    ];

    assert_eq!(bootloader_hello_event.to_packet(), packet);
}
