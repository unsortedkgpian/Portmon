use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtocolType {
    Tcp,
    Udp,
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolType::Tcp => write!(f, "TCP"),
            ProtocolType::Udp => write!(f, "UDP"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkPort {
    pub port: u16,
    pub protocol: ProtocolType,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub state: PortState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortState {
    Listening,
    Active,
    Bound,
}

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortState::Listening => write!(f, "LISTENING"),
            PortState::Active => write!(f, "ACTIVE"),
            PortState::Bound => write!(f, "BOUND"),
        }
    }
}

impl NetworkPort {
    pub fn new(port: u16, protocol: ProtocolType) -> Self {
        Self {
            port,
            protocol:protocol.clone(),
            pid: None,
            process_name: None,
            state: match protocol {
                ProtocolType::Tcp => PortState::Listening,
                ProtocolType::Udp => PortState::Active,
            },
        }
    }

    pub fn with_process(mut self, pid: u32, name: String) -> Self {
        self.pid = Some(pid);
        self.process_name = Some(name);
        self
    }

    pub fn display_pid(&self) -> String {
        self.pid.map_or("N/A".to_string(), |p| p.to_string())
    }

    pub fn display_process(&self) -> String {
        self.process_name.as_deref().unwrap_or("Unknown").to_string()
    }
}
