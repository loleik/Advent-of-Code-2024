use libs::read_input::InputData;

// Printing to get an idea of how horrific this runtime would be haha.
fn count_stones(stone: i64, target: i32) -> i32 {
    fn recursion(stones: Vec<i64>, mut step: i32, target: i32) -> Vec<i64> {
        print!("\x1B[2J\x1B[1;1H");
        println!("Blinks: {step}");

        let mut new_stones: Vec<i64> = Vec::new();

        for stone in stones {
            let zero: bool = stone == 0;
            let digits: bool = stone.to_string().len() % 2 == 0;

            if zero {
                new_stones.push(1);
            } else if digits {
                let stone_str: String = stone.to_string();
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }

        step += 1;

        if step > target {
            return new_stones
        } else {
            recursion(new_stones, step, target)
        }
    }

    recursion(vec![stone], 1, target).len() as i32
}

fn parts(input: InputData) -> (i32, i32) {
    let stones: Vec<i64> = input.list[0].iter().map(|s| s.parse().unwrap()).collect();
    let mut result: (i32, i32) = (0,0);

    for stone in stones {
        result.0 += count_stones(stone, 25);
        result.1 += count_stones(stone, 75);
    }

    result
}

pub fn wrapper(input: InputData) {
    let results: (i32, i32) = parts(input);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
