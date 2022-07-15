pub mod config;
pub mod qr;

use std::error::Error;
use qrcodegen::{QrCode, QrCodeEcc};
use colored::*;

pub struct Configuration {
    pub input: String,
    pub ec: QrCodeEcc,
    pub margin: u32,
    pub fgcolor: String,
    pub bgcolor: String,
    pub size: String
}

pub fn run(config: Configuration) -> Result<(), Box<dyn Error>> {
    let qr = QrCode::encode_text(config.input.as_str(), config.ec).unwrap();

    draw(&qr, config);

    Ok(())
}

fn draw(qr: &QrCode, config: Configuration) {

    let margin = config.margin as i32;


    // Loop through each other row, because we draw 2x1 pixels at a time 
    if config.size == "small" {
        for y in (-margin..qr.size() + margin).step_by(2) {
            for x in -margin..qr.size() + margin {

                // y is the current row, and therefore represents the 'top' pixel
                // y+1 is the next row, and represents the 'bottom' pixel
                let top = qr.get_module(x, y);
                let bot = qr.get_module(x, y+1);

                let c: char = match (top, bot) {
                    (true, false)  => '▀',
                    (false, true)  => '▄',
                    (false, false) => ' ',
                    (true, true)   => '█',
                };
                print!("{}", c.to_string()
                              .color(config.fgcolor.as_str())
                              .on_color(config.bgcolor.as_str()));
            }
            println!();
        }
    } else {
            for y in -margin..qr.size() + margin {
                for x in -margin..qr.size() + margin {

                    // y is the current row, and therefore represents the 'top' pixel
                    // y+1 is the next row, and represents the 'bottom' pixel
                    let px = qr.get_module(x, y);

                    let c: char = match px {
                        false  => ' ',
                        true   => '█',
                    };
                    print!("{0}{0}", c.to_string()
                                  .color(config.fgcolor.as_str())
                                  .on_color(config.bgcolor.as_str()));
                }
                println!();
        }
    }
}

