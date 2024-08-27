use clap::Parser;
use std::{env, fs, io};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    all: char,
}

fn main() -> io::Result<()> {
    if let Ok(x) = env::current_dir() {
        let args = Args::parse();
        let dir = fs::read_dir(x)?;

        for file in dir {
            if let Ok(x) = file {
                // " = 34 in ascii

                print!("{:?} ", x.file_name());
            }
        }
    }

    Ok(())
}
