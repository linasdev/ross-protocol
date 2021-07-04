use crate::ross_event::ross_bootloader_event::*;
use crate::ross_event::ross_programmer_event::*;

#[derive(Debug, PartialEq)]
pub enum RossEventPacketError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}

#[derive(Debug, PartialEq)]
pub enum RossEventPacket {
    BootloaderHello(RossBootloaderHelloEvent),
    BootloaderStartUpload(RossBootloaderStartUploadEvent),

    ProgrammerHello(RossProgrammerHelloEvent),
    ProgrammerStartUpload(RossProgrammerStartUploadEvent),
}
