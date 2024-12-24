use libs::read_input::InputData;

const NUMPAD: [[char; 3]; 4] = [
     ['7','8','9'], 
     ['4','5','6'],
     ['1','2','3'],
     [' ','0','A'],
];

const DPAD: [[char; 3]; 2] = [
     [' ','^','A'], 
     ['<','v','>'],
];

pub fn wrapper(input: InputData){
    let codes: Vec<String> = input.list.into_iter().flatten().collect();

    for x in codes { println!("{x}") }
}