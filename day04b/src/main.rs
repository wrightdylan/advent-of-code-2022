use std::{fs, collections::HashSet};

/*
 * This time we need to find how many partial overlaps there are between elves.
 * 
 * Coding strategy
 * ===============
 * 
 * Same as before, but use is_disjoint, which is equivalent to testing if there
 * is an empty intersection.
 */

// I didn't need to explicitly return a bool since these methods already return a bool
fn check_complete_overlap(elf_one: &HashSet<usize>, elf_two: &HashSet<usize>) -> bool {
    elf_one.is_subset(&elf_two) | elf_one.is_superset(&elf_two)
}

fn main() {
    let mut complete_overlaps: usize = 0;
    let mut partial_overlaps: usize = 0;
    
    let data = fs::read_to_string("./input.txt").expect("Unable to read file.");
    for line in data.lines() {
        let assignments: Vec<usize> = line.split([',', '-'])
                                          .flat_map(|a| a.parse::<usize>())
                                          .collect();
        
        
        let elf_one: HashSet<usize> = (assignments[0]..=assignments[1]).collect();
        let elf_two: HashSet<usize> = (assignments[2]..=assignments[3]).collect();

        if check_complete_overlap(&elf_one, &elf_two) { complete_overlaps += 1 };
        if !elf_one.is_disjoint(&elf_two) { partial_overlaps += 1 };
        // I need to find a better way of doing this. Apparently incrementing is faux pas in Rust.
        // The Rusty way is with .count(), so need to build the previous statements as iterators.
    }

    println!("The number of completely overlapping assignments is: {complete_overlaps}");
    println!("The number of completely overlapping assignments is: {partial_overlaps}");
}
