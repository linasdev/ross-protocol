#![allow(mutable_transmutes)]
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::transmute;

use crate::convert_packet::ConvertPacket;
use crate::interface::*;
use crate::packet::Packet;

pub const BROADCAST_ADDRESS: u16 = 0xffff;

#[derive(Debug)]
pub enum ProtocolError {
    InterfaceError(InterfaceError),
    NoSuchHandler,
    PacketTimeout,
}

pub struct Protocol<'a, I: Interface> {
    device_address: u16,
    interface: I,
    #[cfg(not(feature = "send"))]
    handlers: BTreeMap<u32, (Box<dyn FnMut(&Packet, &mut Self) + 'a>, bool)>,
    #[cfg(feature = "send")]
    handlers: BTreeMap<u32, (Box<dyn FnMut(&Packet, &mut Self) + Send + 'a>, bool)>,
}

impl<'a, I: Interface> Protocol<'a, I> {
    pub fn new(device_address: u16, interface: I) -> Self {
        Protocol {
            device_address,
            interface,
            handlers: BTreeMap::new(),
        }
    }

    pub fn tick(&mut self) -> Result<(), ProtocolError> {
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
                InterfaceError::NoPacketReceived => Ok(()),
                _ => Err(ProtocolError::InterfaceError(err)),
            },
        }
    }

    pub fn send_packet(&mut self, packet: &Packet) -> Result<(), ProtocolError> {
        if packet.device_address == self.device_address {
            self.handle_packet(&packet, true);

            if self.device_address != BROADCAST_ADDRESS {
                return Ok(());
            }
        }

        match self.interface.try_send_packet(packet) {
            Ok(_) => Ok(()),
            Err(err) => Err(ProtocolError::InterfaceError(err)),
        }
    }

    pub fn add_packet_handler<'s>(
        &'s mut self,
        #[cfg(not(feature = "send"))]
        handler: Box<dyn FnMut(&Packet, &mut Self) + 'a>,
        #[cfg(feature = "send")]
        handler: Box<dyn FnMut(&Packet, &mut Self) + Send + 'a>,
        capture_all_addresses: bool,
    ) -> Result<u32, ProtocolError> {
        let id = self.get_next_handler_id();

        self.handlers.insert(id, (handler, capture_all_addresses));

        Ok(id)
    }

    pub fn remove_packet_handler(&mut self, id: u32) -> Result<(), ProtocolError> {
        match self.handlers.remove(&id) {
            None => Err(ProtocolError::NoSuchHandler),
            Some(_) => Ok(()),
        }
    }

    pub fn exchange_packet<F: Fn(), R: ConvertPacket<R>>(
        &mut self,
        packet: Packet,
        capture_all_addresses: bool,
        wait_closure: F,
    ) -> Result<R, ProtocolError> {
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
                    InterfaceError::NoPacketReceived => break,
                    _ => return Err(ProtocolError::InterfaceError(err)),
                },
            }
        }

        Err(ProtocolError::PacketTimeout)
    }

    pub fn exchange_packets<F: Fn(), R: ConvertPacket<R>>(
        &mut self,
        packet: Packet,
        capture_all_addresses: bool,
        wait_closure: F,
    ) -> Result<Vec<R>, ProtocolError> {
        let mut events = vec![];

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
                            events.push(received_event);
                        }
                    }
                }
                Err(err) => match err {
                    InterfaceError::NoPacketReceived => break,
                    _ => return Err(ProtocolError::InterfaceError(err)),
                },
            }
        }

        return Ok(events);
    }

    fn handle_packet(&self, packet: &Packet, owned_address: bool) {
        unsafe {
            for handler in transmute::<&Self, &mut Self>(self).handlers.values_mut() {
                if owned_address || handler.1 {
                    handler.0(packet, transmute(self));
                }
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
