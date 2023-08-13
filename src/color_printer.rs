use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn print_colored(prefix: &str, value: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)))
        .unwrap();
    write!(&mut stdout, "{}", prefix).unwrap();
    stdout.reset().unwrap();
    writeln!(&mut stdout, "{}", value).unwrap();
}
