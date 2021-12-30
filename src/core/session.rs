use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SessionType {
    Dedicated,
    Client,
    Host,
}

impl Default for SessionType {
    fn default() -> Self {
        SessionType::Dedicated
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Session {
    pub session_type: SessionType,
    pub address: Option<String>,
    pub port: u16,
    pub debug: bool,
    pub tickrate: f64,
}

impl Session {
    pub fn new(is_server: bool, is_client: bool, address: Option<String>, port: u16, debug: bool, tickrate: f64) -> Self {
        Self {
            session_type: get_session_type(is_server, is_client),
            address,
            port,
            debug,
            tickrate,
        }
    }

    pub fn is_headless(&self) -> bool {
        self.session_type == SessionType::Dedicated
    }

    pub fn is_server(&self) -> bool {
        self.session_type == SessionType::Dedicated || self.session_type == SessionType::Host
    }

    pub fn is_client(&self) -> bool {
        self.session_type == SessionType::Client || self.session_type == SessionType::Host
    }
}

fn get_session_type(is_server: bool, is_client: bool) -> SessionType {
    if is_server && is_client {
        return SessionType::Host;
    } else if is_server && !is_client {
        return SessionType::Dedicated;
    } else if !is_server && is_client {
        return SessionType::Client;
    } else {
        panic!("Invalid session type");
    }
}

impl Display for SessionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionType::Dedicated => write!(f, "dedicated"),
            SessionType::Client => write!(f, "client"),
            SessionType::Host => write!(f, "host"),
        }
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_headless() {
            write!(f, "{}:{} (headless), t = {}", self.session_type, self.port, self.tickrate)
        } else {
            write!(f, "{}:{}, t = {}", self.session_type, self.port, self.tickrate)
        }
    }
}
