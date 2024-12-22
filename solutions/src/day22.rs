use libs::read_input::InputData;

use std::collections::{HashSet, HashMap};

fn parse(input: &InputData) -> Vec<u64> {
    let mut values: Vec<u64> = Vec::new();

    input.list.iter().for_each(|line| {
        if line.len() == 1 {
            values.push(line[0].parse::<u64>().expect("Couldn't parse value."));
        }
    });

    values
}

fn mix(value: u64, secret: u64) -> u64 {
    value ^ secret
}

fn prune(value: u64) -> u64 {
    value % 16777216
}

fn prng(value: u64) -> u64 {
    let step_1 = prune(mix(value * 64, value));

    let step_2 = prune(mix(step_1 / 32, step_1));

    let final_step = prune(mix(step_2 * 2048, step_2));

    final_step
}

fn part_1(input: &InputData) {
    let mut values: Vec<u64> = parse(input);

    for i in 0..values.len() {
        for _j in 0..10 {
            values[i] = prng(values[i]);
        }
    }

    println!("Part 1: {}", values.iter().sum::<u64>());
    println!();
}

fn part_2(input: &InputData) {
    let mut values: Vec<u64> = parse(input);
    let mut prices: Vec<i8> = Vec::new();
    
    let mut counts: HashMap<Vec<i8>, i8> = HashMap::new();

    for i in 0..values.len() {
        prices.push((values[i] % 10) as i8);
        let mut last_4: Vec<i8> = Vec::new();

        for j in 0..2000 {
            values[i] = prng(values[i]);
            prices.push((values[i] % 10) as i8);

            if j > 4 {
                counts.entry(last_4.clone())
                      .and_modify(|c| *c += prices[j])
                      .or_insert(prices[j]);
                last_4.remove(0);
                last_4.push(prices[j] - prices[j - 1]);
            } else if j > 0 {
                last_4.push(prices[j] - prices[j - 1]);
            }
        }
    }

    let test: Vec<i8> = vec![-2i8,1i8,-1i8,3i8];

    println!("[-2, 1, -1, 3] : {:?}", counts.get(&test));
}

pub fn wrapper(input: InputData) {
    part_1(&input);
    part_2(&input);
}