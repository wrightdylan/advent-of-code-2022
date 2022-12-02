use day02a::read_lines;

/*
 * The file to be read contains the strategy of the game. Each row is a round,
 * and is split into two columns. The first column is the predicted play of the
 * opponent, and the second column is the response. The shapes have their own
 * scores, so we could maybe use an enum to assign scores to shapes, such that:
 * 
 * A, X = Rock     = 1 pt
 * B, Y = Paper    = 2 pts
 * C, Z = Scissors = 3 pts
 * 
 * Score is calculated on the shape selected (X, Y, and Z) plus the outcome:
 * 
 * loss = 0 pts
 * draw = 3 pts
 * win  = 6 pts
 * 
 * Coding strategy
 * ===============
 * 
 * Each line in the file is read as a string. Each string consists of a char,
 * space, and char. Score keeping can be kept very simple with one variable.
 * The value of the response can be added immediately. Opponent play and
 * response can be compared using an impl.
 */

enum Shape {
    Rock,
    Paper,
    Scissors
}
// No Lizard or Spock

impl Shape {
    fn play(&self, opponent: &Self) -> usize {
        self.turn_point() + self.round_point(opponent)
    }

    fn turn_point(&self) -> usize {
        match self {
            Shape::Rock     => 1,
            Shape::Paper    => 2,
            Shape::Scissors => 3,
        }
    }

	// Set conditions mapping each outcome
    fn round_point(&self, opponent: &Self) -> usize {
        match (self, opponent) {
            (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Paper, Shape::Rock) | (Shape::Rock, Shape::Scissors) | (Shape::Scissors, Shape::Paper) => 6,
            _ => 0,
        }
    }
}

fn main() {
    let mut total_score: usize = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(round) = line {
                let mut turn = round.split_ascii_whitespace();
                let opponent_turn = match turn.next().expect("valid input") {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                    _   => panic!("invalid input")
                };
                let player_turn = match turn.next().expect("valid input") {
                    "X" => Shape::Rock,
                    "Y" => Shape::Paper,
                    "Z" => Shape::Scissors,
                    _   => panic!("invalid input")
                };
                
                total_score += player_turn.play(&opponent_turn);
            }
            
        }
    }

    println!("Total score by following the strategy guide: {total_score}");
}
