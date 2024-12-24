use libs::read_input::InputData;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct State {
    wire: String,
    signal: bool,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.wire == other.wire
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wire.hash(state);
    }
}

impl State {
    fn and(&self, other: &Self, wire: String) -> Self {
        State {
            wire: wire,
            signal: self.signal && other.signal
        }
    }

    fn or(&self, other: &Self, wire: String) -> Self {
        State {
            wire: wire,
            signal: self.signal || other.signal
        }
    }

    fn xor(&self, other: &Self, wire: String) -> Self {
        State {
            wire: wire,
            signal: self.signal ^ other.signal
        }
    }

    fn new(wire: String, signal: bool) -> Self {
        State {
            wire: wire,
            signal: signal
        }
    }
}

fn parse(input: &InputData) -> (HashSet<State>, Vec<Vec<String>>) {
    let mut states: HashSet<State> = HashSet::new();
    let mut logic: Vec<Vec<String>> = Vec::new();

    input.list.iter().for_each(|line| {
        if line.len() == 2 {
            let wire: String = line[0].replace(":", "");
            let signal: bool = match line[1].as_str() {
                "1" => true,
                "0" => false,
                _ => panic!("Invalid state!")
            };
            
            states.insert(State::new(wire, signal));
        } else if line.len() != 0 {
            logic.push(line.to_vec());
        }
    });

    (states, logic)
}

// This code is entirely wrong and terrible.
fn part_1(states: &mut HashSet<State>, logic: &Vec<Vec<String>>) {
    for statement in logic {
        match statement[1].as_str() {
            "AND" => {
                let a: State = State::new(statement[0].clone(), false);
                let b: State = State::new(statement[2].clone(), false);
                
                let a_got = states.get(&a).cloned().unwrap_or_else(|| {
                    &states.insert(a.clone());
                    a
                });

                let b_got = states.get(&b).cloned().unwrap_or_else(|| {
                    &states.insert(b.clone());
                    b
                });

                let c = State::and(
                    &a_got,
                    &b_got,
                    statement[4].clone()
                );

                states.insert(c);
            }
            "OR" => {
                let a: State = State::new(statement[0].clone(), false);
                let b: State = State::new(statement[2].clone(), false);
                let a_got = states.get(&a).cloned().unwrap_or_else(|| {
                    &states.insert(a.clone());
                    a
                });

                let b_got = states.get(&b).cloned().unwrap_or_else(|| {
                    &states.insert(b.clone());
                    b
                });

                let c = State::or(
                    &a_got,
                    &b_got,
                    statement[4].clone()
                );

                states.insert(c);
            }
            "XOR" => {
                let a: State = State::new(statement[0].clone(), false);
                let b: State = State::new(statement[2].clone(), false);

                let a_got = states.get(&a).cloned().unwrap_or_else(|| {
                    &states.insert(a.clone());
                    a
                });

                let b_got = states.get(&b).cloned().unwrap_or_else(|| {
                    &states.insert(b.clone());
                    b
                });

                let c = State::xor(
                    &a_got,
                    &b_got,
                    statement[4].clone()
                );

                states.insert(c);
            }
            _ => { println!("Invalid operator: {}", statement[1]) }
        }
    }

    let mut z_states: Vec<&State> = Vec::new();

    for state in states.iter() {
        if state.wire.starts_with("z") {
            z_states.push(state);
        }
    }

    z_states.sort_by(|a, b| a.wire.cmp(&b.wire));

    let mut result: Vec<i32> = Vec::new();

    for i in (0..z_states.len()).rev() {
        match z_states[i].signal {
            true => result.push(1),
            false => result.push(0),
        }
    }

    println!("{result:?}")
}

pub fn wrapper(input: InputData) {
    let mut states: (HashSet<State>, Vec<Vec<String>>) = parse(&input);

    part_1(&mut states.0, &states.1);
}