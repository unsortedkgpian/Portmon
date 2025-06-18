// // This is your "presentation layer" - handles all the visual formatting

// use crate::models::{NetworkPort, ProtocolType};
// use colored::*;
// use tabled::{Table, Tabled};
// use chrono::Local;
// use tabled::settings::{Style, object::Rows, Alignment};


// #[derive(Tabled)]
// struct PortDisplay {
//     #[tabled(rename = "PORT")]
//     port: String,
//     #[tabled(rename = "PID")]
//     pid: String,
//     #[tabled(rename = "PROCESS")]
//     process: String,
//     #[tabled(rename = "STATE")]
//     state: String,
// }

// pub struct PortFormatter {
//     show_colors: bool,
// }

// impl PortFormatter {
//     pub fn new(show_colors: bool) -> Self {
//         Self { show_colors }
//     }

//     pub fn format_ports(&self, ports: &[NetworkPort]) -> String {
//         let mut output = String::new();
        
//         // Header with timestamp
//         output.push_str(&self.create_header());
//         output.push('\n');
        
//         // Separate TCP and UDP ports
//         let tcp_ports: Vec<_> = ports.iter()
//             .filter(|p| matches!(p.protocol, ProtocolType::Tcp))
//             .collect();
//         let udp_ports: Vec<_> = ports.iter()
//             .filter(|p| matches!(p.protocol, ProtocolType::Udp))
//             .collect();
        
//         // TCP Section
//         if !tcp_ports.is_empty() {
//             output.push_str(&self.format_section("🔗 TCP PORTS (LISTENING)", &tcp_ports));
//             output.push('\n');
//         }
        
//         // UDP Section
//         if !udp_ports.is_empty() {
//             output.push_str(&self.format_section("📡 UDP PORTS (ACTIVE)", &udp_ports));
//             output.push('\n');
//         }
        
//         // Footer
//         output.push_str(&self.create_footer());
        
//         output
//     }

//     fn create_header(&self) -> String {
//         let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
//         let title = "🌐 NETWORK PORT MONITOR 🌐";
        
//         if self.show_colors {
//             format!(
//                 "{}{}{}",
//                 "┌".repeat(60).cyan().bold(),
//                 format!("\n│{:^58}│", title).cyan().bold(),
//                 format!("\n│{:^58}│\n", timestamp).white(),
//             )
//         } else {
//             format!(
//                 "{}\n{:^60}\n{:^60}\n{}",
//                 "=".repeat(60),
//                 title,
//                 timestamp,
//                 "=".repeat(60)
//             )
//         }
//     }

//     fn format_section(&self, title: &str, ports: &[&NetworkPort]) -> String {
//         let mut output = String::new();
        
//         // Section header
//         if self.show_colors {
//             output.push_str(&format!("├{}┤\n", "─".repeat(58)).cyan());
//             output.push_str(&format!("│{:^58}│\n", title).green().bold().to_string());
//             output.push_str(&format!("├{}┤\n", "─".repeat(58)).cyan());
//         } else {
//             output.push_str(&format!("{}\n{:^60}\n{}\n", "-".repeat(60), title, "-".repeat(60)));
//         }
        
//         // Create table data
//         let mut table_data = Vec::new();
//         for port in ports {
//             let port_display = PortDisplay {
//                 port: if self.show_colors {
//                     port.port.to_string().purple().bold().to_string()
//                 } else {
//                     port.port.to_string()
//                 },
//                 pid: if self.show_colors {
//                     port.display_pid().yellow().bold().to_string()
//                 } else {
//                     port.display_pid()
//                 },
//                 process: if self.show_colors {
//                     port.display_process().white().bold().to_string()
//                 } else {
//                     port.display_process()
//                 },
//                 state: if self.show_colors {
//                     port.state.to_string().green().to_string()
//                 } else {
//                     port.state.to_string()
//                 },
//             };
//             table_data.push(port_display);
//         }
        
//         // Create and style table
//         let mut table = Table::new(&table_data);
// let table = if self.show_colors {
//     table
//         .with(Style::modern())
//         .modify(Rows::new(0..1), Alignment::center())
//         .modify(Rows::new(1..), Alignment::left())
// } else {
//     table
//         .with(Style::ascii())
//         .modify(Rows::new(0..1), Alignment::center())
//         .modify(Rows::new(1..), Alignment::left())
// };

        
        
//         // Add table to output with proper indentation
//         let table_str = table.to_string();
//         for line in table_str.lines() {
//             if self.show_colors {
//                 output.push_str(&format!("│ {:<56} │\n", line));
//             } else {
//                 output.push_str(&format!("{}\n", line));
//             }
//         }
        
//         output
//     }

//     fn create_footer(&self) -> String {
//         let tip = "💡 Tip: Run with --watch for real-time monitoring";
//         let complete = "✨ Scan complete! ✨";
        
//         if self.show_colors {
//             format!(
//                 "{}{}{}",
//                 format!("└{}┘\n", "─".repeat(58)).cyan(),
//                 tip.white().bold(),
//                 format!("\n{}", complete).green().bold()
//             )
//         } else {
//             format!("{}\n{}\n{}", "=".repeat(60), tip, complete)
//         }
//     }
// }

