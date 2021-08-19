use crate::ross_frame::RossFrameError;
use crate::ross_interface::ross_can::RossCanError;
#[cfg(feature = "std")]
use crate::ross_interface::ross_serial::RossSerialError;
use crate::ross_interface::ross_usart::RossUsartError;
use crate::ross_packet::RossPacket;
use crate::ross_packet::RossPacketBuilderError;

pub mod ross_can;
#[cfg(feature = "std")]
pub mod ross_serial;
pub mod ross_usart;

#[derive(Debug)]
pub enum RossInterfaceError {
    CanError(RossCanError),
    UsartError(RossUsartError),
    #[cfg(feature = "std")]
    SerialError(RossSerialError),
    BuilderError(RossPacketBuilderError),
    FrameError(RossFrameError),
    NoPacketReceived,
}

pub trait RossInterface {
    fn try_get_packet(&mut self) -> Result<RossPacket, RossInterfaceError>;
    fn try_send_packet(&mut self, packet: &RossPacket) -> Result<(), RossInterfaceError>;
}
