use libs::read_input::VecChars;

use crossterm::{ExecutableCommand, style::{Print, SetForegroundColor, Color}};
use indicatif::{ProgressBar, ProgressStyle, TermLike};
use std::{fmt::format, io::{self, stdout, Write}};
use console::{pad_str, Alignment, Term};
use std::thread;
use std::time::Duration;

// Expands the input string out to a usable form.
fn expand(input: &Vec<char>) -> Vec<(i32, String)> {
    let mut expanded: Vec<(i32, String)> = Vec::new(); // The output variable
    let mut empty: bool = false; // Tracks if a section is empty space.
    let mut id_counter: i32 = 0; // Counter to add the IDs

    // Loop through the whole input.
    (0..input.len()).for_each(|i| {
        // Reset empty space tracker.
        empty = false;
        // Grab the current character.
        let x: char = input[i];
        // If it's divisible by 2, then its empty space. Plus 1 to account for indexes.
        if (i+1) % 2 == 0 {
            empty = true;
        }

        // Loop through to the end of the current section, which is defined by the current character.
        (0..x.to_digit(10).unwrap()).for_each(|_| {
            match empty {
                false => expanded.push((id_counter, id_counter.to_string())), // Populated by data.
                true => expanded.push((0, ".".to_string())) // Empty space.
            }
        });

        if !empty { id_counter += 1 } // Increment the ID's if not an empty space.
    });

    expanded // Return the expanded disk, which will be used for everything.
}

// The lazy function. "Defragments" by entry not by block.
fn defrag(input: &VecChars) -> Vec<(i32, String)> {
    let mut expanded = expand(&input.flat_board); // Expand.

    let mut i: usize = 0; // The start pointer for finding empty space.
    let mut j: usize = expanded.len() - 1; // The end pointer for finding non-empty values.
    let initial_range: u64 = (j - i + 1) as u64; // The initial range of the loop, used for progress bar.

    // Use term to center the start message properly.
    let term: Term = Term::stdout(); 
    let width: usize = term.width() as usize;
    let start: std::borrow::Cow<'_, str> = pad_str(
        "--> LAZY DEFRAG <--", 
        width, 
        Alignment::Center, 
        None
    );

    io::stdout().flush().unwrap();

    // Print the start message and define the progress bar.
    println!("{}", start);
    let progress_bar: ProgressBar = ProgressBar::new(initial_range);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{elapsed_precise}▐{wide_bar}") // Include {msg} for the message
            .unwrap()
            .progress_chars("█▓░"),
    );

    progress_bar.set_message("Checking entries:");

    // Loop until the pointers meet.
    while i < j {
        // Increment i until we find empty space.
        while i < j && expanded[i].1 != ".".to_string() {
            i += 1;
            // Update the progress bar.
            progress_bar.set_position(initial_range - (j - i + 1) as u64);
            // Slowed down artificially for the visuals. Not needed.
            thread::sleep(Duration::from_nanos(10));
        }

        // If i finds empty space, send out j from the end to find a character to fill it.
        while i < j && expanded[j].1 == ".".to_string() {
            j = j.saturating_sub(1);
            progress_bar.set_position(initial_range - (j - i + 1) as u64);
        }

        // If we find one, do the swap, then continue if i < j.
        if i < j {
            expanded.swap(i, j);
            i += 1;
            j = j.saturating_sub(1);
            progress_bar.set_position(initial_range - (j - i + 1) as u64);
            thread::sleep(Duration::from_nanos(10));
        }
    }

    // Finish message.
    progress_bar.finish_with_message("All entries checked!");

    // Get and print the checksum.
    println!("Filesystem Checksum: {}", checksum(&expanded));

    expanded // Return the final filesystem.
}

// My method for part 2 is definitley not the fastest, but it helped me understand the problem better.
// Locate and document all file blocks in the filesystem.
fn find_blocks(input: &Vec<(i32, String)>) -> Vec<(usize, usize)> {
    let mut blocks: Vec<(usize, usize)> = Vec::new(); // Block information.
    let mut in_block: bool = false; // Tracks if we're currently traversing a block.
    let mut start: usize = 0; // Start of the current block.
    let mut i: usize = 0; // Index for looping.

    // Looop through the whole filesystem.
    while i < input.len() {
        // If we find a non-empty character and aren't already in a block, start one.
        if input[i].1 != ".".to_string() && !in_block {
            start = i;
            in_block = true;
        }

        // If we reach the end of a block, store the start and end, and exit the block.
        if i != input.len() - 1 && input[i].1 != input[i+1].1 && input[i].1 != ".".to_string() {
            blocks.insert(0, (start, i));
            in_block = false;
            i += 1;
        // If we're at the end of the filesystem, check backwards to avoid indexing out of bounds.
        } else if i == input.len() - 1 && input[i - 1] == input[i] {
            blocks.insert(0, (start, i));
            break;
        // Otherwise continue traversing the block.
        } else {
            i += 1;
        }
    }

    // Blocks are sorted in reverse to simulate searching from the end of the filesystem, this is to accommodate for how the problem is set up.
    blocks // Return blocks.
}

