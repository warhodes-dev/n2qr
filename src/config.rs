use std::error::Error;
use clap::Parser;
use qrcodegen::QrCodeEcc;

#[derive(Parser)]
#[clap(name = "n2qr")]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(author = "Wm. A. Rhodes <warhodes@gmail.com>")]
#[clap(about = "Terminal QR code printer")]
pub struct Cli {

    #[clap(default_value_t = 1, short, long, value_parser)]
    error: u32,

    
    
}
