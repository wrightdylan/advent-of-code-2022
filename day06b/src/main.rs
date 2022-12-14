use std::{str, collections::HashSet};

/*
 * The communications device is now detecting packets, but still isn't working.
 * It also needs to look for messages, and the start of message marker is like
 * the start of packet marker, but consists of 14 distinct characters instead
 * of 4.
 * 
 * Coding strategy
 * ===============
 * 
 * Basically this is the exact same as before, but with a little refactoring to
 * give both answers.
 */

fn find_header(data: &[u8], size: usize) -> (usize, &str) {
    let (idx, header) = data.windows(size)
        .enumerate()
        .filter(|(_, char)| HashSet::<u8>::from_iter(char.iter().cloned()).len() == size)
        .next()
        .unwrap();
    
    (idx + size, str::from_utf8(header).unwrap())
}

fn main() {
    let data = include_bytes!("../input.txt");
    let packet = find_header(data, 4);
    let message = find_header(data, 14);

    println!("The packet marker is at position {}, and consist of {}.", packet.0, packet.1);
    println!("The message marker is at position {}, and consists of {}.", message.0, message.1);
}
