use libs::read_input::InputData;

use std::collections::HashSet;

// Struct for patterns, not really needed but eh.
#[derive(Debug)]
struct Patterns { available: HashSet<String>, test: Vec<String> }

fn parse(input: InputData) -> Patterns {
    let mut available: HashSet<String> = HashSet::new();
    let mut test: Vec<String> = Vec::new();

    // Iterate through all lines and shove them into the corresponding variable after parsing as needed.
    input.list.iter().for_each(|i| {
        if i.len() > 1 {
            available = i.into_iter()
                                .map(|s| s.replace(',', ""))
                                .collect()
        } else if i.len() == 1 {
            test.push(i[0].clone());
        }
    });

    // Return the struct.
    Patterns { available: available, test: test }
}

// Overall I struggle with dynamic programming, but viewing it as a combinatorial problem rather than a vague programming concept has actually helped.
fn part_1(patterns: &Patterns) -> Vec<bool> {
    let mut results: Vec<bool> = Vec::new();

    // Check every query string.
    for test in &patterns.test {
        let n: usize = test.len();
        let mut dp: Vec<bool> = vec![false; n + 1]; // Populate the DP vector.
        dp[0] = true; // Base case for n = 0 is true.

        // Loop through slices from the string finding combinations and matches.
        for i in 1..=n {
            for j in 0..i {
                if dp[j] && patterns.available.contains(&test[j..i].to_string()) {
                    dp[i] = true;
                    break; // Break and set dp[i] as true when we find one.
                }
            }
        }
        results.push(dp[n]); // We just need there to be one combination per query string.
    }

    results
}

// Very slightly different to part 1.
fn part_2(patterns: &Patterns) -> Vec<u64> {
    let mut results: Vec<u64> = Vec::new();

    for test in &patterns.test {
        let n: usize = test.len();
        let mut dp: Vec<u64> = vec![0; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            for j in 0..i {
                if dp[j] > 0 && patterns.available.contains(&test[j..i].to_string()) {
                    dp[i] += dp[j]; // Don't break, instead we count all combinations.
                }
            }
        }
        results.push(dp[n]);
    }

    results
}

pub fn wrapper(input: InputData) {
    let patterns: Patterns = parse(input);

    let part_1_result: Vec<bool> = part_1(&patterns);
    println!("Part 1: {}", part_1_result.iter().map(|&b| b as usize).sum::<usize>());

    let part_2_result: u64 = part_2(&patterns).iter().sum();
    println!("Part 2: {part_2_result}")
}