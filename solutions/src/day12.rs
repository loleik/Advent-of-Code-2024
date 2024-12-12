use std::collections::{HashSet, VecDeque};

use libs::read_input::VecChars;

// Lots of repeated logic today. This can be tidied up significantly.

// Identifies the distinct regions in the map. Uses flood-fill.
fn find_regions(input: &VecChars) -> Vec<Vec<usize>> {
    let mut regions: Vec<Vec<usize>> = Vec::new(); // Final result.
    let mut queue: VecDeque<usize> = VecDeque::new(); // Queue for visitng nodes.
    let mut visited: HashSet<usize> = HashSet::new(); // Visited nodes.

    // Loop through every node in the map.
    for x in 0..input.flat_board.len() {
        // If we've already visited and used x then continue.
        if visited.contains(&x) {
            continue;
        }
        
        // Push x to the back of the queue.
        queue.push_back(x);
        let ch: char = input.flat_board[x]; // Current character to look for.
        let mut region: Vec<usize> = Vec::new(); // Current region found.

        // Continue looping until the queue is exhausted.
        while !queue.is_empty() {
            // Grab the node at the front of the queue, and remove it form the queue.
            let n: usize = queue.pop_front().unwrap();

            // If node isn't in a region already, and matches the current character continue.
            if !visited.contains(&n) && input.flat_board[n] == ch {
                visited.insert(n); // We've now used the node.
                region.push(n); // It is in a region so store it.

                let row: usize = n / input.width; // row = index / width
                let col: usize = n % input.width; // col = index % width

                // Check if neighbour exists and hasn't been visited.
                // If they do and haven't, then add them to the queue.
                // Up
                if row > 0 && !visited.contains(&(n - input.width)) {
                    queue.push_back(n - input.width);
                }
                // Down
                if row < (input.height - 1) && !visited.contains(&(n + input.width)) {
                    queue.push_back(n + input.width);
                }
                // Left
                if col > 0 && !visited.contains(&(n - 1)) {
                    queue.push_back(n - 1);
                }
                // Right
                if (col + 1) < input.width && !visited.contains(&(n + 1)) {
                    queue.push_back(n + 1);
                }
            }
        }

        // If we found a region, store it.
        if !region.is_empty() {
            regions.push(region);
        }
    }

    regions // Returns the vector of all distinct regions.
}

// Calculates the perimeter of a given region.
fn perimeter(region: &Vec<usize>, input: &VecChars) -> i32 {
    let mut perimeter: i32 = 0; // Final result.

    // Loop through all indexes within the region.
    for &index in region.iter() {
        let row: usize = index / input.width;
        let col: usize = index % input.width;

        // Check if neighbour is outside the map, or hasn't been visited.
        // If either, increment perimeter.
        // Up
        if row == 0 || !region.contains(&(index - input.width)) {
            perimeter += 1;
        }
        // Down
        if row == input.height - 1 || !region.contains(&(index + input.width)) {
            perimeter += 1;
        }
        // Left
        if col == 0 || !region.contains(&(index - 1)) {
            perimeter += 1;
        }
        // Right
        if col + 1 == input.width || !region.contains(&(index + 1)) {
            perimeter += 1;
        }
    }

    perimeter
}

