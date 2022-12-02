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

                total_points = total_points + elf_play.result(you_play_string)?;
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
struct GeneralError;

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "General lazy error")
    }
}

impl error::Error for GeneralError {}

impl FromStr for Play {
    type Err = GeneralError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(GeneralError)
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

    fn result(&self, your_move: &str) -> Result<u32, Box<dyn error::Error>> {
        match (self, your_move) {
            (Play::Rock, "X") => Ok(Play::LOSE_POINTS + Play::Scissors.value()),
            (Play::Rock, "Y") => Ok(Play::DRAW_POINTS + Play::Rock.value()),
            (Play::Rock, "Z") => Ok(Play::WIN_POINTS + Play::Paper.value()),
            (Play::Paper, "X") => Ok(Play::LOSE_POINTS + Play::Rock.value()),
            (Play::Paper, "Y") => Ok(Play::DRAW_POINTS + Play::Paper.value()),
            (Play::Paper, "Z") => Ok(Play::WIN_POINTS + Play::Scissors.value()),
            (Play::Scissors, "X") => Ok(Play::LOSE_POINTS + Play::Paper.value()),
            (Play::Scissors, "Y") => Ok(Play::DRAW_POINTS + Play::Scissors.value()),
            (Play::Scissors, "Z") => Ok(Play::WIN_POINTS + Play::Rock.value()),
            _ => Err(Box::new(GeneralError))
        }
    }
}