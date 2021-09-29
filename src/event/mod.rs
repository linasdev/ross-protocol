pub mod bcm_event;
pub mod bootloader_event;
pub mod button_event;
pub mod configurator_event;
pub mod event_code;
pub mod general_event;
pub mod internal_event;
pub mod programmer_event;

#[derive(Debug, PartialEq)]
pub enum EventError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}
