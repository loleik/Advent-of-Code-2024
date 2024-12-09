use libs::read_input::VecChars;

use num::complex::Complex;
use std::collections::HashSet;

fn part1(input: &VecChars) -> (i32, Vec<char>) {
    let mut cache: HashSet<i32> = HashSet::new();
    let mut new_input: Vec<char> = input.flat_board.clone();

    (0..input.flat_board.len()).for_each(|i| {
        let a: Complex<i32> = if input.flat_board[i] != '.' {
            Complex::new((i % input.width) as i32, (i / input.width) as i32)
        } else {
            Complex::new(input.flat_board.len() as i32, 0)
        };

        if a.re < input.flat_board.len() as i32 {
            (i+1..input.flat_board.len()).for_each(|j| {
                if input.flat_board[j] == input.flat_board[i] {
                    let b: Complex<i32> = Complex::new(
                        (j % input.width) as i32, (j / input.width) as i32
                    );

                    let k_1: Complex<i32> = b + (b - a);
                    let k_2: Complex<i32> = a - (b - a);

                    let k_1i: i32 = k_1.im * (input.width as i32) + k_1.re;
                    let k_2i: i32 = k_2.im * (input.width as i32) + k_2.re;

                    if k_1.re < (input.width as i32)&& k_1.im < (input.height as i32)
                    && k_1.re >= 0 && k_1.im >= 0
                    && !cache.contains(&k_1i) {
                        cache.insert(k_1i);
                        new_input[k_1i as usize] = input.flat_board[i]
                    }

                    if k_2.re < (input.width as i32) && k_2.im < (input.height as i32)
                    && k_2.re >= 0 && k_2.im >= 0
                    && !cache.contains(&k_2i) {
                        cache.insert(k_2i);
                        new_input[k_2i as usize] = input.flat_board[i]
                    }
                }
            });
        }
    });

    (cache.len() as i32, new_input)
}

/*fn part2(input: &VecChars) {
    let mut cache: HashSet<i32> = HashSet::new();
    let mut new_input: Vec<char> = input.flat_board.clone();

    (0..input.flat_board.len()).for_each(|i| {
        let a: Complex<i32> = if input.flat_board[i] != '.' {
            Complex::new((i % input.width) as i32, (i / input.width) as i32)
        } else {
            Complex::new(input.flat_board.len() as i32, 0)
        };

        if a.re < input.flat_board.len() as i32 {
            (i+1..input.flat_board.len()).for_each(|j| {
                if input.flat_board[j] == input.flat_board[i] {
                    let b: Complex<i32> = Complex::new(
                        (j % input.width) as i32, (j / input.width) as i32
                    );

                    let diff = b - a;

                    let mut k_1: Complex<i32> = b + diff;
                    let k_2: Complex<i32> = a - diff;

                    let mut k_1i: i32 = k_1.im * (input.width as i32) + k_1.re;
                    let k_2i: i32 = k_2.im * (input.width as i32) + k_2.re;

                    if k_1.re < (input.width as i32)&& k_1.im < (input.height as i32)
                    && k_1.re >= 0 && k_1.im >= 0
                    && !cache.contains(&k_1i) {                        
                        loop {
                            cache.insert(k_1i);
                            new_input[k_1i as usize] = '#';
                            
                            k_1 = k_1 + diff;
                            k_1i = k_1.im * (input.width as i32) + k_1.re;

                            if k_1i > (input.flat_board.len() as i32) {
                                break;
                            }
                        }
                    }

                    if k_2.re < (input.width as i32) && k_2.im < (input.height as i32)
                    && k_2.re >= 0 && k_2.im >= 0
                    && !cache.contains(&k_2i) {
                        cache.insert(k_2i);
                        new_input[k_2i as usize] = input.flat_board[i];
                    }
                }
            });
        }
    });

    println!("{}", cache.len());

    print_2d_vector(new_input, input.width);
}*/

fn print_2d_vector(flat_vec: Vec<char>, width: usize) {
    let height = flat_vec.len() / width;
    for row in 0..height {
        let start_index = row * width;
        let end_index = start_index + width;
        println!("{:?}", &flat_vec[start_index..end_index]);
    }
}

pub fn wrapper(input: VecChars) {
    let part1_result: (i32, Vec<char>) = part1(&input);
    //part2(&input);

    println!("Part 1: {}", part1_result.0);
    //println!("Part 2: {}", part2_result.0);

    print_2d_vector(part1_result.1, input.width);
}