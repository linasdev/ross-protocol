use crate::ross_error::ross_error_packet::RossErrorPacket;
use crate::ross_event::ross_event_packet::RossEventPacket;

#[derive(Debug, PartialEq)]
pub enum RossPacketType {
    ErrorPacket(RossErrorPacket),
    EventPacket(RossEventPacket),
}
