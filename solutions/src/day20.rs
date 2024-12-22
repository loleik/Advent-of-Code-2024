use libs::read_input::{parse_to_vec_chars_2, VecChars};
use libs::print_inputs::print_map;
use libs::traversal::bfs;

fn find_cheat(input: &mut VecChars, path: &Vec<usize>) {
    let width: isize = input.width as isize;
    let directions: [isize; 4] = [
        1, width, -1, -width
    ]; // Right, Down, Left, Up
    
    let index: isize = path[1] as isize;
    let index_row: isize = index / input.width as isize;
    let index_col: isize = index / input.width as isize;

    for d in directions {
        let n: isize = index + d;

        match d {
            -1 | 1 => {
                if index_row == n / input.width as isize {
                    input.flat_board[n as usize] = '█';
                    for d2 in directions {
                        input.flat_board[(n + d2) as usize] = '█';
                        println!("{index_row} {}", (n + d2) / input.width as isize);
                        if index_row == (n + d2) / input.width as isize {
                            input.flat_board[(n + d2) as usize] = '█'
                        }
                    }
                }
            }
            _ => {
                if n >= 0 && n < input.flat_board.len() as isize {
                    input.flat_board[n as usize] = '█';
                    input.flat_board[(n + d) as usize] = '█'
                }
            }
        }
    }
}

pub fn wrapper(path: &str) {
    let mut input = parse_to_vec_chars_2(path);

    // Find the start. Panic if it isn't there.
    let start: (usize, usize, usize) = (0..input.flat_board.len()).find_map(|c| {
        if input.flat_board[c] == 'S' {
            Some(( c, c / input.width, c % input.width ))
        } else {
            None
        }
    }).expect("No start found in the map!");

    // Find the end. Panic if it isn't there.
    let end: (usize, usize, usize) = (0..input.flat_board.len()).find_map(|c| {
        if input.flat_board[c] == 'E' {
            Some(( c, c / input.width, c % input.width ))
        } else {
            None
        }
    }).expect("No end found in the map!");

    // Find the normal route through the map.
    let normal_path: Vec<usize> = 
        match bfs(&mut input, start, end, false) {
            Some(path) => path,
            None => vec![], // Shouldn't happen today.
        };
    
    find_cheat(&mut input, &normal_path);

    print_map(&input.flat_board, input.height, input.width);
}