#[macro_use]
extern crate clap;
#[macro_use]
extern crate nom;

use clap::{App};
use std::{fmt, process};
use std::str::FromStr;

const DEFAULT_WIDTH: u8 = 10;
const DEFAULT_HEIGHT: u8 = 5;

#[derive(Debug)]
pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

impl fmt::Display for Color {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(width) = formatter.width() {
            write!(formatter, "\x1b[48;2;{};{};{}m{}\x1b[0m", self.red, self.green, self.blue, " ".repeat(width))
        } else {
            write!(formatter, "{:1}", self)
        }
    }
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}

named!(hex2<&str, u8>,
    map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
);

named!(hex_color<&str, Color>,
  do_parse!(
    opt!(tag!("#")) >>
    red:   hex2 >>
    green: hex2 >>
    blue:  hex2 >>
    (Color { red, green, blue })
  )
);

fn preview_color(color: &Color, width: u8, height: u8) {
    for _ in 0..height {
        println!("{:width$}", color, width = usize::from(width))
    }
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let app = App::from_yaml(yaml);
    let matches = app.get_matches();

    let width = matches.value_of("width").and_then(|w| {
        let result = u8::from_str(w);
        if result.is_err() {
            eprintln!("Invalid value for width: {}\nDefaulting to the default width", w)
        }
        result.ok()
    }).unwrap_or(DEFAULT_WIDTH);
    let height = matches.value_of("height").and_then(|h| {
        let result = u8::from_str(h);
        if result.is_err() {
            eprintln!("invalid value for height: {}\nDefaulting to the default height", h)
        }
        result.ok()
    }).unwrap_or(DEFAULT_HEIGHT);

    if let Some(hex) = matches.value_of("hex") {
        match hex_color(hex) {
            Ok((_, color)) => {
                preview_color(&color, width, height);
                process::exit(0);
            },
            Err(_) => {
                eprintln!("Invalid value for hex: {}", hex);
                process::exit(1);
            },
        }
    }

    if let (Some(red), Some(green), Some(blue)) = (matches.value_of("red"), matches.value_of("green"), matches.value_of("blue")) {
        let r = u8::from_str(red).unwrap_or_else(|_| {
            eprintln!("Invalid value for red: {}", red);
            process::exit(1)
        });
        let g = u8::from_str(green).unwrap_or_else(|_| {
            eprintln!("Invalid value for green: {}", green);
            process::exit(1)
        });
        let b = u8::from_str(blue).unwrap_or_else(|_| {
            eprintln!("Invalid value for blue: {}", blue);
            process::exit(1)
        });

        let color = Color { red: r, green: g, blue: b };

        preview_color(&color, width, height);
        process::exit(0);
    }

    eprintln!("`farbe --help` to show usage");
    process::exit(1);
}
