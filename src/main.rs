extern crate getopts;

use std::fs::File;
use std::io::{BufReader, BufRead, Write};

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("s", "skip", "Skipped line number", "FILENAME");
    opts.optopt("n", "num", "Total line number", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");

    let matches = opts.parse(std::env::args().skip(1)).unwrap();

    let skip = matches.opt_str("s").and_then(|s| s.parse().ok()).unwrap_or(usize::MIN);
    let count = matches.opt_str("n").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);

    let reader: Box<dyn BufRead> = if !matches.free.is_empty() {
        match File::open(matches.free[0].clone()) {
            Ok(f) => Box::new(BufReader::new(f)),
            _ => panic!("Can not open file: {}", matches.free[0]),
        }
    } else {
        Box::new(std::io::stdin().lock())
    };

    let lines = reader.split(b'\n').skip(skip).take(count)
        .map(|l| {
            String::from_utf8_lossy(&l.unwrap()).to_string()
        });

    match matches.opt_str("o").and_then(|output| File::create(output).ok()) {
        Some(mut out) => {
                for line in lines {
                    writeln!(out, "{}", line).expect("Can not write file");
                }
        },
        _ => {
            for line in lines {
                println!("{}", line);
            }
        },
    }
}
