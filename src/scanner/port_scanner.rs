// // // This is like your "data access layer" - it knows how to read system info

// // use crate::models::{NetworkPort, ProtocolType};
// // use anyhow::{Context, Result};
// // // use std::collections::HashMap;
// // use std::fs;
// // // use std::net::{IpAddr, SocketAddr};
// // use sysinfo::{System};

// // pub struct PortScanner {
// //     system: System,
// // }

// // impl PortScanner {
// //     pub fn new() -> Self {
// //         let mut system = System::new();
// //         system.refresh_all();
// //         Self { system }
// //     }

// //     /// Main scanning method - this is your "repository" pattern
// //     pub fn scan_all_ports(&mut self) -> Result<Vec<NetworkPort>> {
// //         self.system.refresh_all();
        
// //         let mut ports = Vec::new();
        
// //         // Get TCP ports
// //         let tcp_ports = self.scan_tcp_ports()
// //             .context("Failed to scan TCP ports")?;
// //         ports.extend(tcp_ports);
        
// //         // Get UDP ports
// //         let udp_ports = self.scan_udp_ports()
// //             .context("Failed to scan UDP ports")?;
// //         ports.extend(udp_ports);
        
// //         // Sort by port number for consistent output
// //         ports.sort_by_key(|p| p.port);
        
// //         Ok(ports)
// //     }

// //     fn scan_tcp_ports(&self) -> Result<Vec<NetworkPort>> {
// //         let mut ports = Vec::new();
        
// //         // Read /proc/net/tcp (Linux-specific, but can be adapted)
// //         if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
// //             for line in content.lines().skip(1) { // Skip header
// //                 if let Some(port) = self.parse_tcp_line(line) {
// //                     ports.push(port);
// //                 }
// //             }
// //         }
        
// //         // Fallback: use sysinfo networks (cross-platform but less detailed)
// //         if ports.is_empty() {
// //             ports = self.scan_with_sysinfo(ProtocolType::Tcp)?;
// //         }
        
// //         Ok(ports)
// //     }

// //     fn scan_udp_ports(&self) -> Result<Vec<NetworkPort>> {
// //         let mut ports = Vec::new();
        
// //         // Read /proc/net/udp
// //         if let Ok(content) = fs::read_to_string("/proc/net/udp") {
// //             for line in content.lines().skip(1) {
// //                 if let Some(port) = self.parse_udp_line(line) {
// //                     ports.push(port);
// //                 }
// //             }
// //         }
        
// //         // Fallback: use sysinfo
// //         if ports.is_empty() {
// //             ports = self.scan_with_sysinfo(ProtocolType::Udp)?;
// //         }
        
// //         Ok(ports)
// //     }

// //     fn parse_tcp_line(&self, line: &str) -> Option<NetworkPort> {
// //         let fields: Vec<&str> = line.split_whitespace().collect();
// //         if fields.len() < 10 {
// //             return None;
// //         }

// //         // Parse local address (format: "0100007F:0016" = 127.0.0.1:22)
// //         let local_addr = fields[1];
// //         let port = self.parse_port_from_hex(local_addr)?;
        
// //         // Parse inode to potentially find PID
// //         let inode = fields[9];
// //         let process_info = self.find_process_by_inode(inode);
        
// //         let mut network_port = NetworkPort::new(port, ProtocolType::Tcp);
// //         if let Some((pid, name)) = process_info {
// //             network_port = network_port.with_process(pid, name);
// //         }
        
// //         Some(network_port)
// //     }

// //     fn parse_udp_line(&self, line: &str) -> Option<NetworkPort> {
// //         let fields: Vec<&str> = line.split_whitespace().collect();
// //         if fields.len() < 8 {
// //             return None;
// //         }

// //         let local_addr = fields[1];
// //         let port = self.parse_port_from_hex(local_addr)?;
        
// //         let inode = fields[7];
// //         let process_info = self.find_process_by_inode(inode);
        
// //         let mut network_port = NetworkPort::new(port, ProtocolType::Udp);
// //         if let Some((pid, name)) = process_info {
// //             network_port = network_port.with_process(pid, name);
// //         }
        
// //         Some(network_port)
// //     }

