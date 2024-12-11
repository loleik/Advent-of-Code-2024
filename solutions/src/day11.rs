use std::collections::HashMap;

use libs::read_input::InputData;

// Function for counting stones.
fn count_stones(stone: i64, target: i32) -> i64 {
    let mut cache: HashMap<i64, Vec<i64>> = HashMap::new(); // Cache to avoid re-evaluating stones.
    let mut current: HashMap<i64, i64> = HashMap::new(); // Current stone.
    current.insert(stone, 1);

    // Evaluating the current stone by going through every blink/step.
    for _ in 1..=target {
        let mut next: HashMap<i64, i64> = HashMap::new(); // Next stone to evaluate.

        for (&stone, &count) in &current {
            // If the cache contains one of the current stones, use the cached value.
            if cache.contains_key(&stone) {
                for &new_stone in &cache[&stone] {
                    *next.entry(new_stone).or_insert(0) += count;
                }
            } else {
                // If the cache doesn't contain it, we create a new one and populate it.
                let mut new_stones = Vec::new();
                if stone == 0 { // First rule for the problem.
                    new_stones.push(1);
                } else if stone.to_string().len() % 2 == 0 { // Second rule for the problem..
                    let stone_str: String = stone.to_string();
                    let (left, right) = stone_str.split_at(stone_str.len() / 2);
                    new_stones.push(left.parse().unwrap());
                    new_stones.push(right.parse().unwrap());
                } else { // Final rule, i.e. rules 1 and 2 don't apply.
                    new_stones.push(stone * 2024);
                }

                // Cache the result of processing the current stone.
                cache.insert(stone, new_stones.clone());

                // Set the next stone to be processed.
                for new_stone in new_stones {
                    *next.entry(new_stone).or_insert(0) += count;
                }
            }
        }

        // Move to the next stone.
        current = next;
    }

    // Return the amount of stones.
    current.values().sum()
}

fn parts(input: InputData) -> (i64, i64) {
    // Parse the line to a vector of integers, so list of stones.
    let stones: Vec<i64> = input.list[0].iter().map(|s| s.parse().unwrap()).collect();
    let mut result: (i64, i64) = (0,0);

    // Process every stone in the list individually.
    // Could probably be made more efficient by carrying the cache over between parts 1 and 2.
    for stone in stones {
        result.0 += count_stones(stone, 25);
        result.1 += count_stones(stone, 75);
    }

    result
}

pub fn wrapper(input: InputData) {
    let results: (i64, i64) = parts(input);

    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
