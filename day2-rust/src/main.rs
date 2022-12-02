use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::error;
use std::fmt;

fn main() -> Result<(), Box<dyn error::Error>> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut total_points: u32 = 0;
    for line in reader.lines() {
        let line_res = line?;
        let line_input: Vec<&str> = line_res.split(" ").collect();
        match (line_input.get(0), line_input.get(1)) {
            (Some(elf_play_string), Some(you_play_string)) => {
                let elf_play = Play::from_str(elf_play_string)?;
                let you_play = Play::from_str(you_play_string)?;

                total_points = total_points + you_play.result(elf_play);
                total_points = total_points + you_play.value();
            }
            _ => ()
        }
    }

    Ok(println!("Total points: {total_points}"))
}

enum Play {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

impl FromStr for Play {
    type Err = EmptyVec;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Play::Rock),
            "B" | "Y" => Ok(Play::Paper),
            "C" | "Z" => Ok(Play::Scissors),
            _ => Err(EmptyVec)
        }
    }
}

impl Play {
    fn value(&self) -> u32 {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3
        }
    }

    const LOSE_POINTS:u32 = 0;
    const DRAW_POINTS: u32 = 3;
    const WIN_POINTS: u32 = 6;

    fn result(&self, opponent_play: Play) -> u32 {
        match (self, opponent_play) {
            (Play::Rock, Play::Rock) => Play::DRAW_POINTS,
            (Play::Rock, Play::Paper) => Play::LOSE_POINTS,
            (Play::Rock, Play::Scissors) => Play::WIN_POINTS,
            (Play::Paper, Play::Rock) => Play::WIN_POINTS,
            (Play::Paper, Play::Paper) => Play::DRAW_POINTS,
            (Play::Paper, Play::Scissors) => Play::LOSE_POINTS,
            (Play::Scissors, Play::Rock) => Play::LOSE_POINTS,
            (Play::Scissors, Play::Paper) => Play::WIN_POINTS,
            (Play::Scissors, Play::Scissors) => Play::DRAW_POINTS
        }
    }
}