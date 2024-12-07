use libs::read_input::InputData;

// Today was suprisingly easy. I didn't even have to optimize like I thought I might.
// The only issue was flying past i32 limits.

// Struct for storing fully refined and parsed input.
struct Refined {
    results: Vec<i64>,
    inputs: Vec<Vec<i64>>
}

// Funciton for initializing new sets of refined results.
impl Refined {
    fn new(r: Vec<i64>, i: Vec<Vec<i64>>) -> Self {
        Refined { results: r, inputs: i }
    }
}

// This took longer than the actual task funnily enough.
fn refine_input(input: InputData) -> Refined {
    // Vectors for results and sets of input respectively.
    let mut r: Vec<i64> = Vec::new();
    let mut i: Vec<Vec<i64>> = Vec::new();

    // Map the values within the full input list to the vectors.
    input.list
            .iter()
            .for_each(|x| {
                // The first value is the result for the line.
                // Parse it as an integer and push to r.
                r.push(x[0]
                    .split(':')
                    .collect::<Vec<_>>()[0]
                    .parse()
                    .expect("Failed to parse as int.")
                );
                // The rest of the values are the results for that line.
                // Parse to integers, collect as a Vec<i64>, and push to i.
                i.push(x[1..]
                    .iter()
                    .map(|y|
                        y.parse()
                         .expect("Failed to parse as int.")
                    ).collect()
                );
            });
    
    // Return the refined struct.
    Refined::new(r, i)
}

// Exhaustive depth first search.
// Could use extra conditions to terminate early if for example cv > ev, to prune the search.
// But no extra optimizations were needed for a reasonable run time.
fn backtracking(ev: i64, cv: i64, inputs: &Vec<i64>, i: usize, part2: bool) -> bool {
    // If we've reached the end of the list, there are no more calculations to check.
    if i == inputs.len() {
        return cv == ev
    }

    // Check if inserting a '+' gets to the expected value.
    let added: i64 = cv + inputs[i];
    if backtracking(ev, added, inputs, i + 1, part2) {
        return true
    }

    // Check if inserting a '*' gets to the expected value.
    let multiplied: i64 = cv * inputs[i];
    if backtracking(ev, multiplied, inputs, i + 1, part2) {
        return true
    }

    // If we're doing part 2, check concatenation '||'.
    if part2 {
        let concat: i64 = format!("{}{}", cv.to_string(), inputs[i].to_string())
            .parse().expect("Failed to parse as int.");
        if backtracking(ev, concat, inputs, i + 1, part2) {
            return true
        }
    }

    false
}

// Wrapper for part 1.
fn part1(refined: &Refined) -> i64 {
    let mut result: i64 = 0; // Sum of successful lines.

    // Loop through all inputs and see if a configuration is found.
    for x in 0..refined.results.len() {
        if backtracking(refined.results[x], 0, &refined.inputs[x], 0, false) {
            // Add to the sum if one is found.
            result += refined.results[x];
        }
    }

    // Return the sum.
    result
}

// Wrapper function for part 2.
// Code is the same as part 1 except with the flag to check concatenation.
// Probably no need for a separate function.
fn part2(refined: Refined) -> i64 {
    let mut result: i64 = 0;

    for x in 0..refined.results.len() {
        if backtracking(refined.results[x], 0, &refined.inputs[x], 0, true) {
            result += refined.results[x];
        }
    }

    result
}

pub fn wrapper(input: InputData) {
    let refined = refine_input(input);

    let part1_result: i64 = part1(&refined);
    let part2_result: i64 = part2(refined);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}