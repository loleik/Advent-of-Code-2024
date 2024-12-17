use libs::read_input::InputData;

// Opcodes represented as u8 we'll just ignore most of the bits.
struct System {
    registers: [u128; 3], // A, B and C registers
    pc: usize, // Program counter
    memory: Vec<u8>, // Stores the program.
    // Memory acts more like a disk drive or something here I suppose due to loading the program at initialization but whatever.
    output: Vec<u8>, // Stores output.
}

impl System {
    fn new(registers: [u128; 3], memory: Vec<u8>) -> Self {
        System { registers: registers, pc: 0, memory: memory, output: Vec::new() }
    }
}

fn initialize(input: &InputData) -> System {
    // Register A is at line 0 index 2.
    let a: u128 = input.list[0][2].parse().expect("Could not parse A.");
    // Register B is at line 1 index 2.
    let b: u128 = input.list[1][2].parse().expect("Could not parse B.");
    // Register C is at line 2 index 2.
    let c: u128 = input.list[2][2].parse().expect("Could not parse C.");

    // Program is stored in line 4 index 1.
    let program: Vec<u8> = input.list[4][1]
                .split(',')
                .collect::<Vec<_>>()
                .iter().map(|x| 
                    x.parse().expect("Could not parse program.")
                ).collect();

    System::new([a,b,c], program)
}

// Fetch next instruction from memory.
fn fetch(system: &mut System) -> u16 {
    if system.pc >= system.memory.len() {
        println!("Halting.");
        return 0x0F00 // Placeholder halt opcode.
    }

    // First byte will be the opcode, second byte is the operand.
    let instruction: u16 = (system.memory[system.pc] as u16) << 8
                           | (system.memory[system.pc + 1] as u16);

    system.pc += 2; // Each instruction is stored as 2 bytes, so increment by 2.

    instruction
}

// Decode the current instruciton and execute it.
// Combined as there aren't that many distinct opcodes anyway.
fn decode_execute(instruction: u16, system: &mut System) -> &mut System {
    // Most significant byte is opcode. Least significant is operand.
    let opcode: u8 = ((instruction & 0xFF00) >> 8) as u8;
    let operand: u8 = (instruction & 0x00FF) as u8;

    // Handling combo operands.
    fn combo_operand(operand: u8, system: &mut System) -> u32 {
        match operand {
            0 | 1 | 2 | 3 => { operand as u32 }
            4 => { system.registers[0] as u32 }
            5 => { system.registers[1] as u32 }
            6 => { system.registers[2] as u32 }
            _ => { panic!("Unrecognized combo operand {operand}!") }
        }
    }

    // Opcode matching and execution.
    match opcode {
        0x00 => { // adv
            let numerator: u128 = system.registers[0];
            let denominator: u128 = 2u128.pow(combo_operand(operand, system));
            
            // Integer division.
            system.registers[0] = numerator / denominator;
        }
        0x01 => { // bxl
            // Bitwise XOR.
            system.registers[1] = system.registers[1] ^ (operand as u128);
        }
        0x02 => { // bst
            // Combo modulo 8.
            system.registers[1] = (combo_operand(operand, system) % 8) as u128;
        }
        0x03 => { // jnz
            // Jump if B register is non-zero.
            if system.registers[0] != 0 {
                system.pc = operand as usize;
            }
        }
        0x04 => { // bxc
            // Bitwize XOR.
            system.registers[1] = system.registers[1] ^ system.registers[2];
        }
        0x05 => { // out
            // Combo modulo 8 then send to output.
            let out: u8 = (combo_operand(operand, system) % 8) as u8;
            print!("{out}, ");
            system.output.push(out);
        }
        0x06 => { // bdv
            let numerator: u128 = system.registers[0];
            let denominator: u128 = 2u128.pow(combo_operand(operand, system));
            
            // Integer division.
            system.registers[1] = numerator / denominator;
        }
        0x07 => { // cdv
            let numerator: u128 = system.registers[0];
            let denominator: u128 = 2u128.pow(combo_operand(operand, system));
            
            // Integer division.
            system.registers[2] = numerator / denominator;
        }
        _ => { println!("Unrecognized opcode {opcode:02X}") }
    }

    system
}

pub fn wrapper(input: InputData) {
    print!("\x1B[2J\x1B[1;1H");
    println!("Situation critical!");
    println!("Bootstrapping process failed. Initializing debugger....");
    println!("");

    // Initialize the system.
    let mut system: System = initialize(&input);

    // "Emulation" loop.
    loop {
        let instruction: u16 = fetch(&mut system);
        if instruction == 0x0F00 { break }
        decode_execute(instruction, &mut system);
    }

    println!();

    // I can't work out part 2. I've tried to reverse engineer the program and I can understan dit, but I see now way of that helpin gme.
}