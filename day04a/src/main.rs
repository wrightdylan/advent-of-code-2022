use std::{fs, collections::HashSet};

/*
 * The file to be read contains pairs of sector assignments. Some of these
 * assignments overlap. Some of these assignmens overlap completely. The task
 * is to find how many of these assignments fall into that latter category.
 * 
 * Coding strategy
 * ===============
 * 
 * Each line in the file is read as a string. Each string will be split in half
 * at the comma (this is a CSV). Should I try using a map this time? The
 * assignments look like they would fit very well as tuples. Alternatively, and
 * this might be easier, is to split the string into a single vector or array,
 * and index the assignment values directly. The main learning point personally
 * is in splitting that string/string slice.
 */

// A cool thing I found out while reading about HashSets on rust-lang.org for
// yesterday's challenge is that there are methods called is_subset and
// is_superset that does just what I need
fn check_overlap(assignments: &Vec<usize>) -> bool {
    let elf_one: HashSet<usize> = (assignments[0]..=assignments[1]).collect();
    let elf_two: HashSet<usize> = (assignments[2]..=assignments[3]).collect();

    if elf_one.is_subset(&elf_two) | elf_one.is_superset(&elf_two) {
        return true
    } else {
        return false
    }
}


fn main() {
    let mut overlaps: usize = 0;
    
    // I'm going to try a different way of reading files. It's not as robust as the previous method, but it doesn't need to be
    let data = fs::read_to_string("./input.txt").expect("Unable to read file.");
    for line in data.lines() {
        let assignments: Vec<usize> = line.split([',', '-'])
                                          .flat_map(|a| a.parse::<usize>()) // Can't parse inline otherwise
                                          .collect();
        
        if check_overlap(&assignments) { overlaps += 1 };
    }

    println!("The number of completely overlapping assignments is: {overlaps}");
}
