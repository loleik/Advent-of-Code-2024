use libs::read_input::VecChars;

use std::collections::BTreeMap;

/*
    I will admit It feels like I've overcomplicated this a lot, and it got confusing to write, so i'll try
    to be as verbose as possible.

    Traverse function takes the position of an X, and searches its neighbours for instances of M. It then
    goes through each instance of M, and sees if XMAS is spelled in that direction.

    Used a flat vector to represent the board. Basic index formula:
    - To find (row,col): index = row * width + col

    Neighbour checking positions:
               1 2 3
               4 X 5
               6 7 8
    1. index = (row - 1) * width + (col - 1)
    2. index = (row - 1) * width + col
    3. index = (row - 1) * width + (col + 1)
    4. index = row * width + (col - 1)
    5. index = row * width + (col + 1)
    6. index = (row + 1) * width + (col - 1)
    7. index = (row + 1) * width + col
    8. index = (row + 1) * width + (col + 1)
*/
fn traverse(input: &VecChars, r: usize, c: usize) -> i32 {
    // This is a BTReeMap of closures containing all the direction formulas from the above comment.
    let mut trav: BTreeMap<&str, Box<dyn Fn((usize, usize, usize)) -> usize>> = BTreeMap::new();
    trav.insert(
        "up-left",
        Box::new(|(row, col, width)| (row - 1) * width + (col - 1)),
    );
    trav.insert("up", Box::new(|(row, col, width)| (row - 1) * width + col));
    trav.insert(
        "up-right",
        Box::new(|(row, col, width)| (row - 1) * width + (col + 1)),
    );
    trav.insert(
        "left",
        Box::new(|(row, col, width)| row * width + (col - 1)),
    );
    trav.insert(
        "right",
        Box::new(|(row, col, width)| row * width + (col + 1)),
    );
    trav.insert(
        "down_left",
        Box::new(|(row, col, width)| (row + 1) * width + (col - 1)),
    );
    trav.insert(
        "down",
        Box::new(|(row, col, width)| (row + 1) * width + col),
    );
    trav.insert(
        "down-right",
        Box::new(|(row, col, width)| (row + 1) * width + (col + 1)),
    );

    // Variables for tracking the locations and directions of 'M', and the 'XMAS' count.
    let mut matches: Vec<(usize, usize, usize)> = vec![];
    let mut directions: Vec<&str> = vec![];
    let mut count: i32 = 0;

    // Initial loop for finding 'M' instances. Goes through each direction function.
    for fun in &trav {
        // Runs the current direction function to get an index.
        let j: usize = fun.1((r, c, input.width));
        // Checks if the direction has an M.
        if input.flat_board[j] == 'M' {
            // Saves the location and direction of the M for later.
            let row: usize = j / input.width;
            let col: usize = j % input.width;
            matches.push((j, col, row));
            directions.push(fun.0);
        }
    }

    // For every instance of M, checks for if it spells XMAS in that direction.
    (0..directions.len()).for_each(|y| {
        // Grab the location information for the current M.
        let values = matches[y];
        // Check if XMAS is spelled in this direction by running the loop_as function.
        let is_xmas = loop_as(values.1, values.2, input, 0, directions[y], &trav);
        if is_xmas {
            // Increase the XMAS count if it was, if it wasn't then nothing happens.
            count += 1;
        }
    });

    // Return count, which is the number of XMAS's spelled.
    count
}

// Checks if XMAS is spelled given the start position, which is an M, and the direction.
fn loop_as(
    c: usize,
    r: usize,
    input: &VecChars,
    check: usize,
    direction: &str,
    trav: &BTreeMap<&str, Box<dyn Fn((usize, usize, usize)) -> usize>>,
) -> bool {
    // The remaining letters needed to spell XMAS.
    let xmas: Vec<char> = vec!['A', 'S'];
    // Get the index of the next character in the desired direction.
    let j: usize = trav.get(direction).unwrap()((r, c, input.width));

    // Checks if the character found in that direciton is the next character needed for XMAS.
    // If it is not, then return false. XMAS isn't spelled.
    if input.flat_board[j] == xmas[check] {
        // If it is, and it is an S, XMAS spelled, return true.
        // If it is, and it is not an S, continue.
        if xmas[check] == 'S' {
            true
        } else {
            // Saving values for next character and continuing recursively.
            let col: usize = j % input.width;
            let row: usize = j / input.width;
            loop_as(col, row, input, check + 1, direction, trav)
        }
    } else {
        false
    }
}

// Wrapper function for part 1. Loops through all characters to find X, and runs traverse when it does.
fn part1(input: &VecChars) -> i32 {
    // Number of XMAS's spelled, found by traverse.
    let mut result: i32 = 0;

    // As explained at the start, the board is represented by a flat vector.
    (0..input.flat_board.len()).for_each(|i| {
        // If X is found, save the needed values and run traverse, checking for XMAS.
        // Otherwise, just move on.
        if input.flat_board[i] == 'X' {
            let row: usize = i / input.width;
            let col: usize = i % input.width;
            result += traverse(input, row, col)
        }
    });

    // Return the XMAS count.
    result
}

/*
    Much simpler than part 1 as it's just a more restricted and simpler version of it.

    This function checks only the desired directions, then checks if the characters are correct, and
    increments the counter if they are.
*/
fn part2(input: &VecChars) -> i32 {
    // Vector of closures representing the diagonal direction formulas.
    let trav: Vec<Box<dyn Fn((usize, usize, usize)) -> usize>> = vec![
        Box::new(|(row, col, width)| (row - 1) * width + (col - 1)),
        Box::new(|(row, col, width)| (row - 1) * width + (col + 1)),
        Box::new(|(row, col, width)| (row + 1) * width + (col - 1)),
        Box::new(|(row, col, width)| (row + 1) * width + (col + 1)),
    ];

    // X-MAS count.
    let mut result: i32 = 0;

    // Loop through the flat vector looking for instances of A.
    (0..input.flat_board.len()).for_each(|i| {
        if input.flat_board[i] == 'A' {
            // If A is found, save the location information.
            let row: usize = i / input.width;
            let col: usize = i % input.width;

            // Run the direction formulas and save the information in a vector of corners.
            let corners: Vec<usize> = trav
                .iter()
                .map(|func| func((row, col, input.width)))
                .collect();

            // Turn these corner indexes into characters.
            let top_left: char = input.flat_board[corners[0]];
            let top_right: char = input.flat_board[corners[1]];
            let bot_left: char = input.flat_board[corners[2]];
            let bot_right: char = input.flat_board[corners[3]];

            /*
               If the corners fit any of the configurations, increment by 1, otherwise ignore.
               The accepted configurations are:
               M.S     M.M     S.M     S.S
               .A.     .A.     .A.     .A.
               M.S     S.S     S.M     M.M
               This could probably be done using rotation of the whole board, and I am interested to
               come back and try that. But this is easy regardless.
            */
            match (top_left, top_right, bot_left, bot_right) {
                ('M', 'S', 'M', 'S') => result += 1,
                ('M', 'M', 'S', 'S') => result += 1,
                ('S', 'M', 'S', 'M') => result += 1,
                ('S', 'S', 'M', 'M') => result += 1,
                _ => result = result,
            }
        }
    });

    // Return the X-MAS count.
    result
}

pub fn wrapper(input: &VecChars) -> (i32, i32) {
    let part1_result: i32 = part1(input);
    let part2_result: i32 = part2(input);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");

    (part1_result, part2_result)
}
