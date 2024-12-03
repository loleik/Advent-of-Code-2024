use clap::Parser;

use libs::read_input::{parse_to_vec, parse_to_vec_nosplit};
use solutions::{day01, day02, day03};

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
        1 => {
            day01::wrapper(&parse_to_vec("input/input_day01"));
        }
        2 => {
            day02::wrapper(&parse_to_vec("input/input_day02"));
        }
        3 => {
            day03::wrapper(&parse_to_vec_nosplit("input/input_day03"));
        }
        _ => println!("Invalid day: {}", args.day),
    }
}

#[cfg(test)]
mod tests {
    use libs::read_input::{InputData, NonSplitData};
    use solutions::day03;

    // For now I will just do basic unit testing with the example inputs.

    // Day 03
    #[test]
    fn day_03_part_1() {
        let input1: Vec<String> = vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        ];

        let input2: Vec<String> = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string(),
        ];

        let result1: i32 = day03::wrapper(&NonSplitData { list: input1 }).0;

        let result2: i32 = day03::wrapper(&NonSplitData { list: input2 }).1;

        assert_eq!(result1, 161);
        assert_eq!(result2, 48)
    }
}
