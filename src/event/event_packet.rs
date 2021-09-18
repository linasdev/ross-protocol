use crate::event::bootloader_event::*;
use crate::event::general_event::*;
use crate::event::programmer_event::*;
use crate::event::bcm_event::*;

#[derive(Debug, PartialEq)]
pub enum EventPacketError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}

#[derive(Debug, PartialEq)]
pub enum EventPacket {
    Ack(AckEvent),
    Data(DataEvent),

    BootloaderHello(BootloaderHelloEvent),

    ProgrammerHello(ProgrammerHelloEvent),
    ProgrammerStartUpload(ProgrammerStartUploadEvent),

    BcmChangeBrightness(BcmChangeBrightnessEvent)
}
