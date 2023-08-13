use clap::{App, Arg, SubCommand};

pub fn create_app() -> App<'static, 'static> {
    App::new("columbus")
        .version("1.0")
        .author("elliot")
        .about("A collection of tools for exploring the vast internet.")
        .subcommand(
            SubCommand::with_name("search-port") // Updated subcommand name
                .about("Scan IP addresses for open ports")
                .arg(Arg::with_name("PORT").required(true).takes_value(true)),
        ) /*
          .subcommand(
              SubCommand::with_name("search-ping") // Updated subcommand name
                  .about("Ping IP addresses")
                  .arg(Arg::with_name("IP").required(true).takes_value(true)),
          )*/
}
