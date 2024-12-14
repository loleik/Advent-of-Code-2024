use libs::read_input::InputData;

use regex::Regex;
use std::fs::File;
use std::io::{self, Write};
use std::usize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Robot {
    pos: (i32, i32),
    v: (i32, i32),
}

impl Robot {
    /*fn new(x: usize, y: usize, vx: i32, vy: i32) -> Self {
        Robot { pos: (x, y), v: (vx, vy) }
    }*/
    fn default() -> Self {
        Robot { pos: (0, 0), v: (0, 0) }
    }
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty(char),
    Robot(usize)
}

fn parse(input: &InputData) -> Vec<Robot> {
    let re: Regex = Regex::new(r"^(?<def>[vp])=(?<col>-?\d+),(?<row>-?\d+)$").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    
    for line in &input.list {
        let mut robot: Robot = Robot::default();
        line.iter().for_each(|l| {
            if let Some(caps) = re.captures(&l) {
                match &caps["def"] {
                    "v" => { robot.v = (caps["col"].parse().unwrap(), caps["row"].parse().unwrap()) }
                    "p" => { robot.pos = (caps["col"].parse().unwrap(), caps["row"].parse().unwrap()) }
                    _ => { println!("Malformed input: {:?}", line) }
                }
            }
        });
        robots.push(robot);
    }

    robots
}

fn tick(robots: &mut Vec<Robot>, secs: u32, width: i32, height: i32) {
    let mut i = 1;

    while i <= secs {
        for r in 0..robots.len() {
            let new_col: i32 = ((robots[r].pos.0 + robots[r].v.0) % width + width)% (width);
            let new_row: i32 = ((robots[r].pos.1 + robots[r].v.1) % height + height) % (height);
            robots[r].pos = (new_col, new_row);
        }
        i += 1
    }
}

fn plot(
    width: i32, 
    height: i32, 
    robots: &mut Vec<Robot>, 
    step: u32, 
    danger: &mut HashMap<u32, usize>
) {
    let mut map: Vec<Cell> = vec![Cell::Empty('.'); (width * height) as usize];

    tick(robots, 1, width, height);

    for robot in robots {
        let i = (robot.pos.1 * width + robot.pos.0) as usize;
        match map[i] {
            Cell::Robot(j) => {
                map[i] = Cell::Robot(j + 1)
            }
            Cell::Empty('.') => {
                map[i] = Cell::Robot(1)
            }
            _ => {}
        }
    }

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

    if step < 10000 {
        let _ = print_out(map, width, step);
    }

    danger.insert(step, top_left * top_right * bot_left * bot_right);
}

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
    let mut robots: Vec<Robot> = parse(&input);
    let mut danger: HashMap<u32, usize> = HashMap::new();
    let mut i: u32 = 0;
    let total: u32 = 10000;
    let mut x: (u32, usize) = (0, usize::MAX);

    /*let progress_bar: ProgressBar = ProgressBar::new(total as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{elapsed_precise}▐{wide_bar}") // Include {msg} for the message
            .unwrap()
            .progress_chars("█▓░"),
    );*/

    while i < total {
        plot(101, 103, &mut robots, i, &mut danger);
        //progress_bar.set_position(i as u64);
        i += 1;
    }

    for level in &danger {
        if level.1 < &x.1 && level.0 != &1325 {
            x = (*level.0, *level.1)
        }
    }

    println!("Second lowest danger level: {x:?} ");
}