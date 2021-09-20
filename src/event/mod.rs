pub mod bcm_event;
pub mod bootloader_event;
pub mod configurator_event;
pub mod event_code;
pub mod general_event;
pub mod programmer_event;
pub mod internal_event;

use crate::event::bootloader_event::*;
use crate::event::general_event::*;
use crate::event::programmer_event::*;
use crate::event::bcm_event::*;
use crate::event::internal_event::*;

#[derive(Debug, PartialEq)]
pub enum EventError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Ack(AckEvent),
    Data(DataEvent),

    BootloaderHello(BootloaderHelloEvent),

    ProgrammerHello(ProgrammerHelloEvent),
    ProgrammerStartUpload(ProgrammerStartUploadEvent),

    BcmChangeBrightness(BcmChangeBrightnessEvent),

    InternalButtonPressed(InternalButtonPressedEvent),
    InternalButtonReleased(InternalButtonReleasedEvent),
}