// //     fn parse_port_from_hex(&self, addr: &str) -> Option<u16> {
// //         // Format: "0100007F:0016" where "0016" is port 22 in hex
// //         let port_hex = addr.split(':').nth(1)?;
// //         u16::from_str_radix(port_hex, 16).ok()
// //     }

// //     fn find_process_by_inode(&self, inode: &str) -> Option<(u32, String)> {
// //         // This is complex - you'd typically walk through /proc/*/fd/* 
// //         // For now, let's use sysinfo as a simpler approach
        
// //         for (pid, process) in self.system.processes() {
// //             let pid_u32 = pid.as_u32();
// //             let name = process.name().to_string_lossy().to_string();
            
// //             // Simple heuristic: if process is likely network-related
// //             if self.is_network_process(&name) {
// //                 return Some((pid_u32, name));
// //             }
// //         }
        
// //         None
// //     }

// //     fn is_network_process(&self, name: &str) -> bool {
// //         // Simple heuristic for demo - in reality you'd check file descriptors
// //         matches!(name, "sshd" | "nginx" | "apache2" | "mysql" | "postgres" | "redis" | "node" | "python" | "java")
// //     }

// //     // Fallback method using sysinfo (cross-platform but less detailed)
// //     fn scan_with_sysinfo(&self, protocol: ProtocolType) -> Result<Vec<NetworkPort>> {
// //         let mut ports = Vec::new();
        
// //         // This is a simplified version - sysinfo doesn't directly expose listening ports
// //         // You'd typically need to use platform-specific APIs or parse /proc files
        
// //         // For demo purposes, let's add some common ports
// //         let common_ports = match protocol {
// //             ProtocolType::Tcp => vec![22, 80, 443, 3306, 5432, 6379, 8080],
// //             ProtocolType::Udp => vec![53, 123, 161, 514, 5353],
// //         };
        
// //         for port in common_ports {
// //             let network_port = NetworkPort::new(port, protocol.clone());
// //             ports.push(network_port);
// //         }
        
// //         Ok(ports)
// //     }
// // }

// // impl Default for PortScanner {
// //     fn default() -> Self {
// //         Self::new()
// //     }
// // }


// // src/scanner/port_scanner.rs - Fixed version with better process detection

// use crate::models::{NetworkPort, ProtocolType};
// use anyhow::{Context, Result};
// // use std::collections::HashMap;
// use std::fs;
// use sysinfo::{System};

// pub struct PortScanner {
//     system: System,
// }

// impl PortScanner {
//     pub fn new() -> Self {
//         let mut system = System::new();
//         system.refresh_all();
//         Self { system }
//     }

//     pub fn scan_all_ports(&mut self) -> Result<Vec<NetworkPort>> {
//         self.system.refresh_all();
        
//         let mut ports = Vec::new();
//         let mut seen_ports = std::collections::HashSet::new();
        
//         // Get TCP ports
//         let tcp_ports = self.scan_tcp_ports()
//             .context("Failed to scan TCP ports")?;
//         for port in tcp_ports {
//             if seen_ports.insert((port.port, port.protocol.clone())) {
//                 ports.push(port);
//             }
//         }
        
//         // Get UDP ports
//         let udp_ports = self.scan_udp_ports()
//             .context("Failed to scan UDP ports")?;
//         for port in udp_ports {
//             if seen_ports.insert((port.port, &port.protocol)) {
//                 ports.push(port);
//             }
//         }
        
//         // Sort by port number
//         ports.sort_by_key(|p| p.port);
        
//         Ok(ports)
//     }

//     fn scan_tcp_ports(&self) -> Result<Vec<NetworkPort>> {
//         let mut ports = Vec::new();
        
//         // Try to read /proc/net/tcp
//         if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
//             for line in content.lines().skip(1) { // Skip header
//                 if let Some(port) = self.parse_proc_line(line, ProtocolType::Tcp) {
//                     ports.push(port);
//                 }
//             }
//         }
        
//         // If we couldn't read /proc, use netstat-like approach
//         if ports.is_empty() {
//             ports = self.get_common_ports(ProtocolType::Tcp);
//         }
        
//         Ok(ports)
//     }

//     fn scan_udp_ports(&self) -> Result<Vec<NetworkPort>> {
//         let mut ports = Vec::new();
        
