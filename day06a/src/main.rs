use std::collections::HashSet;

/*
 * You have been given a malfunctioning handheld communication device. To
 * communicate with other Elves it needs to lock on to their signal which is a
 * seemingly random stream of characters. To fix the device you neeed to add a
 * subroutine that detects a start of packet marker, which is a sequence of 4
 * different characters.
 * 
 * Coding strategy
 * ===============
 * 
 * The input file is a single byte stream. The answer is just the first start
 * of packet marker? Having a 'sliding window' over that stream would require
 * an iterator. We can use the windows() method on slice, and take a hash of
 * the window, get the length, and if it's equal to 4, then we're good.
 */

fn main() {
    // Going to try and seriously up my game with this approach. I was going to
    // give this a go yesterday as well, but that parsing was giving me a headache
    let data = include_bytes!("../input.txt");
    let (idx, _) = data.windows(4)
        .enumerate() // Returns (index, value)
        .filter(|(_, char)| HashSet::<u8>::from_iter(char.iter().cloned()).len() == 4)
        .next()
        .unwrap();
    println!("The packet marker is at position {}", idx + 4);
}
