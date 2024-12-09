use libs::read_input::VecChars;

fn expand(input: &Vec<char>) -> Vec<(i32, char)> {
    let mut expanded: Vec<(i32, char)> = Vec::new();
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
                false => expanded.push((id_counter, x)),
                true => expanded.push((0, '.'))
            }
        });

        if !empty { id_counter += 1 }
    });

    expanded
}

fn defrag(mut input: Vec<(i32, char)>) -> Vec<(i32, char)> {
    let mut i = 0;
    let mut j = input.len() - 1;

    while i < j {
        while i < j && input[i].1 != '.' {
            i += 1;
        }

        while i < j && input[j].1 == '.' {
            j = j.saturating_sub(1)
        }

        if i < j {
            input.swap(i, j);
            i += 1;
            j = j.saturating_sub(1)
        }
    }

    input
}


// I can't work this out at all. The loop seems to freeze and never exit,
// or exit early depending on what conditions I add.
// I can get it to print the right answer for the example if it's left until it freezes.
// But that doesn't work for part 2.
fn defrag_blocks(mut input: Vec<(i32, char)>) -> Vec<(i32, char)> {
    let mut i = 0; // Pointer for moving forward.
    let mut j = input.len() - 1; // Pointer for moving backwards.

    //println!("{input:?}");

    // Loop until the pointers cross each other.
    while i < j {
        // Decrement backward pointer until we find a non-dot (a number) entry.
        while input[j].1 == '.' {
            j = j.saturating_sub(1);
        }

        // Start a second backward pointer to identify the end of the block of numbers.
        let mut l: usize = j;
        while input[l].1 == input[j].1 {
            l = l.saturating_sub(1);
        }

        // Increment forward pointer until it reaches a dot.
        while i < j && input[i].1 != '.' {
            i += 1;
        }

        // Start a second forward pointer to identify the end of the block of dots.
        let mut k: usize = i;
        while k < j && input[k].1 == '.' {
            k += 1;
        }

        // Check if the block of numbers can fit into the block of dots.
        if i < j && (k - i) >= (j - l) {
            // Swap the blocks
            for x in 0..(j - l) {
                input.swap(i + x, j - x);
            }

            // Update pointers to reflect the swapped blocks
            i = 0;
            j = l;
        } else if i < j {
            i = k;
        } else {
            //println!("{input:?}");
            //println!("{i} {k} {j} {l}");
            println!("{}", checksum(input.clone()));
            i = 0;
            j = l;
        }
        //println!("{input:?}");
    }

    //println!("{input:?}");

    input
}

fn checksum(input: Vec<(i32, char)>) -> i64 {
    let mut result: i64 = 0;

    for i in 0..input.len() {
        result += (input[i].0 as i64) * (i as i64)
    }

    result
}

pub fn wrapper(input: VecChars) {
    let expanded_1: Vec<(i32, char)> = expand(&input.flat_board);
    let expanded_2: Vec<(i32, char)> = expand(&input.flat_board);

    let part1_result: i64 = checksum(defrag(expanded_1));
    let part2_result: i64 = checksum(defrag_blocks(expanded_2));

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}