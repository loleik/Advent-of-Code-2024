use libs::read_input::InputData;
use libs::print_inputs::print_map;
use std::collections::VecDeque;

// Struct for storing warehouse information.
#[derive(Debug)]
struct Warehouse {
    map: Vec<char>,
    robot: (usize, usize, usize),
    h_w: (usize, usize),
    instructions: VecDeque<char>
}

// Function for generating a new warehouse.
impl Warehouse {
    fn new(
        map: Vec<char>, 
        robot: (usize, usize, usize), 
        h_w: (usize, usize), 
        instructions: VecDeque<char>) 
    -> Self {
        Warehouse { map: map, robot: robot, h_w: h_w, instructions: instructions }
    }
}

// Function for parsing input.
fn parse(input: &InputData) -> Warehouse {
    let mut map: Vec<char> = Vec::new(); // Empty map.
    let mut queue: VecDeque<char> = VecDeque::new(); // Empty queue.
    let mut height_width: (usize, usize) = (0, input.list[0][0].len()); // Height and width storage.

    input.list.iter().for_each(|line| {
        if line.len() != 0 { // If it isn't the blank line.
            let mut line_split: Vec<char> = line[0].chars().collect();
            // Increment the height and push the row to the map.
            if line_split.len() == input.list[0][0].chars().collect::<Vec<_>>().len() {
                height_width.0 += 1;
                map.append(&mut line_split)
            } else {
                // Add all the instrucitons to the queue.
                line_split.iter().for_each(|l| queue.push_back(*l));
            }
        }
    });

    // Find the robot. Panic if it isn't there.
    let robot: (usize, usize, usize) = (0..map.len()).find_map(|c| {
        if map[c] == '@' {
            Some(( c, c / height_width.1, c % height_width.1 ))
        } else {
            None
        }
    }).expect("No robot found in the map!");

    Warehouse::new(map, robot, height_width, queue) // Return warehouse.
}

