use day02b::read_lines;

/*
 * The file to be read contains the strategy of the game, but as it turns out,
 * the strategy is not what was originally thought. As before, each row is a
 * round, and is split into two columns. The first column is the predicted play
 * of the opponent. As before, the shapes have their own scores, so we could
 * use the same enum to assign scores to shapes:
 * 
 * A = Rock     = 1 pt
 * B = Paper    = 2 pts
 * C = Scissors = 3 pts
 * 
 * The second column shows the desired outcome of the round, such that:
 * 
 * X = Lose
 * Y = Draw
 * Z = Win
 * 
 * Score is calculated on the shape played in response plus the outcome:
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
 * A new enum for the outcome will be needed, with a modification to the Shape 
 * impl.
 */

#[derive(Clone, Copy)]
 enum Shape {
    Rock,
    Paper,
    Scissors
}
// No Lizard or Spock

impl Shape {
    fn turn_point(&self) -> usize {
        match self {
            Shape::Rock     => 1,
            Shape::Paper    => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_to_lose(&self) -> Self {
        match self {
            Shape::Rock     => Shape::Scissors,
            Shape::Paper    => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    fn play_to_win(&self) -> Self {
        match self {
            Shape::Rock     => Shape::Paper,
            Shape::Paper    => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win  => 6
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
                let outcome = match turn.next().expect("valid input") {
                    "X" => Outcome::Lose,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    _   => panic!("invalid input")
                };
                
                // No need to work out the correct response to draw since it's the same
                let response = match outcome {
                    Outcome::Win  => opponent_turn.play_to_win(),
                    Outcome::Lose => opponent_turn.play_to_lose(),
                    _ => opponent_turn
                };

                // No need to calculate score on outcome since the outcome is already known
                total_score += response.turn_point() + outcome.score();
            }
            
        }
    }

    println!("Total score by correctly following the strategy guide: {total_score}");
}
