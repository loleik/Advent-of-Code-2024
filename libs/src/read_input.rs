use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct InputData {
    pub list: Vec<Vec<String>>,
}

pub struct NonSplitData {
    pub list: Vec<String>,
}

pub struct VecChars {
    pub height: usize,
    pub width: usize,
    pub flat_board: Vec<char>,
}

// I've really overcomplicated this nightmarish function
pub fn parse_to_vec_chars(path: &str) -> VecChars {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {e}"),
    };

    let reader: BufReader<File> = BufReader::new(file);

    let mut board: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("Line error").chars().collect::<Vec<_>>())
        .collect();

    let mut width = board[0].len();

    board.insert(0, vec!['0'; width]);
    board.push(vec!['0'; width]);

    let height = board.len();

    (0..height).for_each(|i| {
        board[i].insert(0, '0');
        board[i].push('0');
    });

    width = board[0].len();

    let flat_board: Vec<char> = board
        .iter()
        .flat_map(|line| line.iter().map(|c| c.to_owned()))
        .collect();

    VecChars {
        height: height,
        width: width,
        flat_board: flat_board,
    }
}

pub fn parse_to_vec_nosplit(path: &str) -> NonSplitData {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {e}"),
    };

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Line error"))
        .collect();

    NonSplitData { list: lines }
}

pub fn parse_to_vec(path: &str) -> InputData {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {e}"),
    };

    let reader = BufReader::new(file);

    let lines: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.expect("Line error")
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect()
        })
        .collect();

    InputData { list: lines }
}
