use std::{collections::HashSet};
use day03a::read_lines;

/*
 * The file to be read contains the contents of rucksacks, with each line being
 * an individual rucksack, with letters representing an item of some sort.
 * Letters are case sensitive. The rucksacks have two compartments, and the
 * contents fit into both evenly, but there are some misplaced items that are
 * in both compartments. Items that appear in both compartments need to be
 * identified, and this is done according to priority (I will assume there may
 * be multiple items that have been placed in both compartments). Priorities of
 * items are in alphabetical order, with lowercase having higher priority than
 * uppercase, such that:
 * 
 * Items 'a' through 'z' have priorities of 1 through 26.
 * Items 'A' through 'Z' have priorities of 27 through 52.
 * 
 * Coding strategy
 * ===============
 * 
 * Each line in the file is read as a string. Each string will be split in half
 * and have the characters in each string slice counted. I won't assume
 * misplaced items will have one in each compartment, as there may well be any
 * number in either, i.e. 1x 'a' in one compartment, 2x 'a' in the other. It
 * should also be assumed that there may be multiple item types that have been
 * placed in both compartments, but I will assume that *only* the highest
 * priority should be counted. The best method for counting would probably be
 * to use hashsets, and then compare the two to see if any keys match.
 * 
 * Priority sorting could be done with an enum, but that will be a minumum of
 * 104 just in definitions, which is not a very DRY approach. Being lazy, I
 * would rather generate a hashmap, and get() the priority value that way.
 * 
 * Addendum
 * ========
 * 
 * Actually on further reading, I could get the priorities directly from the
 * characters using their ASCII/Unicode values:
 * 
 * 'a' = 97
 * 'A' = 65
 * 
 * So the priority of 'a' is its value - 96, and for 'A' 65 - 27 = 38.
 */

fn priority(item: &char) -> usize {
    // Must use raw pointer otherwise casting is invalid
    if item.is_lowercase() {
        *item as usize - 96
    } else {
        *item as usize - 38
    }
}

fn main() {
    let mut total_score: usize = 0;
    
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(rucksack) = line {
                let first_half: HashSet<char> = rucksack[..rucksack.len()/2].chars().collect();
                let second_half: HashSet<char> = rucksack[rucksack.len()/2..].chars().collect();

                // Fortunately HashSet has a fn in its impl that finds values in both self and other
                let duplicates: HashSet<_> = first_half.intersection(&second_half).collect();
                // From testing it appears there only is one duplicate in both halves

                // Ehhhh... iterate over one. This is ridiculous
                for item in duplicates {
                    total_score += priority(item);
                }
            }
        }
    }

    println!("The sum of priorities is: {total_score}");
}
