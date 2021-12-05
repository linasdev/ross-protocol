pub mod bcm;
pub mod bootloader;
pub mod button;
pub mod configurator;
pub mod event_code;
pub mod general;
pub mod internal;
pub mod message;
pub mod programmer;
pub mod relay;

#[derive(Debug, PartialEq)]
pub enum EventError {
    /// The provided packet was of a wrong event type
    WrongEventType,
}
