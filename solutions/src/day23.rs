use libs::read_input::InputData;

use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Network {
    network: HashMap<String, HashSet<String>>,
    vertices: HashSet<String>
}

fn parse(input: &InputData) -> Network {
    let re: Regex = Regex::new(r"(?<x>[a-z]{2}?)(-)(?<y>[a-z]{2}?)").unwrap();
    let mut network: HashMap<String, HashSet<String>>= HashMap::new();
    let mut vertex_set: HashSet<String> = HashSet::new();

    input.list.iter().for_each(|pair| {
        if let Some(caps) = re.captures(pair[0].as_str()) {
            if network.contains_key(&caps["x"]) {
                network.get_mut(&caps["x"]).unwrap().insert(caps["y"].to_owned());
            } else {
                network.insert(caps["x"].to_owned(), HashSet::from([caps["y"].to_owned()]));
            }
            if network.contains_key(&caps["y"]) {
                network.get_mut(&caps["y"]).unwrap().insert(caps["x"].to_owned());
            } else {
                network.insert(caps["y"].to_owned(), HashSet::from([caps["x"].to_owned()]));
            }
            vertex_set.insert(caps["x"].to_owned());
            vertex_set.insert(caps["y"].to_owned());
        };
    });

    Network {
        network: network,
        vertices: vertex_set
    }
}

// Originally this was combined with the parse function, but it is very slow so I've separated it to comment it out while working on part 2.
fn part_1(input: &Network) -> usize {
    let network: &HashMap<String, HashSet<String>> = &input.network;
    let mut cycles: Vec<Vec<String>>= Vec::new();

    // I paid for these cores and I will use one of them.
    for (node, neighbours) in network {
        for neighbour in neighbours {
            if let Some(second_neighbours) = network.get(neighbour) {
                for second_neighbour in second_neighbours {
                    if let Some(third_neighbour) = network.get(second_neighbour) {
                        if third_neighbour.contains(node) {
                            let mut cycle: Vec<String> = vec![node.clone(), neighbour.clone(), second_neighbour.clone()];
                            cycle.sort();
                            if !cycles.contains(&cycle) {
                                cycles.push(cycle);
                            }
                        }
                    }
                }
            }
        }
    }

    cycles.retain(|cycle| {
        cycle.iter().any(|s| s.starts_with('t'))
    });

    println!("{}", cycles.len());

    cycles.len()
}

fn bron_kerbosch(
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    network: &Network
) {
    if p.is_empty() && x.is_empty() {
        println!("Maximal Clique: {r:?}");
        return;
    }

    let p_clone: HashSet<String> = p.clone();

    for v in p_clone {
        let mut v_set: HashSet<String> = HashSet::new();
        let n_v: &HashSet<String> = network.network.get(&v).unwrap();
        v_set.insert(v.clone());
        let mut new_r: HashSet<String> = r.union(&v_set).cloned().collect();
        let mut new_p: HashSet<String> = p.intersection(&n_v).cloned().collect();
        let mut new_x: HashSet<String> = x.intersection(&n_v).cloned().collect();

        bron_kerbosch(&mut new_r, &mut new_p, &mut new_x, network);

        p.retain(|x| !v_set.contains(x));
        x.extend(v_set);
    };

    //println!("End of recursion: {r:?}");
}

pub fn wrapper(input: InputData) {
    let network: Network = parse(&input);

    //part_1(&network);

    let mut initial_p = network.vertices.clone();

    bron_kerbosch(
        &mut HashSet::new(), 
        &mut initial_p,
        &mut HashSet::new(), 
        &network
    );
}