// impl Default for PortFormatter {
//     fn default() -> Self {
//         Self::new(true)
//     }
// }
// src/display/formatter.rs - Fixed version

use crate::models::{NetworkPort, ProtocolType};
use colored::*;
use chrono::Local;

pub struct PortFormatter {
    show_colors: bool,
}

impl PortFormatter {
    pub fn new(show_colors: bool) -> Self {
        Self { show_colors }
    }

    pub fn format_ports(&self, ports: &[NetworkPort]) -> String {
        let mut output = String::new();
        
        // Header with timestamp
        output.push_str(&self.create_header());
        output.push('\n');
        
        // Separate TCP and UDP ports
        let tcp_ports: Vec<_> = ports.iter()
            .filter(|p| matches!(p.protocol, ProtocolType::Tcp))
            .collect();
        let udp_ports: Vec<_> = ports.iter()
            .filter(|p| matches!(p.protocol, ProtocolType::Udp))
            .collect();
        
        // TCP Section
        if !tcp_ports.is_empty() {
            output.push_str(&self.format_section("🔗 TCP PORTS (LISTENING)", &tcp_ports));
            output.push('\n');
        }
        
        // UDP Section
        if !udp_ports.is_empty() {
            output.push_str(&self.format_section("📡 UDP PORTS (ACTIVE)", &udp_ports));
            output.push('\n');
        }
        
        // Footer
        output.push_str(&self.create_footer());
        
        output
    }

    fn create_header(&self) -> String {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let title = "🌐 NETWORK PORT MONITOR 🌐";
        
        if self.show_colors {
            format!(
                "{}\n{}\n{}\n{}",
                format!(
                    "{}{}{}",
                    "┌".cyan().bold(),
                    "─".repeat(58).cyan().bold(),
                    "┐".cyan().bold()
                ),
                format!("│{:^58}│", title).cyan().bold(),
                format!("│{:^58}│", timestamp).white(),
                format!(
                    "{}{}{}",
                    "├".cyan().bold(),
                    "─".repeat(58).cyan().bold(),
                    "┤".cyan().bold()
                )
            )
        } else {
            format!(
                "{}\n{:^60}\n{:^60}\n{}",
                "=".repeat(60),
                title,
                timestamp,
                "=".repeat(60)
            )
        }
        
    }

    fn format_section(&self, title: &str, ports: &[&NetworkPort]) -> String {
        let mut output = String::new();
        
        // Section header
        if self.show_colors {
            output.push_str(&format!("│{:^58}│\n", title).green().bold());
            output.push_str(&format!("├{}┤\n", "─".repeat(58)).cyan().bold());
        } else {
            output.push_str(&format!("{:^60}\n{}\n", title, "-".repeat(60)));
        }
        
        // Table header
        if self.show_colors {
            output.push_str(&format!(
                "│ {:<8} │ {:<8} │ {:<20} │ {:<12} │\n",
                "PORT".purple().bold(),
                "PID".yellow().bold(),
                "PROCESS".white().bold(),
                "STATE".green().bold()
            ));
            output.push_str(&format!("├{}┤\n", "─".repeat(58)).cyan());
        } else {
            output.push_str(&format!(
                "| {:<8} | {:<8} | {:<20} | {:<12} |\n",
                "PORT", "PID", "PROCESS", "STATE"
            ));
            output.push_str(&format!("{}\n", "-".repeat(60)));
        }
        
        // Port data
        for port in ports {
            let port_str = if self.show_colors {
                port.port.to_string().purple()
            } else {
                port.port.to_string().normal()
            };
            
            let pid_str = if self.show_colors {
                port.display_pid().yellow()
            } else {
                port.display_pid().normal()
            };
            
            let process_str = if self.show_colors {
                port.display_process().white()
            } else {
                port.display_process().normal()
            };
            
            let state_str = if self.show_colors {
                port.state.to_string().green()
            } else {
                port.state.to_string().normal()
            };
            
            if self.show_colors {
                output.push_str(&format!(
                    "│ {:<8} │ {:<8} │ {:<20} │ {:<12} │\n",
                    port_str, pid_str, process_str, state_str
                ));
            } else {
                output.push_str(&format!(
                    "| {:<8} | {:<8} | {:<20} | {:<12} |\n",
                    port.port, port.display_pid(), port.display_process(), port.state
                ));
            }
        }
        
        if self.show_colors {
            output.push_str(&format!("├{}┤\n", "─".repeat(58)).cyan());
        } else {
            output.push_str(&format!("{}\n", "-".repeat(60)));
        }
        
        output
    }

    fn create_footer(&self) -> String {
        let tip = "💡 Tip: Run with --watch for real-time monitoring";
        let complete = "✨ Scan complete! ✨";
        
        if self.show_colors {
            format!(
                "{}\n{}\n{}",
                format!(
                    "{}{}{}",
                    "└".cyan().bold(),
                    "─".repeat(58).cyan().bold(),
                    "┘".cyan().bold()
                ),
                tip.white().bold(),
                complete.green().bold()
            )
        } else {
            format!("{}\n{}\n{}", "=".repeat(60), tip, complete)
        }
        
    }
}

impl Default for PortFormatter {
    fn default() -> Self {
        Self::new(true)
    }
}