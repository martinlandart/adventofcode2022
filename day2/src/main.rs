use std::{fs, str::FromStr};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    let rounds: Vec<Round> = contents
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| Round::from_str(s).unwrap())
        .collect();

    println!("Score {}", calculate_score(rounds))
}

fn calculate_score(rounds: Vec<Round>) -> u32 {
    rounds
        .into_iter()
        .map(|x| x.calculate_score())
        .reduce(|accum, item| accum + item)
        .unwrap()
}

#[derive(Clone, Debug, PartialEq, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    fn score(&self) -> u32 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Response {
    Lose,
    Draw,
    Win,
}
impl Response {
    fn score(&self) -> u32 {
        match self {
            Response::Lose => 0,
            Response::Draw => 3,
            Response::Win => 6,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Round {
    opponent: Play,
    response: Response,
}

impl Round {
    fn calculate_score(&self) -> u32 {
        let shape_to_play: Play = match (&self.opponent, &self.response) {
            (x, Response::Draw) => *x,
            (Play::Rock, Response::Win) => Play::Paper,
            (Play::Paper, Response::Win) => Play::Scissors,
            (Play::Scissors, Response::Win) => Play::Rock,
            (Play::Rock, Response::Lose) => Play::Scissors,
            (Play::Paper, Response::Lose) => Play::Rock,
            (Play::Scissors, Response::Lose) => Play::Paper,
        };

        self.response.score() + shape_to_play.score()
    }
}

#[derive(Debug)]
struct RoundParseError;

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().count() == 0 {
            println!("broken line {}", s);
            return Err(RoundParseError);
        }
        let mut chars = s.chars();

        let play = match chars.next().unwrap() {
            'A' => Ok(Play::Rock),
            'B' => Ok(Play::Paper),
            'C' => Ok(Play::Scissors),
            _ => Err("unknown play"),
        };
        chars.next();

        let response = match chars.next().unwrap() {
            'X' => Ok(Response::Lose),
            'Y' => Ok(Response::Draw),
            'Z' => Ok(Response::Win),
            _ => Err("unknown response"),
        };

        Ok(Round {
            opponent: play.unwrap(),
            response: response.unwrap(),
        })
    }
}

#[test]
fn rock_paper_scissorcs_test() {
    assert_eq!(
        Round {
            opponent: Play::Rock,
            response: Response::Draw
        }
        .calculate_score(),
        4
    );
}

#[test]
fn score_test() {
    assert_eq!(Response::Win.score(), 6);
    assert_eq!(Response::Draw.score(), 3);
    assert_eq!(Response::Lose.score(), 0);
}

#[test]
fn calculate_score_test() {
    let input = [
        Round {
            opponent: Play::Rock,
            response: Response::Draw,
        },
        Round {
            opponent: Play::Paper,
            response: Response::Lose,
        },
        Round {
            opponent: Play::Scissors,
            response: Response::Win,
        },
    ];

    let got = calculate_score(input.to_vec());

    assert_eq!(got, 12)
}

#[test]
fn parse_input_test() {
    assert_eq!(
        Round::from_str("C X").unwrap(),
        Round {
            opponent: Play::Scissors,
            response: Response::Lose
        }
    )
}