// Calculates the amount of corners a region has.
// Number of corners = Numer of sides.
fn corners(region: &Vec<usize>, input: &VecChars) -> i32 {
    let mut corner_count: i32 = 0; // Final result

    // If the region is a single node, it always has 4 corners, no need to proceed.
    if region.len() == 1 {
        return 4;
    }

    // Loop through all indexes in the region to find corners.
    for &index in region.iter() {
        let row: usize = index / input.width;
        let col: usize = index % input.width;

        // Conditions for if each direction is outside the map or not.
        let up: bool = row > 0;
        let down: bool = row < input.height - 1;
        let left: bool = col > 0;
        let right: bool = col < input.width - 1;

        // Things get messy from here but I'll do my best to explain.

        // Matching up and right directions.
        match (up, right) {
            (true, true) => { // Both are in the map.
                if !region.contains(&(index - input.width)) &&
                   !region.contains(&(index + 1)) {
                    /* This case would be true for A:
                        B B
                        A B
                    */
                    corner_count += 1;
                } else if !region.contains(&(index - input.width)) &&
                          region.contains(&((index - input.width) + 1)) {
                    /* This case would be true for A:
                        A B
                        A A
                    */
                    corner_count += 1;
                }
            }
            (false, true) => {
                // If up is outside, and right isn't in the region, that's a corner.
                if !region.contains(&(index + 1)) {
                    corner_count += 1;
                }
            }
            (true, false) => {
                // If right is outside, and up isn't in the region, that's a corner.
                if !region.contains(&(index - &input.width)) {
                    corner_count += 1;
                }
            }
            (false, false) => {
                // If both are outside, that's a corner.
                corner_count += 1;
            }
        }

        // The rest of the conditions are the same, just applied in different directions.

        match (up, left) {
            (true, true) => {
                if !region.contains(&(index - input.width)) &&
                   !region.contains(&(index - 1)) {
                    corner_count += 1;
                } else if !region.contains(&(index - input.width)) &&
                          region.contains(&((index - input.width) - 1)) {
                    corner_count += 1;
                }
            }
            (false, true) => {
                if !region.contains(&(index - 1)) {
                    corner_count += 1;
                }
            }
            (true, false) => {
                if !region.contains(&(index - &input.width)) {
                    corner_count += 1;
                }
            }
            (false, false) => {
                corner_count += 1;
            }
        }

        match (down, right) {
            (true, true) => {
                if !region.contains(&(index + input.width)) &&
                   !region.contains(&(index + 1)) {
                    corner_count += 1;
                } else if !region.contains(&(index + input.width)) &&
                          region.contains(&((index + input.width) + 1)) {
                    corner_count += 1;
                }
            }
            (false, true) => {
                if !region.contains(&(index + 1)) {
                    corner_count += 1;
                }
            }
            (true, false) => {
                if !region.contains(&(index + &input.width)) {
                    corner_count += 1;
                }
            }
            (false, false) => {
                corner_count += 1;
            }
        }

        match (down, left) {
            (true, true) => {
                if !region.contains(&(index + input.width)) &&
                   !region.contains(&(index - 1)) {
                    corner_count += 1;
                } else if !region.contains(&(index + input.width)) &&
                          region.contains(&((index + input.width) - 1)) {
                    corner_count += 1;
                }
            }
            (false, true) => {
                if !region.contains(&(index - 1)) {
                    corner_count += 1;
                }
            }
            (true, false) => {
                if !region.contains(&(index + &input.width)) {
                    corner_count += 1;
                }
            }
            (false, false) => {
                corner_count += 1;
            }
        }
    }

    corner_count
}

// Function for part 1.
fn part1(input: &VecChars) -> i32 {
    let mut result: i32 = 0; // Total cost of fence.

    let regions: Vec<Vec<usize>> = find_regions(&input); // Find the regions.

    // For every region, calculate perimeter p and then add p * area to the total cost.
    for r in regions {
        result += perimeter(&r, &input) * (r.len() as i32);
    }

    result // Return total price.
}

// Function for part 2.
fn part2(input: &VecChars) -> i32 {
    let mut result: i32 = 0; // Total cost of fence.

    let regions: Vec<Vec<usize>> = find_regions(&input); // Find the regions.

    // For every region, calculate the number of corners, and add corners * area to the total cost.
    for r in regions {
        result += corners(&r, &input) * (r.len() as i32);
    }

    result // Return total cost.
}

// Wrapper for both parts.
pub fn wrapper(input: VecChars) {
    let part1_result: i32 = part1(&input);
    let part2_result:i32 = part2(&input);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}