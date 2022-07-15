use n2qr::{run, Configuration};
use clap::{Arg, Command};
use qrcodegen::QrCodeEcc;
use std::io;

static COLORS: &[&str] = 
    &[  "black",   "bright_black",
        "red",     "bright_red",
        "green",   "bright_green",
        "yellow",  "bright_yellow",
        "blue",    "bright_blue",
        "magenta", "bright_magenta",
        "purple",  "bright_purple", 
        "cyan",    "bright_cyan",
        "white",   "bright_white" ];

fn main() {

    let args = Command::new("qr")
                        .version("0.1.0")
                        .author("warhodes <warhodes@gmail.com>")
                        .about("Terminal QR code generator")
                        .arg(Arg::new("error")
                            .short('e')
                            .long("error")
                            .value_name("level")
                            .possible_values(&["low", "medium", "high", "veryhigh"])
                            .help("Level of error correction in encoding."))
                        .arg(Arg::new("fgcolor")
                            .short('f')
                            .long("fgcolor")
                            .value_name("color")
                            .possible_values( COLORS )
                            .default_value("white")
                            .help("Foreground color"))
                        .arg(Arg::new("bgcolor")
                            .short('b')
                            .long("bgcolor")
                            .value_name("color")
                            .possible_values( COLORS )
                            .default_value("black")
                            .help("Background color"))
                        .arg(Arg::new("margin")
                            .short('m')
                            .long("margin")
                            .value_name("margin")
                            .default_value("4")
                            .help("Margin around output QR code"))
                        .arg(Arg::new("size")
                            .short('s')
                            .long("size")
                            .value_name("size")
                            .possible_values( &["big", "small"] )
                            .help("Size of QR code render (pixels per glyph)"))
                        .arg(Arg::new("input")
                            .index(1)
                            .value_name("input")
                            .help("String to convert to QR. Leave empty to read from stdin"))
                        .get_matches();

    let ec: QrCodeEcc = match args.value_of("error") {
        Some("low")      => QrCodeEcc::Low,
        Some("medium")   => QrCodeEcc::Medium,
        Some("high")     => QrCodeEcc::Quartile,
        Some("veryhigh") => QrCodeEcc::High,
        _                => QrCodeEcc::Medium,
    };

    let input: String = match args.value_of("input") {
        Some(input) => input.to_string(),
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            buffer
        }
    };

    let margin: u32 = match args.value_of("margin") {
        Some(i) => i.parse::<u32>().unwrap(),
        None => 0
    };

    let fgcolor: String = match args.value_of("fgcolor") {
        Some(c) => c.to_string().replace('_', " "),
        None => "white".to_string(),
    };

    let bgcolor: String = match args.value_of("bgcolor") {
        Some(c) => c.to_string().replace('_', " "),
        None => "black".to_string(),
    };
    
    let size: String = match args.value_of("size") {
        Some(s) => s.to_string(),
        None => "small".to_string(),
    };

    let config = Configuration { ec, input, margin, fgcolor, bgcolor, size };

    if let Err(e) = run(config) {
        println!("Error: {:?}", e);
    }
}

