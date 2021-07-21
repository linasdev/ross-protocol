use crate::ross_event::ross_event_packet::RossEventPacketError;
use crate::ross_packet::RossPacket;

#[derive(Debug, PartialEq)]
pub enum RossConvertPacketError {
    /// Provided packet was not appropriately sized
    WrongSize,
    /// The provided packet was of a wrong type
    WrongType,
    /// Event packet specific error
    EventPacket(RossEventPacketError),
}

pub trait RossConvertPacket<T> {
    fn try_from_packet(packet: &RossPacket) -> Result<T, RossConvertPacketError>;
    fn to_packet(&self) -> RossPacket;
}
