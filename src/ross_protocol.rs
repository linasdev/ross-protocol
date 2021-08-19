use alloc::boxed::Box;
use alloc::collections::BTreeMap;

use crate::ross_convert_packet::RossConvertPacket;
use crate::ross_interface::*;
use crate::ross_packet::RossPacket;

pub const BROADCAST_ADDRESS: u16 = 0xffff;

#[derive(Debug)]
pub enum RossProtocolError {
    InterfaceError(RossInterfaceError),
    NoSuchHandler,
    PacketTimeout,
}

pub struct RossProtocol<'a, I: RossInterface> {
    device_address: u16,
    interface: I,
    handlers: BTreeMap<u32, (Box<dyn FnMut(&RossPacket, &mut I) + 'a>, bool)>,
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
                if packet.device_address == self.device_address
                    || packet.device_address == BROADCAST_ADDRESS
                {
                    self.handle_packet(&packet, true);
                } else {
                    self.handle_packet(&packet, false);
                }

                Ok(())
            }
            Err(err) => match err {
                RossInterfaceError::NoPacketReceived => Ok(()),
                _ => Err(RossProtocolError::InterfaceError(err)),
            },
        }
    }

    pub fn send_packet(&mut self, packet: &RossPacket) -> Result<(), RossProtocolError> {
        match self.interface.try_send_packet(packet) {
            Ok(_) => Ok(()),
            Err(err) => Err(RossProtocolError::InterfaceError(err)),
        }
    }

    pub fn add_packet_handler<'s>(
        &'s mut self,
        handler: Box<dyn FnMut(&RossPacket, &mut I) + 'a>,
        capture_all_addresses: bool,
    ) -> Result<u32, RossProtocolError> {
        let id = self.get_next_handler_id();

        self.handlers.insert(id, (handler, capture_all_addresses));

        Ok(id)
    }

    pub fn remove_packet_handler(&mut self, id: u32) -> Result<(), RossProtocolError> {
        match self.handlers.remove(&id) {
            None => Err(RossProtocolError::NoSuchHandler),
            Some(_) => Ok(()),
        }
    }

    pub fn exchange_packet<F: Fn(), R: RossConvertPacket<R>>(
        &mut self,
        packet: RossPacket,
        capture_all_addresses: bool,
        retry_count: u32,
        wait_closure: F,
    ) -> Result<R, RossProtocolError> {
        for _ in 0..retry_count {
            self.send_packet(&packet)?;

            wait_closure();

            loop {
                match self.interface.try_get_packet() {
                    Ok(received_packet) => {
                        if capture_all_addresses
                            || received_packet.device_address == self.device_address
                            || received_packet.device_address == BROADCAST_ADDRESS
                        {
                            if let Ok(received_event) = R::try_from_packet(&received_packet) {
                                return Ok(received_event);
                            }
                        }
                    }
                    Err(err) => match err {
                        RossInterfaceError::NoPacketReceived => break,
                        _ => return Err(RossProtocolError::InterfaceError(err)),
                    },
                }
            }
        }

        Err(RossProtocolError::PacketTimeout)
    }

    fn handle_packet(&mut self, packet: &RossPacket, owned_address: bool) {
        for handler in self.handlers.values_mut() {
            if owned_address || handler.1 {
                handler.0(packet, &mut self.interface);
            }
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
