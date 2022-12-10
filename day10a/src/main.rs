/*
 * The device has a CRT and simple CPU driven by a precision clock. The CPU has
 * a single register, X, which starts at value 1, and takes two instructions:
 * 
 * - addx V; takes 2 cycles to complete, and increases X by V (can be negative)
 * - noop; takes 1 cycle and does nothing
 * 
 * Signal strength is calculated by the cycle number multiplied by the value of
 * the X register. This is done on the 20th cycle, and every 40th cycle therea-
 * fter. Find the sum of signal strengths at cycles 20, 60, 100, 140, 180, 220.
 * 
 * Coding strategy
 * ===============
 * 
 * Parsing strings has become quite easy at this point. If the input begins with
 * an 'n', nothing happens, if it begins with an 'a', split the string and get
 * the value in the second half.
 * 
 * Counting cycles... there's multiple ways of doing this. You could use a Hash
 * Map containing the cycle number and value of X, or a vector that simply con-
 * tains the value of X, using the index for cycle. The HashMap or vector could
 * be checked afterwards at specific points to get X values. A more elegant ap-
 * proach is to calculate from within the instruction iteration itself, and have
 * a sort of tick-tock/flip-flop register for every 20 cycles, i.e. the first
 * tick at 20 cycles determines the value of X, the next 20 (40th cycle) is a 
 * tock and skips reading, and the next tick is on 60 and reads the value, etc,
 * and is easily accomplished by modulo 40 == 20. Since it's not possible to 
 * skip cycles within an iterator, it will be necessary to simulate a clock
 * within the iterator and check the cycle number every cycle, so for addx this
 * needs to be checked twice.
 */

fn parse_instructions(data: &str) -> Vec<(&str, isize)> {
    data.lines()
        .map(|line| {
            let (opcode, value) = match line {
                "noop" => ("noop", 0),
                _ => {
                    let (op_str, val_str) = line.split_once(" ").unwrap();
                    let opcode = op_str;
                    let value = val_str.parse::<isize>().unwrap();
                    (opcode, value)
                }
            };
            (opcode, value)
        })
        .collect()
}

fn process(instructions: Vec<(&str, isize)>, sig_str: &mut Vec<isize>) {
    let mut cycle: isize = 1; // has to start at 1 for the math to work
    let mut x: isize = 1;
    for line in instructions {
        check(cycle, x, sig_str);
        if line.0 == "noop" {
            cycle += 1;
        } else {
            cycle += 1;

            check(cycle, x, sig_str);
            x += line.1;
            cycle += 1;
        }
    }
}

fn check(cycle: isize, x: isize, sig_str: &mut Vec<isize>) {
    if cycle % 40 == 20 {
        sig_str.push(cycle * x);
    }
    println!("{} {} {:?}", cycle, x, sig_str); // Remove
}

fn main() {
    let mut sig_str: Vec<isize> = Vec::new();
    let data = include_str!("../input.txt");
    let instructions = parse_instructions(data);
    process(instructions, &mut sig_str);
    let value: isize = sig_str.iter().sum();
    println!("Sum of signal strengths = {}.", value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor() {
        let mut sig_str: Vec<isize> = Vec::new();
        let data = include_str!("../test.txt");
        let instructions = parse_instructions(data);
        process(instructions, &mut sig_str);
        let value: isize = sig_str.iter().sum();
        assert!(value == 13140);
    }
}
