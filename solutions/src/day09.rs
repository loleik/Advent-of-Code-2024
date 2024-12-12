use libs::read_input::VecChars;

fn expand(input: &Vec<char>) -> Vec<(i32, String)> {
    let mut expanded: Vec<(i32, String)> = Vec::new();
    let mut empty: bool = false;
    let mut id_counter: i32 = 0;

    (0..input.len()).for_each(|i| {
        empty = false;
        let x: char = input[i];
        if (i+1) % 2 == 0 {
            empty = true;
        }

        (0..x.to_digit(10).unwrap()).for_each(|_| {
            match empty {
                false => expanded.push((id_counter, id_counter.to_string())),
                true => expanded.push((0, ".".to_string()))
            }
        });

        if !empty { id_counter += 1 }
    });

    expanded
}

fn defrag(input: &VecChars) -> Vec<(i32, String)> {
    let mut expanded = expand(&input.flat_board);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    /*for z in &expanded {
        print!("{}", z.1);
    }
    println!();*/

    let mut i = 0;
    let mut j = expanded.len() - 1;

    while i < j {
        while i < j && expanded[i].1 != ".".to_string() {
            i += 1;
        }

        while i < j && expanded[j].1 == ".".to_string() {
            j = j.saturating_sub(1);
        }

        if i < j {
            /*for z in 0..expanded.len() {
                if z == i {
                    print!("\x1b[41m\x1b[30m{}\x1b[0m", expanded[z].1);
                } else if z == j {
                    print!("\x1b[42m\x1b[30m{}\x1b[0m", expanded[z].1);
                } else {
                    print!("\x1b[1m{}\x1b[0m", expanded[z].1);
                }
            }
            println!();*/
            expanded.swap(i, j);
            i += 1;
            j = j.saturating_sub(1)
        }
    }

    /*for z in &expanded {
        print!("{}", z.1);
    }
    println!();*/

    expanded
}

fn find_blocks(input: &Vec<(i32, String)>) -> Vec<(usize, usize)> {
    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut in_block: bool = false;
    let mut start: usize = 0;
    let mut i: usize = 0;

    while i < input.len() {
        if input[i].1 != ".".to_string() && !in_block {
            start = i;
            in_block = true;
        }

        if i != input.len() - 1 && input[i].1 != input[i+1].1 && input[i].1 != ".".to_string() {
            blocks.insert(0, (start, i));
            in_block = false;
            i += 1;
        } else if i == input.len() - 1 && input[i - 1] == input[i] {
            blocks.insert(0, (start, i));
            break;
        } else {
            i += 1;
        }
    }

    blocks
}

fn defrag_blocks(input: &VecChars) -> Vec<(i32, String)> {
    let mut expanded: Vec<(i32, String)> = expand(&input.flat_board);
    let blocks: Vec<(usize, usize)> = find_blocks(&expanded);

    for block in &blocks {
        let size: usize = block.1 - block.0 + 1;
        let mut i: usize = 0;
        
        //println!("{block:?} {size}");


        /*for x in &expanded {
            print!("{}", x.1)
        }
        println!();*/

        while i < block.0 {
            while i < block.0 && expanded[i].1 != ".".to_string() {
                i += 1;
            }

            let mut j: usize = i;
            while j < block.1 && expanded[j].1 == ".".to_string() {
                j += 1;
            }

            //println!("{}", j - i);

            if j - i >= size {
                //println!("{} : {} : {}-{}", j - i, size, i, j);

                for k in 0..size {
                    expanded.swap(i + k, block.0 + k);
                }
                break;
            } else {
                i = j + 1;
                continue;
            }
        }
    }

    expanded
}

fn checksum(input: Vec<(i32, String)>) -> i64 {
    let mut result: i64 = 0;

    for i in 0..input.len() {
        result += (input[i].0 as i64) * (i as i64)
    }

    result
}

pub fn wrapper(input: VecChars) {
    let part1_result: i64 = checksum(defrag(&input));

    let part2_result: i64 = checksum(defrag_blocks(&input));
    
    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}