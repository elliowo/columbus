mod clap_setup;
mod color_printer;
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
