use libs::read_input::VecChars;

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

fn print_map(input: &TopographicMap) {
    for row in 0..input.map_h {
        for col in 0..input.map_w {
            let i: usize = row * input.map_w + col;
            if input.th.contains(&i) {
                print!("\x1b[31m{:3}\x1b[0m ", input.map[row * input.map_w + col]);
            } else if input.p.contains(&i) {
                print!("\x1b[32m{:3}\x1b[0m ", input.map[row * input.map_w + col]);
            } else {
                print!("{:3} ", input.map[row * input.map_w + col]);
            }
        }
        println!();
    }
    println!();
}

// I am struggling quite a bit today for some reason.
fn part1(input: TopographicMap) -> i32{
    print_map(&input);

    if input.th.len() == 0 { return 0 }
    let mut count: i32 = 0;
    let mut neighbours: Vec<Vec<usize>> = Vec::new();

    for x in &input.th {
        if input.map[*x] == 9 { return 1 }

        let row: usize = x / input.map_w;
        let col: usize = x % input.map_w;

        let mut current_n: Vec<usize> = Vec::new();

        if row > 0 {
            if input.map[*x] + 1 == input.map[x - input.map_w] {
                current_n.push(x - input.map_w);
            }
        }
        if row < input.map_h - 1 {
            if input.map[*x] + 1 == input.map[x + input.map_w] {
                current_n.push(x + input.map_w);
            }
        }
        if col > 0 {
            if input.map[*x]+ 1 == input.map[x - 1] {
                current_n.push(x - 1);
            }
        }
        if col < input.map_w - 1 {
            if input.map[*x] + 1 == input.map[x + 1] {
                current_n.push(x + 1);
            }
        }

        neighbours.push(current_n);
    }

    for n in neighbours {
        count += part1(TopographicMap::new(
            input.map.clone(), input.map_h, input.map_w, n, input.p.clone()
        ));
    }

    count
}

pub fn wrapper(input: VecChars) {
    let topography: TopographicMap = parse(input);

    let part1_result: i32 = part1(topography);

    println!("Part 1: {part1_result}");
}
