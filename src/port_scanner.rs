use crate::color_printer;
use crate::random_ip::generate_random_ipv4;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

pub fn scan_ports(port: u16) {
    const TIMEOUT_MS: u64 = 50;
    let timeout = Duration::from_millis(TIMEOUT_MS);

    loop {
        let ip_addr: Ipv4Addr = generate_random_ipv4();
        let socket_addr = SocketAddr::new(IpAddr::V4(ip_addr), port);

        match TcpStream::connect_timeout(&socket_addr, timeout) {
            Ok(_) => {
                color_printer::print_colored(
                    &format!("Port {} is open on ", port),
                    &ip_addr.to_string(),
                    termcolor::Color::Green,
                );
            }
            Err(_) => {
                // color_printer::print_colored(&format!("Port {} is closed on ", port), &ip_addr.to_string(), termcolor::Color::Red);
            }
        }
    }
}
