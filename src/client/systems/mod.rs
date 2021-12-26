mod c_init;
mod c_packets;
mod c_messages;
mod c_input;

pub(crate) use c_init::*;
pub(crate) use c_packets::*;
pub(crate) use c_messages::*;
pub(crate) use c_input::*;

pub(crate) type ServerIds = std::collections::HashMap<u32, (u32, u32)>;
