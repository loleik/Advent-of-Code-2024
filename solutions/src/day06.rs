use std::collections::{HashMap, HashSet};

use libs::read_input::VecChars;

// Struct for positions with directions. Used for obstructions and start points.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    row: usize, // row on the grid
    col: usize, // collumn on the grid
    index: usize, // index within the vector
    direction: char, // direciton if it's a start point
}

impl Position {
    // Defines a new start point. Includes direction.
    fn new_start(r: usize, c: usize, i: usize, d: char) -> Self {
        Position {
            row: r, col: c, index: i, direction: d
        }
    }

    // Defines a new obstruction. No direction, so set it to null.
    fn new_obst(r: usize, c: usize, i: usize) -> Self {
        Position {
            row: r, col: c, index: i, direction: '\0'
        }
    }

    // Default for initializing variables. Contents don't matter.
    fn default() -> Self {
        Position{
            row: 0, col: 0, index: 0, direction: '\0'
        }
    }
}

// Function to find obstruction positions and start position from input.
fn find_info(input: &VecChars) -> (HashMap<usize, Position>, Position) {
    // Hashmap for storing positions with their keys.
    let mut obst: HashMap<usize, Position> = HashMap::new();
    // The start position of the board.
    let mut start: Position = Position::default();

    // Loop through the map looking for important characters.
    (0..input.flat_board.len()).for_each(|c| {
        // # = obstruction, ^,<,>,v = start point.
        match input.flat_board[c] {
            '#' => {
                // Add the obstruction data to the hashmap.
                obst.insert(c, Position::new_obst(
                    c / input.width, c % input.width, c
                ));
            }
            '^' => {
                // Set the start point.
                start = Position::new_start(
                    c / input.width, c % input.width, c, 'u'
                );
            }
            '<' => {
                // Set the start point.
                start = Position::new_start(
                    c / input.width, c % input.width, c, 'l'
                );
            }
            '>' => {
                // Set the start point.
                start = Position::new_start(
                    c / input.width, c % input.width, c, 'r'
                );
            }
            'v' => {
                // Set the start point.
                start = Position::new_start(
                    c / input.width, c % input.width, c, 'd'
                );
            }
            _ => {}
        }
    });

    // Return both.
    (obst, start)
}

// Function for traveling along one direction to an obstruction or border.
fn travel(
    input: &VecChars, 
    obst: &HashMap<usize, Position>, 
    start: &Position,
    cache: &HashSet<usize>,
    starts: &Vec<Position>
) -> (Position, bool, bool, HashSet<usize>, Vec<Position>) {
    /*
    Not proud of this code at all. The idea started out good I think but it got
    too complicated for me.

    We need to take the start position and direction, and see if there are any
    obstructions in that direction. Take (row=10,col=5) and 'u':
    - There are 10 positions to check as there are 10 rows above.
    - Each of these positions will have col=5.
    - index = row * width + col = (0..=9) * width + 5
    - So write a loop to calculate these indexes, and check them in the hashmap.
    */

    // This will store the new starting point.
    let mut new_start: Position = Position::default();
    // Checks if a boundary was hit.
    let mut boundary: bool = false;
    // Checks if a loop occurs.
    let mut looop: bool = false;
    // Copy of the cache and starts, very inefficient.
    let mut new_cache: HashSet<usize> = cache.clone();
    let mut new_starts: Vec<Position> = starts.clone();

    // Check what direciton the guard is traveling.
    // Lots of repeated logic. I will explain it all once.
    match start.direction {
        'u' => {
            // Add the current start to the cache.
            new_cache.insert(start.index);
            // Loop through the collumn from the start position moving upwards.
            for y in (0..start.row).rev() {
                // Set the current index using index = row * width + col.
                let current = y * input.width + start.col;
                // If the current index is a known obstruction, we stop searching.
                if obst.contains_key(&current) {
                    // Set the new start point for the next loop.
                    new_start = Position::new_start(
                        y + 1, start.col, (y + 1) * input.width + start.col, 'r'
                    );
                    break;
                } else if input.flat_board[current] == '0' {
                    // If we hit a null character, that's a boundary, break.
                    boundary = true;
                    break;
                }
                // If nothing happened, add current to cache.
                new_cache.insert(current);
            };
        }
        'd' => {
            new_cache.insert(start.index);
            // Loop through the collumn from the start position moving downwards.
            for y in start.row..input.height {
                let current = y * input.width + start.col;
                if obst.contains_key(&current) {
                    new_start = Position::new_start(
                        y - 1, start.col, (y - 1) * input.width + start.col, 'l'
                    );
                    break;
                } else if input.flat_board[current] == '0' {
                    boundary = true;
                    break;
                }
                new_cache.insert(current);
            };
        }
        'l' => {
            new_cache.insert(start.index);
            // Loop through the current row from the starting point moving left.
            for x in (0..start.col).rev() {
                let current = start.row * input.width + x;
                if obst.contains_key(&current) {
                    new_start = Position::new_start(
                        start.row, x + 1, start.row * input.width + (x + 1), 'u'
                    );
                    break;
                } else if input.flat_board[current] == '0' {
                    boundary = true;
                    break;
                }
                new_cache.insert(current);
            };
        }
        'r' => {
            new_cache.insert(start.index);
            // Loop through the current row from the starting point, moving right.
            for x in start.col..input.width {
                let current = start.row * input.width + x;
                if obst.contains_key(&current) {
                    new_start = Position::new_start(
                        start.row, x - 1, start.row * input.width + (x - 1), 'd'
                    );
                    break;
                } else if input.flat_board[current] == '0' {
                    boundary = true;
                    break;
                }
                new_cache.insert(current);
            };
        }
        _ => { boundary = true; }
    }

    // Check if EXACTLY new_start is a previous starting point.
    // Previously I was only checking the index, which doesn't work, direciton is important.
    if starts.contains(&new_start) {
        // If it is, we've looped.
        looop = true;
    } else if new_start.direction != '\0' {
        // Push it into the starts list if it wasn't there and isn't an end point.
        new_starts.push(new_start);
    }

    // Return the required values.
   (new_start, boundary, looop, new_cache, new_starts)
}

