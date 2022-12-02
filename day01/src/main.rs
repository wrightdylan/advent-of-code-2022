use std::{io::{self, BufRead}, fs::File, path::Path};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Read the file line by line
    let mut current: u32 = 0;
    let mut elves = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            // So apparently I will need a vector after all to store the values of the top three Elves
            // Possible approaches can be to push totals of each Elf onto a vector and then iterate through that vector for the top three
            // Or initialise a vector of three 0s and update that, pushing the smallest number out
            // The first is easier, but the second has a smaller memory footprint. Is that something I should be concerned about at this stage?
            if let Ok(value) = line {
                if value.is_empty() {
                    elves.push(current);
                    current = 0;
                } else {
                    current += value.parse::<u32>().unwrap();
                }
            }
        }
    }
    elves.sort();
    elves.reverse();
    let largest = elves.first().unwrap();
    let top_three: u32 = elves.iter().take(3).sum();
    println!("Most calories: {largest}");
    println!("Calories held by top three Elves: {top_three}");
    // Janky, but it works!
}
