use std::{io::{self, BufRead}, fs::File, path::Path};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Read the file line by line
    /*let file = File::open("./input.txt").unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        println!("{line:?}"); // test output
    }*/

    // This is the better, idiomatic method
    let mut current: u32 = 0;
    let mut largest: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            // Parse lines which are known to be either a positive integer or newline
            // Elves are separated in the input file by newlines
            // Use an iterator to count calories per Elf, and use a vector to store totals
            // Since this is iterative, we can simultaneously keep tabs on the largest total
            // Exercise does not seem to care which Elf it is, just how many calories he's holding
            if let Ok(value) = line {
                if value.is_empty() {
                    if current > largest {
                        largest = current;
                    }
                    current = 0;
                } else {
                    // Convert those strings to numbers
                    current += value.parse::<u32>().unwrap();
                }
            }
            // Wow, that solution is so ugly it makes me want to puke. The vector wasn't necessary afterall.
        }
    }
    println!("Most calories: {}", &largest);
}
