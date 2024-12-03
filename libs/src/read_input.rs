use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct InputData {
    pub list: Vec<Vec<String>>,
}

pub struct NonSplitData {
    pub list: Vec<String>
}

pub fn parse_to_vec_nosplit(path: &str) -> NonSplitData {
    let file = match File::open(path) {
        Ok(value) => value,
        Err(e) => panic!("Error: {e}"),
    };

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|line|
            line.expect("Line error")
        )
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
