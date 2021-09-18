use crate::error::error_packet::ErrorPacket;
use crate::event::event_packet::EventPacket;

#[derive(Debug, PartialEq)]
pub enum PacketType {
    ErrorPacket(ErrorPacket),
    EventPacket(EventPacket),
}