// Function for part 1. It runs travel until we hit a boundary.
fn part1(input: &VecChars, obst: &HashMap<usize, Position> ,start: Position) -> (i32, bool, HashSet<usize>) {
    // Sets a mutable start point to update in the loop.
    let mut mut_start: Position = start;
    // Initialize the cache and starts vector.
    // HashSet was used for cache due to built in uniqueness of values.
    let mut cache: HashSet<usize> = HashSet::new();
    let mut starts: Vec<Position> = Vec::new();
    // Variable for loop checking, used in part 2.
    let mut looop: bool = false;

    // Add the initial start to the vector of starting points.
    starts.push(start);

    loop {
        // Run travel with the current values and store all outputs.
        let out: (Position, bool, bool, HashSet<usize>, Vec<Position>) = travel(&input, &obst, &mut_start, &cache, &starts);

        // Update relevant variables.
        mut_start = out.0; // New starting point.
        cache = out.3; // Updated cache.
        starts = out.4; // Updated starting point list.

        // Exit conditions.
        if out.1 {
            break; // If we hit a boundary, exit.
        } else if out.2 {
            looop = true; // Loop tracking for part 2.
            break; // If we loop, exit.
        }
    }

    // The cache length is the number of unique nodes visited.
    let result: usize = cache.len();

    // Return result for part 1, along with loop indicator and cache for part 2.
    (result as i32, looop, cache)
}

// Function for counting how many new obstructions create loops.
fn part2(input: VecChars, mut obst: HashMap<usize, Position>, start: Position) -> i32 {
    // This is the list of visited indexes, which was used as the cache in part 1.
    // Adding obstructions on points where the guard never even goes to on their original path
    // is useless, so only consider visited nodes.
    let visited: HashSet<usize> = part1(&input, &obst, start).2;
    
    // Result tracking.
    let mut result: i32 = 0;

    // Iterate through the visited nodes on the basic route.
    visited.into_iter().for_each(|i| {
        // We only add obstructions to points that have nothing on them.
        match input.flat_board[i] {
            '.' => {
                // Add the obstruction.
                obst.insert(i, Position::new_obst(i / input.width, i % input.width, i));
                // Check if the loop tracker indicates a loop.
                if part1(&input, &obst, start).1 {
                    // Increment result if it does.
                    result += 1;
                } 
            }
            _ => {} // We don't care about 0, #, or the guards starting position.
        }
        // Remove the new obstruction for the next iteration.
        obst.remove(&i);
    });

    // Return the number of obstructions that cause loops.
    result
}

// Wrapper function for parts 1 and 2.
pub fn wrapper(input: VecChars) {
    let info: (HashMap<usize, Position>, Position) = find_info(&input);
    let part1_result: i32 = part1(&input, &info.0, info.1).0;
    let part2_result: i32 = part2(input, info.0, info.1);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}