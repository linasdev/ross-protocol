use crate::ross_event::ross_bootloader_event::*;
use crate::ross_event::ross_programmer_event::*;

#[derive(Debug)]
pub enum RossEventPacketError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}

pub enum RossEventPacket {
    BootloaderHello(RossBootloaderHelloEvent),
    BootloaderStartUpload(RossBootloaderStartUploadEvent),

    ProgrammerHello(RossProgrammerHelloEvent),
    ProgrammerStartUpload(RossProgrammerStartUploadEvent),
}
