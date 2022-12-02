use std::{fs::File, io::{self, BufRead}, path::Path};

// I have a feeling I will be using this function for every day's challenge, so will keep this in a library instead.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}