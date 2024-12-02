use std::cmp::Ordering;

use libs::read_input::InputData;

// Parses numbers, stored in strings, to an i32 and returns the refined data.
fn refine_input(input: &InputData) -> Vec<Vec<i32>> {
    let reports: Vec<Vec<i32>> = input
        .list
        .iter()
        .map(|line| {
            line.iter()
                .map(|level| level.parse().expect("Failed to parse level"))
                .collect()
        })
        .collect();

    reports
}
// Function for checking the safety of a given report.
fn check_report(report: &[i32]) -> (bool, Vec<usize>) {
    // Booleans for condition checking
    let mut ascending: bool = false;
    let mut descending: bool = false;
    let mut in_range: bool = true;
    let mut errors: Vec<usize> = vec![];

    // Loop through each value, indexing so that the next index is occupied.
    (0..report.len() - 1).for_each(|x| {
        // Calculate the difference between the current value and next value.
        let diff: i32 = report[x] - report[x + 1];

        match diff.cmp(&0) {
            Ordering::Less => ascending = true,
            Ordering::Greater => descending = true,
            Ordering::Equal => ()
        };

        // Mark the error location for later.
        if ascending && descending {
            errors.push(x);
            errors.push(x + 1);
            if x == 1 {
                errors.push(0);
            }
        };

        // Check if the difference is within acceptable bounderies.
        if !(1 <= diff.abs() && diff.abs() <= 3) {
            in_range = false;
            // Mark the error location for later.
            errors.push(x + 1);
            errors.push(x);
        }
    });

    match (ascending, descending, in_range) {
        // Both ascended and descended, did neither, or unacceptable difference, unsafe.
        (true, true, _) | (false, false, _) | (_, _, false) => (false, errors),
        // All conditions met, safe.
        _ => (true, errors),
    }
}

/*
   Takes a list of reports, and returns how many of them are safe. Conditions:
   - Levels must be strictly increasing or decreasing
   - Adjascent levels must differ by at least 1, and at most 3
   I will admit the logic for part 2 got messy, I may come back to it in the future.
*/
fn safety(reports: Vec<Vec<i32>>) -> (i32, i32) {
    // Solution to part 1.
    let mut safe_1: i32 = 0;
    // Give solution to part 2 when added to safe_1.
    let mut safe_2: i32 = 0;

    for report in reports {
        let result: (bool, Vec<usize>) = check_report(&report);
        if result.0  {
            // If the report is safe, increment by 1.
            safe_1 += 1;
        } else {
            // Try removing each error one at a time.
            for x in &result.1 {
                // Create a new vector with the current error value dropped.
                let mut unsafe_report: Vec<i32> = report.clone();
                unsafe_report.remove(*x);

                // If the report is now safe, increment safe_2 and break.
                if check_report(&unsafe_report).0 {
                    safe_2 += 1;
                    break;
                }
            }
        }
    }

    (safe_1, safe_1 + safe_2)
}

pub fn wrapper(input: &InputData) {
    let reports: Vec<Vec<i32>> = refine_input(input);
    let results: (i32, i32) = safety(reports);
    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}
