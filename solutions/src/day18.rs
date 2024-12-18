use libs::read_input::InputData;
//use libs::print_inputs::print_map;

use std::collections::{VecDeque, HashSet, HashMap};

// Memory structure.
struct Memory {
    map: Vec<char>, // Flat map of memory.
    falling: Vec<(usize, usize, usize)>, // (index, row, col)
    height: usize,
    width: usize,
}

impl Memory {
    // Generates new memory structure.
    fn new(
        map: Vec<char>, 
        falling: Vec<(usize, usize, usize)>, 
        height: usize, width: usize
    ) -> Self {
        Memory {
            map: map,
            falling: falling,
            height: height,
            width: width
        }
    }
}

fn parse(input: &InputData, height: usize, width: usize) -> Memory {
    let mut falling: Vec<(usize, usize, usize)> = Vec::new(); // Falling byte information.

    // Grabs all the coordinates from the input and parses them.
    input.list.iter().for_each(|l| {
        let mut current: std::str::Split<'_, char> = l[0].split(',');
        match (current.next(), current.next()) {
            (Some(x), Some(y)) => {
                let x_p: usize = x.parse::<usize>().expect("Failed to parse y.");
                let y_p: usize = y.parse::<usize>().expect("Failed to parse x.");
                let i_p: usize = y_p * width + x_p;

                falling.push((i_p, y_p, x_p)); // (index, row, col)
            }
            _ => { println!("Line not valid: {current:?}") },
        }
    });

    // Returns memory structure.
    Memory::new(vec!['.'; height * width], falling, height, width)
}

// Breadth first search for shortest path between start and goal.
fn bfs(
    memory: &mut Memory, 
    start: (usize, usize, usize), 
    goal: (usize, usize, usize))
    -> Option<Vec<usize>> {
    let mut queue: VecDeque<usize> = VecDeque::new(); // Queue for traversing.
    let mut visited: HashSet<usize> = HashSet::new(); // Visited nodes.
    let mut parents: HashMap<usize, usize> = HashMap::new(); // Parents for backtracking.

    queue.push_back(start.0); // Enqueue start index.
    visited.insert(start.0); // Mark start index as visited.

    // Loop until queue is empty.
    while !queue.is_empty() {
        let v: usize = queue.pop_front().unwrap(); // We know queue is non-empty within, so unwrap v.

        // If we've reached the goal, backtrack path and return it.
        if v == goal.0 {
            let mut path: Vec<usize> = Vec::new();
            let mut current: usize = v;

            while current != start.0 {
                //memory.map[current] = 'O';
                path.push(current);
                current = parents[&current]
            }
            
            //memory.map[start.0] = 'O';
            path.push(start.0);
            path.reverse();
            return Some(path)
        }

        // Form neighbours vector for traversal.
        let mut neighbours: Vec<usize> = Vec::new();

        if v / memory.width > 0 { neighbours.push(v - memory.width) } // up
        if v / memory.width < memory.height - 1 { neighbours.push(v + memory.width) } // down
        if v % memory.width > 0 { neighbours.push(v - 1) } // left
        if v % memory.width < memory.width - 1 { neighbours.push(v + 1) } // right

        for w in neighbours {
            if !visited.contains(&w) // Don't revisit nodes.
               && memory.map[w] != '#' { // Can't pass through fallen bytes.
                visited.insert(w); // Mark neighbour as visited.
                parents.insert(w, v); // Mark neighbours parent for backtracking.
                queue.push_back(w); // Enqueue neighbour for traversal.
            }
        }
    }

    None // No path found. Useful for part 2.
}

pub fn wrapper(input: InputData) {
    // Initialize memory struct.
    let mut memory: Memory = parse(&input, 71, 71);

    // Part 1. Drop first 1024 bytes.
    for i in 0..1024 {
        memory.map[ memory.falling[i].0 ] = '#'
    }

    //print_map(&memory.map, 7, 7);

    // Start and end.
    let start: (usize, usize, usize) = (0, 0, 0);
    let goal: (usize, usize, usize) = (memory.map.len() - 1, 70, 70);

    // Generate a shortest path using breadth first search.
    let path: Vec<usize> = match bfs(&mut memory, start, goal) {
        Some(p) => p,
        None => vec![]
    };

    // If the path exists, return length -1 to omit the start index.
    if path.len() > 0 {
        println!("Shortest path length: {}", path.len() - 1)
    } else {
        println!("No path found!")
    }
    println!();

    // Part 2. Drop bytes until a path cannot be found at all then break.
    for i in 1024..memory.falling.len() {
        memory.map[ memory.falling[i].0 ] = '#';

        if bfs(&mut memory, start, goal) == None {
            println!("{:?} blocks all paths!", memory.falling[i]);
            break;
        }
    }

}