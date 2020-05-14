use std::io::prelude::*;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let args = Opt::from_args();
    if args.line_number == 0 {
        eprintln!("line number must be > 0");
        std::process::exit(1);
    }

    let file: Box<dyn Read> = match args.input {
        None => Box::new(std::io::stdin()),
        Some(path) => Box::new(std::fs::File::open(path)?),
    };

    let f = std::io::BufReader::new(file);

    for (number, line) in f.lines().enumerate() {
        if number >= args.line_number {
            println!("{}", line.unwrap());
            break;
        }
    }

    Ok(())
}

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pick", about = "Pick a specific line from stdin or a file")]
struct Opt {
    #[structopt()]
    line_number: usize,

    /// Optional Input file
    #[structopt(short = "f", long = "file", parse(from_os_str))]
    input: Option<PathBuf>,
}
