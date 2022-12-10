/*
 * The X register controls the horizontal position of a sprite, and the sprite
 * is 3 pixels wide. The screen is 40 x 6 pixels. The left-most pixel is 0 and
 * the right-most is 39. Pixels are lit by raster scanning. The CRT is tied to
 * clock cycles as well, and draws one pixel per cycle. If the sprite is posit-
 * ioned such that one of its three pixels is the pixel currently being drawn,
 * the screen produces a lit pixel (#); otherwise, the screen leaves the pixel
 * dark (.)(.)
 * 
 * Coding strategy
 * ===============
 * 
 * Given my desire to run both parts in part B, and the number of variables th-
 * at will be passed around, it may be better to refactor. We'll see if that's
 * necessary. Going back to the coding solution in part A, it looks like using
 * a vector to store the state of the cpu is necessary after all. A single ext-
 * ra vector is all that is needed to store the serialised raster image.
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

fn process(instructions: Vec<(&str, isize)>, sig_str: &mut Vec<isize>, raster: &mut Vec<char>) {
    let mut cycle: isize = 1;
    let mut x: isize = 1;
    for line in instructions {
        check(cycle, x, sig_str, raster);
        if line.0 == "noop" {
            cycle += 1;
        } else {
            cycle += 1;

            check(cycle, x, sig_str, raster);
            x += line.1;
            cycle += 1;
        }
    }
}

fn check(cycle: isize, x: isize, sig_str: &mut Vec<isize>, raster: &mut Vec<char>) {
    if cycle % 40 == 20 {
        sig_str.push(&cycle * x);
    }
    
    // Gives the screen width, needed to serialise
    let pixel = (&cycle - 1) % 40;

    // At the start of screen draw, pixel = 0 and sprite (x) = 1. The sprite is
    // 3 pixels wide. If the sprite is stationary, then on the next cycle pixel
    // = 1, sprite = 1 still. Next cycle pixel = 2, sprite = 1. So when the two
    // overlap it will be when -1 <= (pixel - sprite) <= 1.
    if (pixel - x) <= 1 && (pixel - x) >= -1 {
        raster.push('#');
    } else {
        raster.push('.');
    }
}

// Deserialise stored raster
fn video(raster: Vec<char>) {
    for v in 0..6 {
        let scan: String = raster[40*v..40*v+40].iter().collect();
        println!("{}", scan);
    }
}

fn main() {
    let mut sig_str: Vec<isize> = Vec::new();
    let mut raster: Vec<char> = Vec::new();
    let data = include_str!("../input.txt");
    let instructions = parse_instructions(data);
    process(instructions, &mut sig_str, &mut raster);
    let value: isize = sig_str.iter().sum();
    println!("Sum of signal strengths = {}.", value);
    video(raster);
}
