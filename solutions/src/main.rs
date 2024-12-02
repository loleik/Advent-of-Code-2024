use clap::Parser;

use libs::read_input::parse_to_vec;
use solutions::{day01, day02};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Specific day to run code for
    #[arg(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day01::wrapper(&parse_to_vec("input/input_day01")),
        2 => day02::wrapper(&parse_to_vec("input/input_day02")),
        _ => println!("Invalid day: {}", args.day),
    }
}
