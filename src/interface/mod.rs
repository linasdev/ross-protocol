use crate::frame::FrameError;
use crate::interface::can::CanError;
#[cfg(feature = "std")]
use crate::interface::serial::SerialError;
use crate::interface::usart::UsartError;
use crate::packet::Packet;
use crate::packet::PacketBuilderError;

pub mod can;
#[cfg(feature = "std")]
pub mod serial;
pub mod usart;

#[derive(Debug)]
pub enum InterfaceError {
    CanError(CanError),
    UsartError(UsartError),
    #[cfg(feature = "std")]
    SerialError(SerialError),
    BuilderError(PacketBuilderError),
    FrameError(FrameError),
    NoPacketReceived,
}

pub trait Interface {
    fn try_get_packet(&mut self) -> Result<Packet, InterfaceError>;
    fn try_send_packet(&mut self, packet: &Packet) -> Result<(), InterfaceError>;
}
