use std::collections::HashMap;
use std::cmp;
use regex::Regex;

use libs::read_input::InputData;

// Struct for storing the ruleset and list of changes generated from the input.
struct Updates {
    rules: HashMap<i32, Vec<i32>>,
    changes: Vec<Vec<i32>>
}

/*
    Parses the input into the Updates structure. I've been trying to keep input
    parsing generic and in the libs crate, but this feels specific enough that
    it should be fine.
*/
fn create_struct(input: &InputData) -> Updates {
    let mut rules_end: usize = 0;

    // Find where the rules end, this is at [].
    (0..input.list.len()).for_each(|i| {
        if input.list[i].len() == 0 {
            rules_end = i;
        }
    });

    // Regex for pulling integers out for the rules.
    let re = Regex::new(r"(?<left>\d{2})(\|)(?<right>\d{2})").unwrap();

    // A mutable vector containing everything as a flattened Vec<String>.
    let mut rules: Vec<String> = input.list.clone()
        .iter()
        .flat_map(|line| line.to_owned())
        .collect();

    // Splits the vector at the end of the rules, creating the vector of sets.
    // Also parses the sets into Vec<i32>'s.
    let sets: Vec<Vec<i32>> = rules.split_off(rules_end)
        .iter()
        .map(|set| 
            set.split(',')
               .map(|s| s.parse().expect("Could not parse to int"))
               .collect()
        )
        .collect();
    
    // The rules map.
    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    for rule in &rules {
        // If input is captured, define it and continue.
        if let Some(caps) = re.captures(rule) {
            // Pull out the named capture gorups.
            let left: i32 = caps["left"].parse().unwrap();
            let right: i32 = caps["right"].parse().unwrap();

            // Keys can have multiple entried in the input.
            if rules_map.contains_key(&left) {
                // Get a mutable copy of the existing value for the key.
                let mut new = rules_map.get(&left).unwrap().clone();
                
                // If it doesn't contain the current value, push it.
                if !new.contains(&right) {
                    new.push(right);
                }

                // Push the new vector to the map.
                rules_map.insert(left, new);
            } else {
                // If the key isn't already there, just add it.
                rules_map.insert(left, vec![right]);
            }
        }
    }

    // Return the parsed input.
    Updates { rules: rules_map, changes: sets }
}

// Very messy function for checking if an input is valid.
fn check_update(update: &Vec<i32>, popped: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    // Get mutable copies for tracking, and get the current value.
    let mut new_list: Vec<i32> = update.clone();
    let current: i32 = new_list.remove(0);
    let mut new_popped: Vec<i32> = popped.clone();

    // Check if the current value has a rule and define it if it has.
    if let Some(current_rule) = rules.get(&current) {
        // Check right that all values follow current rule.
        for n in &new_list {
            if !current_rule.contains(n) {
                // If they don't, it's invalid.
                return false;
            }
        }

        // Check left for any invalid positions.
        for m in popped {
            if current_rule.contains(m) {
                // If they exist, it's invalid
                return false
            }
        }
    }

    // Add the current value to the left part of the update.
    new_popped.push(current);

    // If it's the last value and we haven't already returned false, it's valid.
    // If it isn't the last value, recursion happens with the next value.
    if new_list.len() == 0 {
        return true
    } else {
        check_update(&new_list, &new_popped, rules)
    }
}

// Sorting algorithm to make invalid updates valid, I'm not sure if it has a name.
// I would like to tidy up check_update() with a method like this.
fn something_sort(update: &mut Vec<i32>, rules: &HashMap<i32, Vec<i32>>) {
    // Closure defining how the values will be sorted.
    update.sort_by(|a, b| {
        // If a rule exists for a, define it and check.
        if let Some(rule_a) = rules.get(&a) {
            // If rule_a contains b, a should be to the left of b.
            if rule_a.contains(b) {
                return cmp::Ordering::Less;
            }
        }

        // If a rule exists for b, define it and check.
        if let Some(rule_b) = rules.get(&b) {
            // If rule_b contains a, b should be to the left of a.
            if rule_b.contains(a) {
                return cmp::Ordering::Greater;
            }
        }

        // Arbitrary default comparison for when rules don't exist.
        a.cmp(b)
    });
}

// Code for getting the final results for both parts.
fn parts(input: Updates) -> (i32, i32) {
    // Initial results.
    let mut result_1: i32 = 0; // Part 1 result.
    let mut result_2: i32 = 0; // Part 2 result.

    // Loop through every update from the structure.
    (0..input.changes.len()).for_each(|x| {
        let update = &input.changes[x];
        // If the update is valid, grab the median and add it to result_1.
        // If it isn't continue to else.
        if check_update(update, &vec![], &input.rules) {
            result_1 += update[update.len() / 2]
        } else {
            // Create a mutable copy of the update.
            let mut update_mut: Vec<i32> = update.clone();

            // Pass the mutable copy to the sorting algorithm.
            something_sort(&mut update_mut, &input.rules);
            
            // If the sort worked, grab the median and add it to result_2.
            if check_update(&update_mut, &vec![], &input.rules) {
                result_2 += update_mut[update_mut.len() / 2]
            } else {
                println!("{update:?} : Sort Failed!")
            }
        }
    });

    // Return the results.
    (result_1, result_2)
}

// Wrapper for parts 1 and 2.
pub fn wrapper(input: &InputData) -> (i32, i32) {
    let updates: Updates = create_struct(input);
    let (part_1, part_2) = parts(updates);

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    (part_1, part_2)
}