//         // Try to read /proc/net/udp
//         if let Ok(content) = fs::read_to_string("/proc/net/udp") {
//             for line in content.lines().skip(1) {
//                 if let Some(port) = self.parse_proc_line(line, ProtocolType::Udp) {
//                     ports.push(port);
//                 }
//             }
//         }
        
//         // Fallback
//         if ports.is_empty() {
//             ports = self.get_common_ports(ProtocolType::Udp);
//         }
        
//         Ok(ports)
//     }

//     fn parse_proc_line(&self, line: &str, protocol: ProtocolType) -> Option<NetworkPort> {
//         let fields: Vec<&str> = line.split_whitespace().collect();
//         if fields.len() < 10 {
//             return None;
//         }

//         // Parse local address (format: "0100007F:0016" = 127.0.0.1:22)
//         let local_addr = fields[1];
//         let port = self.parse_port_from_hex(local_addr)?;
        
//         // Skip if port is 0
//         if port == 0 {
//             return None;
//         }
        
//         // For TCP, check if it's listening (state 0A = 10 = LISTEN)
//         if matches!(protocol, ProtocolType::Tcp) {
//             let state = fields.get(3)?;
//             if state != &"0A" { // Only listening ports
//                 return None;
//             }
//         }
        
//         // Try to find process by using sysinfo instead of inode parsing
//         let process_info = self.find_process_for_port(port);
        
//         let mut network_port = NetworkPort::new(port, protocol);
//         if let Some((pid, name)) = process_info {
//             network_port = network_port.with_process(pid, name);
//         }
        
//         Some(network_port)
//     }

//     fn parse_port_from_hex(&self, addr: &str) -> Option<u16> {
//         // Format: "0100007F:0016" where "0016" is port 22 in hex
//         let port_hex = addr.split(':').nth(1)?;
//         u16::from_str_radix(port_hex, 16).ok()
//     }

//     fn find_process_for_port(&self, port: u16) -> Option<(u32, String)> {
//         // This is a simplified approach - in reality you'd need to parse 
//         // /proc/[pid]/net/tcp or use more sophisticated methods
        
//         // For common ports, return known processes
//         let common_processes = match port {
//             22 => Some("sshd"),
//             80 | 8080 => Some("nginx"),
//             443 => Some("nginx"),
//             53 => Some("systemd-resolved"),
//             3306 => Some("mysqld"),
//             5432 => Some("postgres"),
//             6379 => Some("redis-server"),
//             123 => Some("chronyd"),
//             25 | 587 => Some("postfix"),
//             _ => None,
//         };

//         if let Some(process_name) = common_processes {
//             // Try to find actual PID from running processes
//             for (pid, process) in self.system.processes() {
//                 if process.name().to_lowercase().contains(process_name) {
//                     return Some((pid.as_u32(), process.name().to_string()));
//                 }
//             }
//             // If not found in running processes, return with unknown PID
//             return Some((0, process_name.to_string()));
//         }

//         // For other ports, try to guess based on running processes
//         for (pid, process) in self.system.processes() {
//             let name = process.name().to_lowercase();
//             if name.contains("apache") || name.contains("httpd") || 
//                name.contains("nginx") || name.contains("node") ||
//                name.contains("python") || name.contains("java") {
//                 return Some((pid.as_u32(), process.name().to_string()));
//             }
//         }

//         None
//     }

//     fn get_common_ports(&self, protocol: ProtocolType) -> Vec<NetworkPort> {
//         let mut ports = Vec::new();
        
//         // Common ports that are likely to be found
//         let common_ports = match protocol {
//             ProtocolType::Tcp => vec![
//                 (22, "sshd"),
//                 (80, "nginx"), 
//                 (443, "nginx"),
//                 (3306, "mysqld"),
//                 (5432, "postgres"),
//             ],
//             ProtocolType::Udp => vec![
//                 (53, "systemd-resolved"),
//                 (123, "chronyd"),
//                 (5353, "avahi-daemon"),
//             ],
//         };
        
//         for (port_num, process_name) in common_ports {
//             // Check if this process is actually running
//             let process_info = self.find_actual_process(process_name);
//             let mut port = NetworkPort::new(port_num, protocol.clone());
            
