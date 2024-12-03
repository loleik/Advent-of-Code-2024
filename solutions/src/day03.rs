use regex::Regex;

use libs::read_input::NonSplitData;

fn part1(input: &NonSplitData) -> i32 {
    let re: Regex = Regex::new(r"(mul\()(?<num1>\d{1,3})(\,)(?<num2>\d{1,3})(\))").unwrap();
    let mut result: i32 = 0;

    input
        .list
        .iter()
        .flat_map(|line| re.captures_iter(line))
        .for_each(|caps| {
            let num1: i32 = caps["num1"].parse().expect("Failed to parse");
            let num2: i32 = caps["num2"].parse().expect("Failed to parse");

            result += num1 * num2;
        });

    result
}

fn part2(input: &NonSplitData) -> i32 {
    let re: Regex = Regex::new(r"(don't\(\))|(do\(\))|(mul\()(?<num1>\d{1,3})(\,)(?<num2>\d{1,3})(\))").unwrap();

    let mut dont = false;
    let mut result: i32 = 0;

    input
    .list
    .iter()
    .flat_map(|line| re.captures_iter(line))
    .for_each(|caps| {
        if let Some(matched) = caps.get(0) {
            match matched.as_str() {
                "don't()" => dont = true,
                "do()" => dont = false,
                _ => ()
            }
        }

        if !dont {
            if let (Some(num1), Some(num2)) = (caps.name("num1"), caps.name("num2")) {
                let num1_int: i32 = num1.as_str().parse().expect("Failed to parse");
                let num2_int: i32 = num2.as_str().parse().expect("Failed to parse");
            
                result += num1_int * num2_int
            }
        }
    });

    result
}

pub fn wrapper(input: &NonSplitData) -> (i32, i32) {
    let part1_result: i32 = part1(input);
    let part2_result: i32 = part2(input);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");

    (part1_result, part2_result)
}
