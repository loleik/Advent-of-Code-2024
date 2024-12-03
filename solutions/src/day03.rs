use regex::Regex;

use libs::read_input::NonSplitData;

/*
    Function for finding instances of exactly mul(num1,num2), where both nums are between
    one and three digits, then multiply them and add them to the result.
*/
fn part1(input: &NonSplitData) -> i32 {
    // Regex for matching to exactly mul(num1,num2).
    let re: Regex = Regex::new(r"(mul\()(?<num1>\d{1,3})(\,)(?<num2>\d{1,3})(\))").unwrap();

    let mut result: i32 = 0;

    input
        .list
        .iter()
        // flat_map which finds each regex match in the line iteratively.
        .flat_map(|line| re.captures_iter(line))
        // for_each capture, extract num1, num2, multiply them and add to the result.
        .for_each(|caps| {
            let num1: i32 = caps["num1"].parse().expect("Failed to parse");
            let num2: i32 = caps["num2"].parse().expect("Failed to parse");

            result += num1 * num2;
        });

    result
}

/*
    Function for handling instances of mul(num1,num2), if they come after an instance of
    do(), or don't come after an instance of don't().
*/
fn part2(input: &NonSplitData) -> i32 {
    // Regex that matches either exactly mul(num1,num2), do(), or don't().
    let re: Regex =
        Regex::new(r"(don't\(\))|(do\(\))|(mul\()(?<num1>\d{1,3})(\,)(?<num2>\d{1,3})(\))")
            .unwrap();

    // Bool for tracking instances of don't().
    let mut dont = false;

    let mut result: i32 = 0;

    // This works similarly to the code for part1, with modifications for do and don't.
    input
        .list
        .iter()
        .flat_map(|line| re.captures_iter(line))
        .for_each(|caps| {
            // Check if the capture is do or don't, and adjust the don't flag accordingly.
            if let Some(matched) = caps.get(0) {
                match matched.as_str() {
                    "don't()" => dont = true,
                    "do()" => dont = false,
                    _ => (),
                }
            }

            // As long as don't is false, process mul(num1,num2) as before, otherwise skip.
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
