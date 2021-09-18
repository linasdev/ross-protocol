use crate::event::event_packet::EventPacketError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub enum ConvertPacketError {
    /// Provided packet was not appropriately sized
    WrongSize,
    /// The provided packet was of a wrong type
    WrongType,
    /// Event packet specific error
    EventPacket(EventPacketError),
}

pub trait ConvertPacket<T> {
    fn try_from_packet(packet: &Packet) -> Result<T, ConvertPacketError>;
    fn to_packet(&self) -> Packet;
}
