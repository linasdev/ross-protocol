use alloc::collections::BTreeMap;
use alloc::boxed::Box;

use crate::ross_packet::RossPacket;
use crate::ross_interface::*;

pub const BROADCAST_ADDRESS: u16 = 0xffff;
pub const TRANSACTION_TIMEOUT_MS: u128 = 2000;
pub const PACKET_TIMEOUT_MS: u128 = 500;


#[derive(Debug)]
pub enum RossProtocolError {
    InterfaceError(RossInterfaceError),
    NoSuchHandler,
}

pub struct RossProtocol<'a, I: RossInterface> {
    device_address: u16,
    interface: I,
    handlers: BTreeMap<u32, Box<dyn Fn(&RossPacket, &mut I) + 'a>>,
}

impl<'a, I: RossInterface> RossProtocol<'a, I> {
    pub fn new(device_address: u16, interface: I) -> Self {
        RossProtocol {
            device_address,
            interface,
            handlers: BTreeMap::new(),
        }
    }

    pub fn tick(&mut self) -> Result<(), RossProtocolError> {
        match self.interface.try_get_packet() {
            Ok(packet) => {
                if packet.device_address == self.device_address ||
                   packet.device_address == BROADCAST_ADDRESS {
                    self.handle_packet(&packet);
                }

                Ok(())
            },
            Err(err) => match err {
                RossInterfaceError::NoPacketReceived => Ok(()),
                _ => Err(RossProtocolError::InterfaceError(err)),
            }
        }
    }

    pub fn send_packet(&mut self, packet: &RossPacket) -> Result<(), RossProtocolError> {
        match self.interface.try_send_packet(packet) {
            Ok(_) => Ok(()),
            Err(err) => Err(RossProtocolError::InterfaceError(err)),
        }
    }

    pub fn add_packet_handler(&'a mut self, handler: Box<dyn Fn(&RossPacket, &mut I) + 'a>) -> Result<u32, RossProtocolError> {
        let id = self.get_next_handler_id();

        self.handlers.insert(id, handler);

        Ok(id)
    }

    pub fn remove_packet_handler(&mut self, id: u32) -> Result<(), RossProtocolError> {
        match self.handlers.remove(&id) {
            None => Err(RossProtocolError::NoSuchHandler),
            Some(_) => Ok(()),
        }
    }

    fn handle_packet(&mut self, packet: &RossPacket) {
        for handler in self.handlers.values() {
            handler(packet, &mut self.interface);
        }
    }

    fn get_next_handler_id(&self) -> u32 {
        let mut first_available_id = 0;

        for id in self.handlers.keys() {
            if first_available_id == *id {
                first_available_id += 1;
            }
        }

        return first_available_id;
    }
}
