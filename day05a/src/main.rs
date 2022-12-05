use std::fs;

/*
 * This is basically a larger version of Tower of Hanoi. Oh joy...
 * The file contains a map of stack, and a list of moves. The map of stacks
 * show how many stacks and which crates are where and in which order. In the
 * text file this is arranged as in columnar format, with the base at the
 * bottom. The list of moves underneath in a line by line format. There are 9
 * stacks, initially at 8 items deep. Line 9 of the file contains the indices
 * of the stacks. Move instructions begin on line 11.
 * 
 * Coding strategy
 * ===============
 * 
 * First off, I refuse to cheat by hardcoding the stacks. This is supposed to
 * be a learning experience for me, so I will attempt to read the file as
 * intended. Because the crates are provided in a physical representation, i.e.
 * in columns and starting from the bottom, parsing this section will be a huge
 * challenge. Possible solution for parsing the stack: read the string and
 * iterate by char. Crates are built using a letter surrounded by brackets,
 * i.e. [G], and there is one space between crates. The first element (index 0)
 * of the string is therefore the side wall of the first crate, so the crate
 * name is on the second element. The next crate name is four elements on.
 * 
 * Start by calculating how many vectors are needed from the length of the
 * string. Push any non-whitespace character into the appropriate vector. Stop
 * reading when numbers are encountered. Reverse the vectors to get the correct
 * order. This is a more robust reading method that will accept stacks of any
 * dimension. Group the whole lot into a vector of vectors for easier indexing.
 * Part 2 will parse the instructions, so skip reading lines until encountering
 * the first move (char 'm'). When parsing instructions, only numerals are
 * needed since the first number indicates howmany times the operation is to be
 * repeated (sounds likea for loop), the second number is the vector from which
 * to move a vectored vector's value, and the third is the vector the value is
 * to be moved. Lots of pop() and push() operations in this.
 */

fn populate_stacks(stack_set: &mut Vec<Vec<char>>, line: &str, cols: &usize) {
    let parser_vector: Vec<char> = line.chars().collect();
    for c in (1..cols-1).step_by(4) {
        if parser_vector[c] != ' ' {
            let stack = (c - 1) / 4;
            stack_set[stack].push(parser_vector[c]);
        }
    }
}

fn hanoi(stack_set: &mut Vec<Vec<char>>, instructions: Vec<Vec<usize>>) {
    let num_inst = instructions.len();
    for e in 0..num_inst {
        for _ in 0..instructions[e][0] {
            let reloc = stack_set[instructions[e][1]].pop().unwrap();
            stack_set[instructions[e][2]].push(reloc);
        }
    }
}

fn main() {
    let mut run_once = false;
    let mut stack_set: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();
    let mut cols = 0;
    let mut stacks = 0;
    let data = fs::read_to_string("./input.txt").expect("Unable to read file.");
    for line in data.lines() {
        // Initialise empty stacks
        if !run_once {
            cols = line.len();
            stacks = ((cols - 3) / 4) + 1;
            for _ in 0..stacks {
                stack_set.push(Vec::new());
            };
            run_once = true;
        }

        // The two sections of the input file need to be parsed differently
        if !line.is_empty() && line.chars().nth(0).unwrap() != 'm' && line.chars().nth(1).unwrap() != '1' {
            populate_stacks(&mut stack_set, line, &cols);
        } else if !line.is_empty() && line.chars().nth(0).unwrap() == 'm' {
            let mut instruction: Vec<usize> = line.split_ascii_whitespace().flat_map(|a| a.parse::<usize>()).collect();
            instruction[1] -= 1; // Indexing corrections
            instruction[2] -= 1; // Indexing corrections
            instructions.push(instruction);
        }
        // OMG that parsing... there has GOT to be a better way. And I really didn't want to build an instructions vector
    }

    // Uno reverse card
    for s in 0..stacks {
        stack_set[s].reverse();
    }
    
    // And finally on to the sorting
    hanoi(&mut stack_set, instructions);

    for a in 0..stacks {
        let l = stack_set[a].len();
        println!("Stack {} has length {} and has crate {} at the top.", a, l, stack_set[a][l-1])
    }
    println!("{:?}", stack_set);
}
