use libs::read_input::InputData;

use std::collections::HashMap;


// I was trying to optimize this for part 2 but I've confused myself and can't think
fn count_stones(stone: i64, target: i32) -> i32 {
    let cache: HashMap<i64, Vec<i64>> = HashMap::new();

    fn recursion(stones: Vec<i64>, mut step: i32, target: i32, mut cache: HashMap<i64, Vec<i64>>) -> Vec<i64> {
        let mut new_stones: Vec<i64> = Vec::new();

        for stone in stones {
            let zero: bool = stone == 0;
            let digits: bool = stone.to_string().len() % 2 == 0;

            if cache.contains_key(&stone) {
                for x in cache.get(&stone).unwrap() {
                    new_stones.push(*x);
                }
            } else if zero {
                new_stones.push(1);
            } else if digits {
                let stone_str: String = stone.to_string();
                let (left, right) = stone_str.split_at(stone_str.len() / 2);
                new_stones.push(left.parse().unwrap());
                new_stones.push(right.parse().unwrap());
                cache.insert(stone, vec![left.parse().unwrap(), right.parse().unwrap()]);
            } else {
                new_stones.push(stone * 2024);
                cache.insert(stone, vec![stone * 2024]);
            }
        }

        step += 1;

        if step > target {
            return new_stones
        } else {
            recursion(new_stones, step, target, cache)
        }
    }

    recursion(vec![stone], 1, target, cache).len() as i32
}

fn part1(input: InputData) -> i32 {
    let stones: Vec<i64> = input.list[0].iter().map(|s| s.parse().unwrap()).collect();
    let mut result = 0;

    for stone in stones {
        result += count_stones(stone, 75);
    }

    result
}

pub fn wrapper(input: InputData) {
    let part1_result: i32 = part1(input);

    println!("Part 1: {part1_result}");
}
