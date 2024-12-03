use std::cmp;
use std::collections::HashMap;

use libs::read_input::InputData;

// Turns the input structure into two vectors of integers, sorted into ascending order.
fn handle_input(input: &InputData) -> (Vec<i32>, Vec<i32>) {
    // Create the left and right vectors
    let mut left_data: Vec<i32> = vec![];
    let mut right_data: Vec<i32> = vec![];

    // Loop over the input and parse the strings to integers. The lines are already split at whitespace.
    for line in &input.list {
        left_data.push(line[0].parse().expect("Failed to parse string"));
        right_data.push(line[1].parse().expect("Failed to parse string"));
    }

    // Sort into ascending order.
    left_data.sort();
    right_data.sort();

    // Output both vectors in a tuple.
    (left_data, right_data)
}

// Works out distances between pairs of elements in asvending order using recursion.
fn part1(lists: &(Vec<i32>, Vec<i32>), cumulative_distance: i32, index: usize) -> i32 {
    // Find the difference by subtracting the smaller value from the larger value.
    let current: i32 =
        cmp::max(lists.0[index], lists.1[index]) - cmp::min(lists.0[index], lists.1[index]);

    // Stop and return if this is the last pair, continue otherwise, incrementing the index.
    if index == lists.0.len() - 1 {
        cumulative_distance + current
    } else {
        part1(lists, cumulative_distance + current, index + 1)
    }
}

// Checks how many times a given value occurs in a given list. Returns the count.
fn count_repetition(target: i32, list: &[i32]) -> i32 {
    let mut counter: i32 = 0;

    for x in list {
        if *x == target {
            counter += 1;
        }
    }

    counter
}

/* Loops through list 1, and checks how many times each value occurs in list 2. Then cumulativley adds
up the value * number of occurances. */
fn part2(lists: &(Vec<i32>, Vec<i32>)) -> i32 {
    // Define a hashmap for caching purposes, so we don't repeat calculations.
    let mut count_cache: HashMap<i32, i32> = HashMap::new();
    let mut cumulative_similarity: i32 = 0;

    for x in &lists.0 {
        // Check whether we've already ran through a value, if we have just take the result from the cache.
        if count_cache.contains_key(x) {
            let temp_count: i32 = *count_cache.get(x).unwrap();

            cumulative_similarity += x * temp_count;
        } else {
            let temp_count: i32 = count_repetition(*x, &lists.1);

            count_cache.insert(*x, temp_count);
            cumulative_similarity += x * temp_count;
        }
    }

    cumulative_similarity
}

// This just brings everything together and prints the answers.
pub fn wrapper(input: &InputData) -> (i32, i32) {
    let lists: (Vec<i32>, Vec<i32>) = handle_input(input);
    let part1_result: i32 = part1(&lists, 0, 0);
    let part2_result: i32 = part2(&lists);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");

    (part1_result, part2_result)
}
