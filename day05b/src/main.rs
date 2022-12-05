use std::fs;

/*
 * It's the same game, but this time the Elves are using CrateMover-9001; an
 * upgrade from what we thought we had. This model moves entire stacks at a
 * time, so they retain their order when moved.
 * 
 * Coding strategy
 * ===============
 * 
 * LOL! Ignore that noise. I'm using an intermediate stack.
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

fn hanoi_pt1(stack_set: &mut Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) {
    let num_inst = instructions.len();
    for e in 0..num_inst {
        for _ in 0..instructions[e][0] {
            let reloc = stack_set[instructions[e][1]].pop().unwrap();
            stack_set[instructions[e][2]].push(reloc);
        }
    }
}

fn hanoi_pt2(stack_set_clone: &mut Vec<Vec<char>>, instructions: &Vec<Vec<usize>>) {
    let num_inst = instructions.len();
    for e in 0..num_inst {
        let mut intermediate: Vec<char> = Vec::new();
        for _ in 0..instructions[e][0] {
            intermediate.push(stack_set_clone[instructions[e][1]].pop().unwrap());
        }
        for _ in 0..instructions[e][0] {
            stack_set_clone[instructions[e][2]].push(intermediate.pop().unwrap());
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

    let mut stack_set_clone = stack_set.clone();

    // And finally on to the sorting
    hanoi_pt1(&mut stack_set, &instructions);
    hanoi_pt2(&mut stack_set_clone, &instructions);

    for a in 0..stacks {
        let l = stack_set[a].len();
        println!("Stack {} has length {} and has crate {} at the top.  Cloned stack {} has length {} and has crate {} at the top.", a, l, stack_set[a][l-1], a, l, stack_set_clone[a][l-1])
    }
    
}
