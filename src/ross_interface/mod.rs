use crate::ross_interface::ross_can::RossCanError;
use crate::ross_interface::ross_usart::RossUsartError;
#[cfg(feature="std")]
use crate::ross_interface::ross_serial::RossSerialError;
use crate::ross_packet::RossPacketBuilderError;
use crate::ross_frame::RossFrameError;
use crate::ross_packet::RossPacket;

pub mod ross_can;
pub mod ross_usart;
#[cfg(feature="std")]
pub mod ross_serial;

#[derive(Debug)]
pub enum RossInterfaceError {
    CanError(RossCanError),
    UsartError(RossUsartError),
    #[cfg(feature="std")]
    SerialError(RossSerialError),
    BuilderError(RossPacketBuilderError),
    FrameError(RossFrameError),
    NoPacketReceived,
}

pub trait RossInterface {
    fn try_get_packet(&mut self) -> Result<RossPacket, RossInterfaceError>;
    fn try_send_packet(&mut self, packet: &RossPacket) -> Result<(), RossInterfaceError>;
}