// Traverses through instructions. Too confusing and direct to work for part 2.
fn traverse(warehouse: &mut Warehouse) {
    while !warehouse.instructions.is_empty() { // While there are instructions left.
        let current: char = warehouse.instructions.pop_front().unwrap(); // Pop next instruction.

        match current { // Check what the instruciton is.
            '^' => { // Up.
                let n_i: usize = warehouse.robot.0 - warehouse.h_w.1; // Upward neighbour.
                match warehouse.map[n_i] { // Check neighbours value.
                    '#' => { continue } // If it's a wall, can't move just continue to next instruction.
                    '.' => { // If its a blank space, swap the robot to the next space and continue.
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    'O' => { // If its a box, process it.
                        // Check upwards until we find either a wall or empty space.
                        let mut check: usize = n_i;
                        while warehouse.map[check] == 'O' {
                            check -= warehouse.h_w.1
                        }
                        match warehouse.map[check] {
                            '#' => { continue } // If it's a wall, can't move, continue.
                            '.' => { // If its an empty space, swap the first box with the first empty space, then swap the robot. Continue.
                                warehouse.map.swap(n_i, check);
                                warehouse.map.swap(n_i, warehouse.robot.0);
                                warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1) 
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            // The rest of the directions follow the same pattern.
            'v' => { // Down.
                let n_i: usize = warehouse.robot.0 + warehouse.h_w.1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    'O' => {
                        let mut check: usize = n_i;
                        while warehouse.map[check] == 'O' {
                            check += warehouse.h_w.1
                        }
                        match warehouse.map[check] {
                            '#' => { continue }
                            '.' => {
                                warehouse.map.swap(n_i, check);
                                warehouse.map.swap(n_i, warehouse.robot.0);
                                warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1) 
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            '<' => { // Left.
                let n_i: usize = warehouse.robot.0 - 1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    'O' => {
                        let mut check: usize = n_i;
                        while warehouse.map[check] == 'O' {
                            check -= 1
                        }
                        match warehouse.map[check] {
                            '#' => { continue }
                            '.' => {
                                warehouse.map.swap(n_i, check);
                                warehouse.map.swap(n_i, warehouse.robot.0);
                                warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1) 
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            '>' => { // Right.
                let n_i: usize = warehouse.robot.0 + 1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    'O' => {
                        let mut check: usize = n_i;
                        while warehouse.map[check] == 'O' {
                            check += 1
                        }
                        match warehouse.map[check] {
                            '#' => { continue }
                            '.' => {
                                warehouse.map.swap(n_i, check);
                                warehouse.map.swap(n_i, warehouse.robot.0);
                                warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1) 
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            _ => println!("Unexpected instruction: {current}")
        }
    }
}

// Calculate the sum of the GPS values.
fn gps(warehouse: &Warehouse) -> usize {
    let mut total_gps: usize = 0;

    for i in 0..warehouse.map.len() {
        if warehouse.map[i] == 'O' {
            total_gps += 100 * (i / warehouse.h_w.1) + (i % warehouse.h_w.1)
        }
    }

    println!("GPS Tota: {total_gps}");
    total_gps
}

// Part 1 wrapper.
fn part1(input: &InputData) {
    let mut warehouse: Warehouse = parse(input);

    traverse(&mut warehouse);

    //print_map(&warehouse.map, warehouse.h_w.0, warehouse.h_w.1);
    gps(&warehouse);
}

// I've given up on this. The part 1 code is far too messy to work for part 2. I don't know how to do it.
fn traverse_part2(warehouse: &mut Warehouse) {
    while !warehouse.instructions.is_empty() {
        let current: char = warehouse.instructions.pop_front().unwrap();

        match current {
            '^' => {
                let n_i: usize = warehouse.robot.0 - warehouse.h_w.1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    '[' | ']' => {
                        let mut check: (usize, char) = (n_i, warehouse.map[n_i]);
                        while check.1 == '[' || check.1 ==  ']' {
                            check.0 -= warehouse.h_w.1;
                            check.1 = warehouse.map[check.0]
                        }
                        match check.1 {
                            '#' => { continue }
                            '.' => {
                                let i: usize = warehouse.robot.0;
                                let l: usize = warehouse.robot.0 - warehouse.h_w.1;
                                let j: usize = check.0;
                                let mut k: usize = 0;

                                while j + k < i {

                                    if j + warehouse.h_w.1 < i
                                    && warehouse.map[j + warehouse.h_w.1] == ']' 
                                    && warehouse.map[j + warehouse.h_w.1 + 1] == '[' {
                                        warehouse.map.swap(j + warehouse.h_w.1 + 1, j + 1);
                                        warehouse.map.swap(j + warehouse.h_w.1 + 2, j + 2);
                                    }

                                    warehouse.map.swap(j + k, j + k + warehouse.h_w.1);
                                    if warehouse.map[j + k] == '[' {
                                        warehouse.map.swap(j + k + 1, j + k + warehouse.h_w.1 + 1);
                                    } else if warehouse.map[j + k] == ']' {
                                        warehouse.map.swap(j + k - 1, j + k + warehouse.h_w.1 - 1);
                                    }
                                    k += warehouse.h_w.1
                                }

                                warehouse.robot = (l, l / warehouse.h_w.1, l % warehouse.h_w.1)
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            'v' => {
                let n_i: usize = warehouse.robot.0 + warehouse.h_w.1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    '[' | ']' => {
                        let mut check: (usize, char) = (n_i, warehouse.map[n_i]);
                        while check.1 == '[' || check.1 ==  ']' {
                            check.0 += warehouse.h_w.1;
                            check.1 = warehouse.map[check.0]
                        }
                        match check.1 {
                            '#' => { continue }
                            '.' => {
                                let i: usize = warehouse.robot.0;
                                let l: usize = warehouse.robot.0 + warehouse.h_w.1;
                                let j: usize = check.0;
                                let mut k: usize = 0;

                                while j - k > i {

                                    if j - warehouse.h_w.1 > i
                                    && warehouse.map[j - warehouse.h_w.1] == ']' 
                                    && warehouse.map[j - warehouse.h_w.1 + 1] == '[' {
                                        warehouse.map.swap(j - warehouse.h_w.1 + 1, j + 1);
                                        warehouse.map.swap(j - warehouse.h_w.1 + 2, j + 2);
                                    }

                                    warehouse.map.swap(j - k, j - k - warehouse.h_w.1);
                                    if warehouse.map[j - k] == '[' {
                                        warehouse.map.swap(j - k + 1, j - k - warehouse.h_w.1 + 1);
                                    } else if warehouse.map[j - k] == ']' {
                                        warehouse.map.swap(j - k - 1, j - k - warehouse.h_w.1 - 1);
                                    }
                                    k += warehouse.h_w.1
                                }

                                warehouse.robot = (l, l / warehouse.h_w.1, l % warehouse.h_w.1)
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            '<' => {
                let n_i: usize = warehouse.robot.0 - 1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    ']' => {
                        let mut check: usize = n_i;
                        while warehouse.map[check] == '[' || warehouse.map[check] == ']' {
                            check -= 1
                        }
                        match warehouse.map[check] {
                            '#' => { continue }
                            '.' => {
                                let i: usize = warehouse.robot.0;
                                let j: usize = check;
                                let mut k: usize = 0;

                                while j +k < i {
                                    warehouse.map.swap(j + k, j + k + 1);
                                    k += 1
                                }

                                warehouse.robot = (i - 1, i / warehouse.h_w.1, i % warehouse.h_w.1)
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            '>' => {
                let n_i: usize = warehouse.robot.0 + 1;
                match warehouse.map[n_i] {
                    '#' => { continue }
                    '.' => {
                        warehouse.map.swap(n_i, warehouse.robot.0);
                        warehouse.robot = (n_i, n_i / warehouse.h_w.1, n_i % warehouse.h_w.1)
                    }
                    '[' => {
                        let mut check: usize = n_i;
                        while warehouse.map[check] == '[' || warehouse.map[check] == ']' {
                            check += 1
                        }
                        match warehouse.map[check] {
                            '#' => { continue }
                            '.' => {
                                let i: usize = warehouse.robot.0;
                                let j: usize = check;
                                let mut k: usize = 0;

                                while j - k > i {
                                    warehouse.map.swap(j - k, j - k - 1);
                                    k += 1
                                }
                                warehouse.robot = (i + 1, i / warehouse.h_w.1, i % warehouse.h_w.1)
                            }
                            _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                        }
                    }
                    _ => println!("Unexpected map item {} at {n_i}", warehouse.map[n_i])
                }
            }
            _ => println!("Unexpected instruction: {current}")
        }
        print_map(&warehouse.map, warehouse.h_w.0, warehouse.h_w.1);
    }
}

// Part 2 wrapper.
fn part2(input: &InputData) {
    let mut warehouse: Warehouse = parse(input);
    let mut new_map: Vec<char> =Vec::new();

    // Update width. Height doesn't change.
    warehouse.h_w.1 = warehouse.h_w.1 * 2;

    // Expand the map.
    warehouse.map.iter().for_each(|c| {
        match c {
            '#' => new_map.append(&mut vec!['#', '#']),
            '.' => new_map.append(&mut vec!['.', '.']),
            '@' => new_map.append(&mut vec!['@', '.']),
            'O' => new_map.append(&mut vec!['[', ']']),
            _ => println!("Unexpected map item {c}")
        }
    });

    // Locate robot in expanded map.
    warehouse.robot = (0..new_map.len()).find_map(|c| {
        if new_map[c] == '@' {
            Some(( c, c / warehouse.h_w.1, c % warehouse.h_w.1 ))
        } else {
            None
        }
    }).expect("No robot found in the map!");

    warehouse.map = new_map;
    
    print_map(&warehouse.map, warehouse.h_w.0, warehouse.h_w.1);

    traverse_part2(&mut warehouse);

}

pub fn wrapper(input: InputData) {
    part1(&input);
    part2(&input);
}