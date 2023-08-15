use crate::color_printer;
use rusqlite::{Connection, Result};
use std::net::Ipv4Addr;
use termcolor::Color;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("search-results.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ips (ip TEXT PRIMARY KEY, port INTEGER)",
        [],
    )?;
    Ok(conn)
}

pub fn query_ips() -> Result<Vec<(String, u16)>, rusqlite::Error> {
    let conn = init_db()?;
    let mut stmt = conn.prepare("SELECT ip, port FROM ips")?;
    let rows = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;
    let mut ips = Vec::new();
    for row in rows {
        ips.push(row?);
    }
    Ok(ips)
}

pub fn query_and_output_ips() {
    match query_ips() {
        Ok(ips) => {
            for (ip, port) in ips {
                color_printer::print_colored(&format!("Port {} is open ", port), &ip, Color::Green);
            }
        }
        Err(err) => {
            eprintln!("Error querying and outputting IPs: {}", err);
        }
    }
}

pub fn insert_ip_port(ip: &Ipv4Addr, port: u16) -> Result<(), rusqlite::Error> {
    let conn = init_db()?;
    conn.execute(
        "INSERT OR IGNORE INTO ips (ip, port) VALUES (?, ?)",
        [ip.to_string(), port.to_string()],
    )?;
    Ok(())
}
