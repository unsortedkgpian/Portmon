// CLI entry point - this orchestrates everything

use anyhow::Result;
use clap::{Arg, Command};
use std::{thread, time::Duration};

mod models;
mod scanner;
mod display;

use scanner::PortScanner;
use display::PortFormatter;

fn main() -> Result<()> {
    let matches = Command::new("portmon")
        .version("0.1.0")
        .author("Your Name")
        .about("A beautiful network port monitor with decorative output")
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Disable colored output")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .help("Watch mode - refresh every N seconds")
                .value_name("SECONDS")
                .value_parser(clap::value_parser!(u64))
        )
        .arg(
            Arg::new("tcp-only")
                .long("tcp")
                .help("Show only TCP ports")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("udp-only")
                .long("udp")
                .help("Show only UDP ports")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    let show_colors = !matches.get_flag("no-color");
    let tcp_only = matches.get_flag("tcp-only");
    let udp_only = matches.get_flag("udp-only");
    
    let formatter = PortFormatter::new(show_colors);
    let mut scanner = PortScanner::new();

    if let Some(interval) = matches.get_one::<u64>("watch") {
        // Watch mode
        watch_ports(&mut scanner, &formatter, *interval, tcp_only, udp_only)?;
    } else {
        // Single scan
        scan_once(&mut scanner, &formatter, tcp_only, udp_only)?;
    }

    Ok(())
}

fn scan_once(
    scanner: &mut PortScanner, 
    formatter: &PortFormatter,
    tcp_only: bool,
    udp_only: bool
) -> Result<()> {
    let mut ports = scanner.scan_all_ports()?;
    
    // Filter by protocol if requested
    if tcp_only {
        ports.retain(|p| matches!(p.protocol, models::ProtocolType::Tcp));
    } else if udp_only {
        ports.retain(|p| matches!(p.protocol, models::ProtocolType::Udp));
    }
    
    let output = formatter.format_ports(&ports);
    println!("{}", output);
    
    Ok(())
}

fn watch_ports(
    scanner: &mut PortScanner,
    formatter: &PortFormatter,
    interval: u64,
    tcp_only: bool,
    udp_only: bool
) -> Result<()> {
    loop {
        // Clear screen (ANSI escape code)
        print!("\x1B[2J\x1B[1;1H");
        
        scan_once(scanner, formatter, tcp_only, udp_only)?;
        
        println!("\nRefreshing in {} seconds... (Ctrl+C to exit)", interval);
        thread::sleep(Duration::from_secs(interval));
    }
}

// Module declarations for the library
pub use models::*;
pub use scanner::*;
pub use display::*;
