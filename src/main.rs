use database::db;

mod clap_setup;
mod color_printer;
mod database;
mod port_scanner;
mod random_ip;

fn main() -> std::io::Result<()> {
    let matches = clap_setup::create_app().get_matches();

    if let Some(matches) = matches.subcommand_matches("search-port") {
        let port = matches
            .value_of("PORT")
            .unwrap_or("25565")
            .parse::<u16>()
            .unwrap();
        port_scanner::scan_ports(port);
    } else if let Some(_matches) = matches.subcommand_matches("update-database") {
        let port_start = 1;
        let port_end = 100;

        port_scanner::scan_and_update_database(port_start, port_end);
    } else if let Some(_matches) = matches.subcommand_matches("search-results") {
        db::query_and_output_ips();
    }
    /*else if let Some(matches) = matches.subcommand_matches("ping") {
              let ip = matches.value_of("IP").unwrap();
              ping_ip(ip);
    }*/
    else {
        println!("please use the --help or -h command");
    }
    Ok(())
}
