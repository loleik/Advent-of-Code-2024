use libs::read_input::VecChars;
use libs::print_inputs::print_map;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::usize;

struct Maze {
    map: Vec<char>,
    start: (usize, usize, usize),
    end: (usize, usize, usize),
    height: usize,
    width: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    position: usize,
    direction: char,
    parent: Option<usize>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Generate the struct for the current maze.
fn get_maze(input: &VecChars) -> Maze {
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

    let maze: Maze = Maze {
        map: input.flat_board.clone(),
        start: start,
        end: end,
        height: input.height,
        width: input.width
    };

    print_map(&maze.map, maze.height, maze.width);

    maze
}

fn dijkstra(maze: &mut Maze) -> (Option<usize>, Option<Vec<usize>>) {
    let mut dist: Vec<_> = (0..maze.map.len()).map(|_| usize::MAX).collect();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    let mut prevs: Vec<Option<usize>> = vec![None; maze.map.len()];

    dist[maze.start.0] = 0;
    heap.push(State { cost: 0, position: maze.start.0, direction: 'r', parent: None });

    while let Some(State { cost, position, direction , parent}) = heap.pop() {
        if position == maze.end.0 {
            let mut path: Vec<usize> = Vec::new();
            let mut current: Option<usize> = Some(position);
            while let Some(pos) = current {
                path.push(pos);
                current = prevs[pos];
            }
            path.reverse();
            return (Some(cost), Some(path));
        }

        let mut neighbours: Vec<(usize, char)> = Vec::new();
        if maze.map[position + 1] != '#' {
            neighbours.push((position + 1, 'r'));
        }
        if maze.map[position - 1] != '#' {
            neighbours.push((position - 1, 'l'));
        }
        if maze.map[position + maze.width] != '#' {
            neighbours.push((position + maze.width, 'd'));
        }
        if maze.map[position - maze.width] != '#' {
            neighbours.push((position - maze.width, 'u'));
        }


        if cost > dist[position] { continue; }

        for (p, c) in neighbours {
            if maze.map[p] == 'c' || p >= dist.len() { continue; }

            let move_cost: usize = if c == direction {
                1
            } else {
                match direction {
                    'u' => if c == 'd' { 2001 } else { 1001 },
                    'd' => if c == 'u' { 2001 } else { 1001 },
                    'l' => if c == 'r' { 2001 } else { 1001 },
                    'r' => if c == 'l' { 2001 } else { 1001 },
                    _ => usize::MAX,
                }
            };

            let new_cost = cost + move_cost;

            if new_cost < dist[p] {
                prevs[p] = Some(position);
                heap.push(State { 
                    cost: new_cost,
                    position: p,
                    direction: c,
                    parent: Some(position)
                });
                dist[p] = new_cost;

                //maze.map[p] = 'X';
                //print!("\x1B[2J\x1B[H");
                //std::thread::sleep(std::time::Duration::from_millis(60));
                //print_map(&maze.map, maze.height, maze.width);
            }
        }
    }

    (None, None)
}

// This works for part 1 but i don't understand how to expand it for part 2.

pub fn wrapper(input: VecChars) {
    let mut maze: Maze = get_maze(&input);
    let result: (Option<usize>, Option<Vec<usize>>) = dijkstra(&mut maze);
    println!("Part 1: {:?}", result.0);
    for r in result.1.unwrap() {
        if maze.map[r] != 'S' && maze.map[r] != 'E' {
            maze.map[r] = '█'
        }
    }
    print_map(&maze.map, maze.height, maze.width);
}