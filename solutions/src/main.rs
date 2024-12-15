use clap::Parser;

use libs::read_input::{
    parse_to_vec, parse_to_vec_chars, parse_to_vec_chars_2, parse_to_vec_nosplit,
};
use solutions::{day01, day02, day03, day04, day05, 
                day06, day07, day08, day09, day10, 
                day11, day12, day13, day14, day15};

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
        4 => {
            day04::wrapper(&parse_to_vec_chars("input/input_day04"));
        }
        5 => {
            day05::wrapper(&parse_to_vec("input/input_day05"));
        }
        6 => {
            day06::wrapper(parse_to_vec_chars("input/input_day06"));
        }
        7 => {
            day07::wrapper(parse_to_vec("input/input_day07"));
        }
        8 => {
            day08::wrapper(parse_to_vec_chars_2("input/input_day08"));
        }
        9 => {
            day09::wrapper(parse_to_vec_chars_2("input/input_day09"));
        }
        10 => {
            day10::wrapper(parse_to_vec_chars_2("input/input_day10"));
        }
        11 => {
            day11::wrapper(parse_to_vec("input/input_day11"));
        }
        12 => {
            day12::wrapper(parse_to_vec_chars_2("input/input_day12"));
        }
        13 => {
            day13::wrapper(parse_to_vec("input/input_day13"));
        }
        14 => {
            day14::wrapper(parse_to_vec("input/input_day14"))
        }
        15 => {
            day15::wrapper(parse_to_vec("input/input_day15"))
        }
        _ => println!("Invalid day: {}", args.day),
    }
}

#[cfg(test)]
mod tests {
    use libs::read_input::NonSplitData;
    use solutions::day03;

    // For now I will just do basic unit testing with the example inputs.

    // To do: Unit tests for day 01 and 02

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
