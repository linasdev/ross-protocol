use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub enum ConvertPacketError {
    /// Provided packet was not appropriately sized
    WrongSize,
    /// The provided packet was of a wrong type
    WrongType,
    /// Event specific error
    Event(EventError),
}

pub trait ConvertPacket<T> {
    fn try_from_packet(packet: &Packet) -> Result<T, ConvertPacketError>;
    fn to_packet(&self) -> Packet;
}
