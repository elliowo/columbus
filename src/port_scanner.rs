use crate::color_printer;
use crate::database::db;
use crate::random_ip::generate_random_ipv4;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

const TIMEOUT_MS: u64 = 50;

pub fn scan_and_update_database(
    port_start: u16,
    port_end: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let timeout = Duration::from_millis(TIMEOUT_MS);

    let ips = db::query_ips()?; // Retrieve stored IP addresses from the database

    for (ip_str, _port) in ips {
        let ip_addr: IpAddr = ip_str.parse()?;

        for port in port_start..=port_end {
            scan_single_ip(&ip_addr.to_string(), port, timeout)?;
        }
    }

    Ok(())
}

pub fn scan_ports(port: u16) {
    let timeout = Duration::from_millis(TIMEOUT_MS);

    loop {
        let ip_addr: Ipv4Addr = generate_random_ipv4();
        scan_single_ip(&ip_addr.to_string(), port, timeout).unwrap_or_else(|err| {
            eprintln!("Error scanning IP and port: {}", err);
        });
    }
}

fn scan_single_ip(
    ip_str: &str,
    port: u16,
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let ip_addr: Ipv4Addr = ip_str.parse()?;
    let socket_addr = SocketAddr::new(IpAddr::V4(ip_addr), port);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => {
            color_printer::print_colored(
                &format!("Port {} is open on ", port),
                ip_str,
                termcolor::Color::Green,
            );

            db::insert_ip_port(&ip_addr, port)?;
        }
        Err(_) => {} // no port found
    }

    Ok(())
}
