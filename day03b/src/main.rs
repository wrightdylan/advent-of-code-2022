use std::{collections::HashSet};
use day03b::read_lines;

/*
 * The file to be read contains the contents of rucksacks, with each line being
 * an individual rucksack, with letters representing an item of some sort.
 * Elves are in groups of three and are represented by a badge. This is
 * determined by one item that is commonly carried by everyone in that group.
 * Sorting is made a little easier by the fact that at most two of the Elves
 * will be carrying any other item type. To make this even easier, the Elves
 * are already arranged in their groups of three in the file, so all that's
 * needed to do is to find the common item between three consecutive lines (no
 * bullshit involving scanning the entire dataset repeatedly).
 * 
 * Coding strategy
 * ===============
 * 
 * Each line in the file is read as a string. Functionality from the previous
 * part must be maintained, but a new hashset of the complete rucksack must be
 * created for each Elf. Create a vector of three hashsets and perform a chain
 * of intersections on the group, i.e. ((a int b) int c).
 */

fn priority(item: &char) -> usize {
    // Must use raw pointer otherwise casting is invalid
    if item.is_lowercase() {
        *item as usize - 96
    } else {
        *item as usize - 38
    }
}

// Group activities
fn elven_threesome(threesome: &Vec<HashSet<char>>) -> usize {
    // Conveniently BitAnd returns the intersection of self and rhs as a new HashSet
    let couple = &threesome[0] & &threesome[1];
    let triplicate = threesome[2].intersection(&couple).next().unwrap();
    priority(triplicate)
}

fn main() {
    let mut total_score: usize = 0;
    let mut total_group_score: usize = 0;
    
    // This is basically a hardcoded hash buffer *shrug*
    let mut threesome: Vec<HashSet<char>> = Vec::new();
    
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

                let elf_hash:HashSet<char> = rucksack.chars().collect();
                threesome.push(elf_hash);
                if threesome.len() == 3 {
                    total_group_score += elven_threesome(&threesome);
                    threesome.clear();
                };
            }
        }
    }

    println!("The sum of priorities is: {total_score}");
    println!("The sum of priorities from Elf groups is: {total_group_score}");
}
