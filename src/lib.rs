// Library interface - clean API for external use

pub mod models;
pub mod scanner;
pub mod display;

// Re-export the main types for easy access
pub use models::{NetworkPort, ProtocolType, PortState, ProcessInfo};
pub use scanner::PortScanner;
pub use display::PortFormatter;

// High-level convenience API
pub fn scan_ports() -> anyhow::Result<Vec<NetworkPort>> {
    let mut scanner = PortScanner::new();
    scanner.scan_all_ports()
}

pub fn format_ports_pretty(ports: &[NetworkPort]) -> String {
    let formatter = PortFormatter::new(true);
    formatter.format_ports(ports)
}

pub fn format_ports_plain(ports: &[NetworkPort]) -> String {
    let formatter = PortFormatter::new(false);
    formatter.format_ports(ports)
}
