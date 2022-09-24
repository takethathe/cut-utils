extern crate getopts;

use getopts::Options;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("s", "skip", "Skipped line number", "number");
    opts.optopt("n", "num", "Total line number", "number");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optflag("h", "help", "Usage information");

    let args: Vec<String> = std::env::args().collect();
    let matches = opts.parse(&args[1..]).unwrap();

    if matches.opt_present("h") {
        print_usage(&args[0].as_str(), &opts);
        return;
    }

    let skip = matches
        .opt_str("s")
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MIN);
    let count = matches
        .opt_str("n")
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);

    let reader: Box<dyn BufRead> = if !matches.free.is_empty() {
        match File::open(matches.free[0].clone()) {
            Ok(f) => Box::new(BufReader::new(f)),
            _ => panic!("Can not open file: {}", matches.free[0]),
        }
    } else {
        Box::new(std::io::stdin().lock())
    };

    const WRITE_FILE_ERROR: &str = "Write output file with error.";
    const NEW_LINE: &[u8] = b"\n";
    let lines = reader.split(NEW_LINE[0]).skip(skip).take(count);
    match matches
        .opt_str("o")
        .and_then(|file| File::create(file).ok())
    {
        Some(ref mut out) => {
            out.write_all(
                &lines
                    .filter_map(|l| l.ok())
                    .collect::<Vec<Vec<u8>>>()
                    .join(NEW_LINE),
            )
            .expect(WRITE_FILE_ERROR);
        }
        _ => {
            for line in lines {
                if let Ok(ref l) = line {
                    println!("{}", String::from_utf8_lossy(l))
                }
            }
        }
    }
}
