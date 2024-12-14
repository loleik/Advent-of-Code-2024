use libs::read_input::InputData;

use regex::Regex;
use std::fs::File;
use std::io::{self, Write};
use std::usize;
use std::collections::HashMap;

// Struct for storing and updating information on an individual robot.
#[derive(Clone, Copy, Debug)]
struct Robot {
    pos: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    // Default function for generating an empty robot instance.
    fn default() -> Self {
        Robot { pos: (0, 0), v: (0, 0) }
    }
}

// Represents a single position.
#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty(char),
    Robot(usize)
}

// Parses input pulling out robot information and putting it into structs.
fn parse(input: &InputData) -> Vec<Robot> {
    // Regex for grabbing x and y values.
    let re: Regex = Regex::new(r"^(?<def>[vp])=(?<col>-?\d+),(?<row>-?\d+)$").unwrap();
    let mut robots: Vec<Robot> = Vec::new(); // Vector of robots.
    
    // Loop through all lines in the input.
    for line in &input.list {
        // Empty robot that will be filled and stored later.
        let mut robot: Robot = Robot::default();
        line.iter().for_each(|l| { // Iterate through the current line.
            // Capture the x and y values.
            if let Some(caps) = re.captures(&l) {
                match &caps["def"] {
                    // V is velocity in the input, p is position. Push information to the empty robot.
                    "v" => { robot.v = (caps["col"].parse().unwrap(), caps["row"].parse().unwrap()) }
                    "p" => { robot.pos = (caps["col"].parse().unwrap(), caps["row"].parse().unwrap()) }
                    _ => { println!("Malformed input: {:?}", line) }
                }
            }
        });
        // Store the newly generated robot.
        robots.push(robot);
    }

    robots // Return robots.
}

// Perform a set amount of ticks for the given robots.
fn tick(robots: &mut Vec<Robot>, secs: u32, width: i32, height: i32) {
    let mut i = 1; // Starting value.

    // Iterate through the specified amount of seconds, moving the robots at each step.
    while i <= secs {
        for r in 0..robots.len() {
            // New col and row, kept in range using modular arithmatic.
            // Modified to be in line with mathematical modulo behaviour.
            let new_col: i32 = ((robots[r].pos.0 + robots[r].v.0) % width + width)% (width);
            let new_row: i32 = ((robots[r].pos.1 + robots[r].v.1) % height + height) % (height);
            robots[r].pos = (new_col, new_row); // Update the current robot.
        }
        i += 1 // Move to the next tick.
    }
}

// Plots the robots in the map.
fn plot(
    width: i32, 
    height: i32, 
    robots: &mut Vec<Robot>, 
    step: u32, // Amount of ticks passed.
    danger: &mut HashMap<u32, usize> // Storage of danger values for every tick.
) {
    let mut map: Vec<Cell> = vec![Cell::Empty('.'); (width * height) as usize]; // Generates the map.

    tick(robots, 1, width, height); // Runs a single tick.

    // Loops through updated robots, plotting them onto the map.
    for robot in robots {
        // index = row * width + col.
        let i = (robot.pos.1 * width + robot.pos.0) as usize;
        match map[i] {
            // If a robot is already there, increment by 1.
            Cell::Robot(j) => {
                map[i] = Cell::Robot(j + 1)
            }
            // If empty, place a 1.
            Cell::Empty('.') => {
                map[i] = Cell::Robot(1)
            }
            _ => {}
        }
    }

    // Count the amount of robots in the top two quadrants.
    let mut top_left: usize = 0;
    let mut top_right: usize = 0;
    for row in 0..(height / 2) {
        for col in 0..(width / 2) {
            let i: usize = (row * width + col) as usize;
            match map[i] {
                Cell::Robot(j) => { top_left += j }
                _ => {}
            }
        }
        for col in (width / 2)..width {
            let i: usize = (row * width + col) as usize;
            match map[i] {
                Cell::Robot(j) => { top_right += j }
                _ => {}
            }
        }
    }

    // Count the robots in the bottom two quadrants.
    let mut bot_left: usize = 0;
    let mut bot_right: usize = 0;
    for row in (height / 2)..height {
        for col in 0..(width / 2) {
            let i: usize = (row * width + col) as usize;
            match map[i] {
                Cell::Robot(j) => { bot_left += j }
                _ => {}
            }
        }
        for col in (width / 2)..width {
            let i: usize = (row * width + col) as usize;
            match map[i] {
                Cell::Robot(j) => { bot_right += j }
                _ => {}
            }
        }
    }

    // For part 2. I was having problems with file limits in folders.
    // Fixed by only generating files for the range you want.
    let _ = print_out(map, width, step);

    // Insert the current steps danger value.
    danger.insert(step, top_left * top_right * bot_left * bot_right);
}

// Not really used for anything other than inspecting the files to check the outputted value.
// My initial plan was to output all the maps as text files, convert them to image, and use something like Pytorch to examine the images and look for a tree.
// I may still do that now that I have the actual solution.
fn print_out(map: Vec<Cell>, width: i32, step: u32) -> io::Result<()> {
    let file: File = File::create(format!("day14/files/{step}.txt"))?;
    let mut writer: io::BufWriter<File> = io::BufWriter::new(file);

    for i in 0..map.len() {
        match map[i] {
            Cell::Empty(_) => {
                if (i + 1) % (width as usize) == 0 {
                    write!(writer, "  \n")?;
                } else {
                    write!(writer, "  ")?;
                }
            }
            Cell::Robot(_) => {
                if (i + 1) % (width as usize) == 0 {
                    write!(writer, "█ \n")?;
                } else {
                    write!(writer, "█ ")?;
                }
            }
        }
    }

    Ok(())
}

pub fn wrapper(input: InputData) {
    let mut robots: Vec<Robot> = parse(&input); // Vector of robots.
    let mut danger: HashMap<u32, usize> = HashMap::new(); // Initial danger value map.
    let mut i: u32 = 0; // Initial tick count.
    let total: u32 = 10000; // Total tick count to work to.
    let mut x: (u32, usize) = (0, usize::MAX); // initial min danger value.

    // Loop through all the ticks between i and total, saving the danger value and output file.
    while i < total {
        plot(101, 103, &mut robots, i, &mut danger);
        i += 1;
    }

    // Loop through the danger levels and find the second smallest one.
    // The smallest will be a vertically squashed looking pattern, the next is the tree, and the next is a horizontally squashed pattern.
    // This three stage pattern then repeats seemingly forever.
    // 1325 is the smallest value.
    for level in &danger {
        if level.1 < &x.1 && level.0 != &1325 {
            x = (*level.0, *level.1)
        }
    }
    // Print value. Plus 1 to account for indexing among other errors.
    // x.0 is the index, which is the answer, x.1 is the danger value for that index.
    println!("Second lowest danger level: ({:?}, {})", x.0 + 1, x.1);
}