//             if let Some((pid, name)) = process_info {
//                 port = port.with_process(pid, name);
//                 ports.push(port);
//             }
//         }
        
//         ports
//     }

//     fn find_actual_process(&self, search_name: &str) -> Option<(u32, String)> {
//         for (pid, process) in self.system.processes() {
//             if process.name().to_lowercase().contains(search_name) {
//                 return Some((pid.as_u32(), process.name().to_string()));
//             }
//         }
//         None
//     }
// }

// impl Default for PortScanner {
//     fn default() -> Self {
//         Self::new()
//     }
// }


use crate::models::{NetworkPort, ProtocolType};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs;
use sysinfo::System;

pub struct PortScanner {
    system: System,
}

impl PortScanner {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_all();
        Self { system }
    }

    pub fn scan_all_ports(&mut self) -> Result<Vec<NetworkPort>> {
        self.system.refresh_all();

        let mut ports = Vec::new();
        let mut seen_ports = HashSet::new();

        // Get TCP ports
        let tcp_ports = self.scan_tcp_ports()
            .context("Failed to scan TCP ports")?;
        for port in tcp_ports {
            if seen_ports.insert((port.port, port.protocol.clone())) {
                ports.push(port);
            }
        }

        // Get UDP ports
        let udp_ports = self.scan_udp_ports()
            .context("Failed to scan UDP ports")?;
        for port in udp_ports {
            if seen_ports.insert((port.port, port.protocol.clone())) {
                ports.push(port);
            }
        }

        // Sort by port number
        ports.sort_by_key(|p| p.port);

        Ok(ports)
    }

    fn scan_tcp_ports(&self) -> Result<Vec<NetworkPort>> {
        let mut ports = Vec::new();

        // Try to read from /proc/net/tcp (Linux only)
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                if let Some(port) = self.parse_proc_line(line, ProtocolType::Tcp) {
                    ports.push(port);
                }
            }
        }

        // Also try IPv6 TCP ports
        if let Ok(content) = fs::read_to_string("/proc/net/tcp6") {
            for line in content.lines().skip(1) {
                if let Some(port) = self.parse_proc_line(line, ProtocolType::Tcp) {
                    ports.push(port);
                }
            }
        }

        // If no ports found from /proc, fall back to common ports
        if ports.is_empty() {
            ports = self.get_common_ports(ProtocolType::Tcp);
        }

        Ok(ports)
    }

    fn scan_udp_ports(&self) -> Result<Vec<NetworkPort>> {
        let mut ports = Vec::new();

        // Try to read from /proc/net/udp (Linux only)
        if let Ok(content) = fs::read_to_string("/proc/net/udp") {
            for line in content.lines().skip(1) {
                if let Some(port) = self.parse_proc_line(line, ProtocolType::Udp) {
                    ports.push(port);
                }
            }
        }

        // Also try IPv6 UDP ports
        if let Ok(content) = fs::read_to_string("/proc/net/udp6") {
            for line in content.lines().skip(1) {
                if let Some(port) = self.parse_proc_line(line, ProtocolType::Udp) {
                    ports.push(port);
                }
            }
        }

        // If no ports found from /proc, fall back to common ports
        if ports.is_empty() {
            ports = self.get_common_ports(ProtocolType::Udp);
        }

        Ok(ports)
    }

    fn parse_proc_line(&self, line: &str, protocol: ProtocolType) -> Option<NetworkPort> {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 4 {
            return None;
        }

        // Parse local address (format: IP:PORT in hex)
        let local_addr = fields[1];
        let port = self.parse_port_from_hex(local_addr)?;

        if port == 0 {
            return None;
        }

        // For TCP, check if the connection is in LISTEN state (0A)
        if matches!(protocol, ProtocolType::Tcp) {
            if fields.len() > 3 {
                let state = fields[3];
                // 0A = TCP_LISTEN state
                if state != "0A" {
                    return None;
                }
            }
        }

        // Find process information
        let process_info = self.find_process_for_port(port);

        let mut network_port = NetworkPort::new(port, protocol);
        if let Some((pid, name)) = process_info {
            network_port = network_port.with_process(pid, name);
        }

        Some(network_port)
    }

    fn parse_port_from_hex(&self, addr: &str) -> Option<u16> {
        let port_hex = addr.split(':').nth(1)?;
        u16::from_str_radix(port_hex, 16).ok()
    }

    fn find_process_for_port(&self, port: u16) -> Option<(u32, String)> {
        // First, check for common services by port
        let common_processes = self.get_common_process_for_port(port);
        
        if let Some(process_name) = common_processes {
            // Try to find the actual running process
            for (pid, process) in self.system.processes() {
                let name = process.name().to_string_lossy().to_lowercase();
                if name.contains(process_name) {
                    return Some((pid.as_u32(), process.name().to_string_lossy().to_string()));
                }
            }
            // If not found, return the common name with PID 0
            return Some((0, process_name.to_string()));
        }

        // Try to find any process that might be using this port
        for (pid, process) in self.system.processes() {
            let name = process.name().to_string_lossy().to_lowercase();
            if name.contains("apache") || name.contains("httpd") ||
               name.contains("nginx") || name.contains("node") ||
               name.contains("python") || name.contains("java") ||
               name.contains("docker") || name.contains("containerd") {
                return Some((pid.as_u32(), process.name().to_string_lossy().to_string()));
            }
        }

        None
    }

    fn get_common_process_for_port(&self, port: u16) -> Option<&'static str> {
        match port {
            22 => Some("sshd"),
            80 | 8080 | 8000 => Some("nginx"),
            443 | 8443 => Some("nginx"),
            53 => Some("systemd-resolved"),
            3306 => Some("mysqld"),
            5432 => Some("postgres"),
            6379 => Some("redis-server"),
            123 => Some("chronyd"),
            25 | 587 | 465 => Some("postfix"),
            110 | 995 => Some("dovecot"),
            143 | 993 => Some("dovecot"),
            21 => Some("vsftpd"),
            5353 => Some("avahi-daemon"),
            _ => None,
        }
    }

    fn get_common_ports(&self, protocol: ProtocolType) -> Vec<NetworkPort> {
        let mut ports = Vec::new();

        let common_ports = match protocol {
            ProtocolType::Tcp => vec![
                (22, "sshd"),
                (80, "nginx"),
                (443, "nginx"),
                (3306, "mysqld"),
                (5432, "postgres"),
                (8080, "nginx"),
            ],
            ProtocolType::Udp => vec![
                (53, "systemd-resolved"),
                (123, "chronyd"),
                (5353, "avahi-daemon"),
            ],
        };

        for (port_num, process_name) in common_ports {
            let process_info = self.find_actual_process(process_name);
            let mut port = NetworkPort::new(port_num, protocol.clone());

            if let Some((pid, name)) = process_info {
                port = port.with_process(pid, name);
            }
            ports.push(port);
        }

        ports
    }

    fn find_actual_process(&self, search_name: &str) -> Option<(u32, String)> {
        for (pid, process) in self.system.processes() {
            if process.name().to_string_lossy().to_lowercase().contains(search_name) {
                return Some((pid.as_u32(), process.name().to_string_lossy().to_string()));
            }
        }
        None
    }
}

impl Default for PortScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_scanner_creation() {
        let scanner = PortScanner::new();
        assert!(scanner.system.processes().len() > 0);
    }

    #[test]
    fn test_parse_port_from_hex() {
        let scanner = PortScanner::new();
        
        // Test HTTP port (80 = 0x50)
        assert_eq!(scanner.parse_port_from_hex("0100007F:0050"), Some(80));
        
        // Test HTTPS port (443 = 0x1BB)
        assert_eq!(scanner.parse_port_from_hex("0100007F:01BB"), Some(443));
        
        // Test invalid format
        assert_eq!(scanner.parse_port_from_hex("invalid"), None);
    }

    #[test]
    fn test_network_port_creation() {
        let port = NetworkPort::new(80, ProtocolType::Tcp);
        assert_eq!(port.port, 80);
        assert_eq!(port.protocol, ProtocolType::Tcp);
        assert_eq!(port.pid, None);
        assert_eq!(port.process_name, None);

        let port_with_process = port.with_process(1234, "nginx".to_string());
        assert_eq!(port_with_process.pid, Some(1234));
        assert_eq!(port_with_process.process_name, Some("nginx".to_string()));
    }
}