// Defragmenting by block instead of entry.
fn defrag_blocks(input: &VecChars) -> Vec<(i32, String)> {
    let mut expanded: Vec<(i32, String)> = expand(&input.flat_board); // Expand.
    let blocks: Vec<(usize, usize)> = find_blocks(&expanded); // Blocks.

    display_filesystem(&expanded, find_blocks(&expanded));

    // Set up terminal and progress bar again.
    let term = Term::stdout();
    let width = term.width() as usize;
    let start = pad_str(
        "--> FULL DEFRAG <--", 
        width, 
        Alignment::Center, 
        None
    );

    io::stdout().flush().unwrap();

    println!("{}", start);
    // Only difference is we have a clear endpoint here for the progress bar, which is the amount of blocks.
    let progress_bar = ProgressBar::new(blocks.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{elapsed_precise}▐{wide_bar}") // Include {msg} for the message
            .unwrap()
            .progress_chars("█▓░"),
    );

    // Loop through all blocks on the filesystem.
    for b in 0..blocks.len() {
        // Update the current block at each step.
        progress_bar.set_message(format!("Checking Blocks: {}/{}", b + 1, blocks.len()));

        let size: usize = blocks[b].1 - blocks[b].0 + 1; // Current block size.
        let mut i: usize = 0; // Set index of pointer for finding empty space.

        // Send the initial pointer out towards the start of the block.
        while i < blocks[b].0 {
            // Increment until there is free space.
            while i < blocks[b].0 && expanded[i].1 != ".".to_string() {
                i += 1;
            }

            // Initialize another pointer and send it out from i to find the end of the free space block.
            let mut j: usize = i;
            while j < blocks[b].1 && expanded[j].1 == ".".to_string() {
                j += 1;
            }

            // If the block of free space is big enough, swap the current block into it and move on to the next block.
            if j - i >= size {
                for k in 0..size {
                    expanded.swap(i + k, blocks[b].0 + k);
                }
                break;
            // If the current block of free space is too small, continue searching.
            } else {
                i = j + 1;
                continue;
            }
        }
        progress_bar.inc(1); // Increment every time a block is processed.
    }

    progress_bar.finish_with_message("All blocks checked!"); // Finished message.

    // Print the filesystem checksum.
    println!("Filesystem Checksum: {}", checksum(&expanded));

    display_filesystem(&expanded, find_blocks(&expanded));

    // Return the new filesystem layout.
    expanded
}

fn display_filesystem(input: &Vec<(i32, String)>, blocks: Vec<(usize, usize)>) {
    let mut i: usize = 0;
    let mut dots: bool = false;
    let mut dots_len: usize = 0;

    while i < input.len() {
        let c: &String  = &input[i].1;

        if c == "." && !dots {
            stdout().execute(Print(" [.]")).unwrap();
            stdout().flush().unwrap();

            i += 1;
            dots_len += 1;
            dots = true;
        } else if c == "." && dots {
            i += 1;
            dots_len += 1;
        } else {
            if dots {
                stdout().execute(Print(format!(":({dots_len})"))).unwrap();
            }

            for block in &blocks {
                if i >= block.0 && i <= block.1 {
                    stdout().execute(Print(format!(" [{c}] "))).unwrap();
                    stdout().flush().unwrap();

                    i = block.1 + 1;
                    break;
                }
            }

            if c != "." {
                dots_len = 0;
                dots = false
            } else {
                i += 1;
            }
        }
    }
    println!();
}

// Calculates the checksum.
fn checksum(input: &Vec<(i32, String)>) -> i64 {
    let mut result: i64 = 0;

    // For every entry, multiply the ID by the position.
    for i in 0..input.len() {
        result += (input[i].0 as i64) * (i as i64)
    }

    result
}

pub fn wrapper(input: VecChars) {
    // Clear the terminal.
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    defrag(&input);

    defrag_blocks(&input);
}