use std::collections::{VecDeque, HashSet, HashMap};

use crate::read_input::VecChars;

pub fn bfs(
    board: &mut VecChars,
    start: (usize, usize, usize), 
    goal: (usize, usize, usize),
    mark_map: bool,
)-> Option<Vec<usize>> {
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

            loop {
                if mark_map { board.flat_board[current] = '█' }
                if current == start.0 { break }
                path.push(current);
                current = parents[&current]
            }
            
            path.push(start.0);
            path.reverse();
            return Some(path)
        }

        // Form neighbours vector for traversal.
        let mut neighbours: Vec<usize> = Vec::new();

        if v / board.width > 0 { neighbours.push(v - board.width) } // up
        if v / board.width < board.height - 1 { neighbours.push(v + board.width) } // down
        if v % board.width > 0 { neighbours.push(v - 1) } // left
        if v % board.width < board.width - 1 { neighbours.push(v + 1) } // right

        for w in neighbours {
            if !visited.contains(&w) // Don't revisit nodes.
               && board.flat_board[w] != '#' { // Can't pass through fallen bytes.
                visited.insert(w); // Mark neighbour as visited.
                parents.insert(w, v); // Mark neighbours parent for backtracking.
                queue.push_back(w); // Enqueue neighbour for traversal.
            }
        }
    }

    None // No path found. Useful for part 2.
}