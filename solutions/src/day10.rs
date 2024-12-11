use libs::read_input::VecChars;

use std::collections::HashSet;

// Struct for storing the topographic map information.
struct TopographicMap {
    map: Vec<u32>, // Input map.
    map_h: usize, // Map height.
    map_w: usize, // Map width.
    th: Vec<usize>, // Trailhead indexes.
    p: Vec<usize>, // Peak indexes.
}

impl TopographicMap {
    // Creates a new TopographicMap
    fn new(map: Vec<u32>, h: usize, w: usize, th: Vec<usize>, p: Vec<usize>) -> Self {
        TopographicMap {
            map: map, map_h: h, map_w: w, th: th, p: p
        }
    }
}

// Finds trailheads and parses Vec<char> -> Vec<u32> for ease of use later.
fn parse(input: VecChars) -> TopographicMap {
    let mut parsed: Vec<u32> = Vec::new(); // Parsed input.
    let mut trailheads: Vec<usize> = Vec::new(); // Trailhead indexes.
    let mut peaks: Vec<usize> = Vec::new(); // Peak indexes.

    (0..input.flat_board.len()).for_each(|i| {
        if input.flat_board[i] == '0' { trailheads.push(i); }
        else if input.flat_board[i] == '9' { peaks.push(i); }
        parsed.push(input.flat_board[i].to_digit(10).unwrap());
    });

    TopographicMap::new(parsed, input.height, input.width, trailheads, peaks)
}
// I am struggling quite a bit today for some reason.
fn part1(input: TopographicMap) {
    fn neighbours(index: usize, height: usize, width: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let row = index / width;
        let col = index % width;

        if row > 0 { result.push(index - width); }
        if row < height - 1 { result.push(index + width); }
        if col > 0 { result.push(index - 1); }
        if col < width  - 1 { result.push(index + 1); }

        result
    }

    fn dfs(
        input: &TopographicMap, 
        index: usize, 
        target: u32, 
        visited: &mut HashSet<usize>
    ) -> i32 {
        println!("visiting: {} {}", index, input.map[index]);
        println!("target: {}", target);

        if input.map[index] != target { 
            println!("nope");
            return 0; 
        }

        if input.map[index] == 9 { return 1; }

        visited.insert(index);

        let mut paths: i32 = 0;
        for neighbour in neighbours(index, input.map_h, input.map_w) {
            if !visited.contains(&neighbour) {
                paths += dfs(&input, neighbour, target + 1, visited);
            }
        }

        visited.remove(&index);

        paths
    }

    let result: Vec<i32> = input.th.iter().map(|i| {
            let mut visited: HashSet<usize> = HashSet::new();
            dfs(&input, *i, 0, &mut visited)
        }).collect();

    println!("{result:?}")
}

pub fn wrapper(input: VecChars) {
    let topography: TopographicMap = parse(input);

    part1(topography);
}
