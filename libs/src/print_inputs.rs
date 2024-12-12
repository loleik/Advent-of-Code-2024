pub fn print_map(map: Vec<char>, height: usize, width: usize) {
    for row in 0..height {
        for col in 0..width {
            let i: usize = row * width + col;
            print!("{:3} ", map[i])
        }
        println!();
    }
    println!();
}