use crate::event::EventError;
use crate::packet::Packet;

#[derive(Debug, PartialEq)]
pub enum ConvertPacketError {
    WrongSize,
    UnknownEnumVariant,
    WrongType,
    Event(EventError),
}

pub trait ConvertPacket<T> {
    fn try_from_packet(packet: &Packet) -> Result<T, ConvertPacketError>;
    fn to_packet(&self) -> Packet;
}
