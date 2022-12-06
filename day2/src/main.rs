use std::{fs, str::FromStr};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    println!("{}", contents);

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

#[derive(Clone, Debug, PartialEq)]
enum Play {
    A,
    B,
    C,
}

#[derive(Clone, Debug, PartialEq)]
enum Response {
    X,
    Y,
    Z,
}
impl Response {
    fn score(&self) -> u32 {
        match self {
            Response::X => 1,
            Response::Y => 2,
            Response::Z => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum RPSResult {
    Win,
    Lose,
    Tie,
}

impl RPSResult {
    fn score(&self) -> u32 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Tie => 3,
            RPSResult::Lose => 0,
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
        self.result().score() + self.response.score()
    }

    fn result(&self) -> RPSResult {
        match self {
            Round {
                opponent: Play::A,
                response: Response::Y,
            }
            | Round {
                opponent: Play::B,
                response: Response::Z,
            }
            | Round {
                opponent: Play::C,
                response: Response::X,
            } => RPSResult::Win,
            Round {
                opponent: Play::A,
                response: Response::X,
            }
            | Round {
                opponent: Play::B,
                response: Response::Y,
            }
            | Round {
                opponent: Play::C,
                response: Response::Z,
            } => RPSResult::Tie,
            _ => RPSResult::Lose,
        }
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
            // return;
        }
        let mut chars = s.chars();

        let play = match chars.next().unwrap() {
            'A' => Ok(Play::A),
            'B' => Ok(Play::B),
            'C' => Ok(Play::C),
            _ => Err("unknown play"),
        };
        chars.next();

        let response = match chars.next().unwrap() {
            'X' => Ok(Response::X),
            'Y' => Ok(Response::Y),
            'Z' => Ok(Response::Z),
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
            opponent: Play::A,
            response: Response::Y
        }
        .result(),
        RPSResult::Win
    );
    assert_eq!(
        Round {
            opponent: Play::B,
            response: Response::X
        }
        .result(),
        RPSResult::Lose
    );
    assert_eq!(
        Round {
            opponent: Play::C,
            response: Response::Z
        }
        .result(),
        RPSResult::Tie
    );
}

#[test]
fn score_test() {
    assert_eq!(RPSResult::Win.score(), 6);
    assert_eq!(RPSResult::Tie.score(), 3);
    assert_eq!(RPSResult::Lose.score(), 0);
}

#[test]
fn calculate_score_test() {
    let input = [
        Round {
            opponent: Play::A,
            response: Response::Y,
        },
        Round {
            opponent: Play::B,
            response: Response::X,
        },
        Round {
            opponent: Play::C,
            response: Response::Z,
        },
    ];

    let got = calculate_score(input.to_vec());

    assert_eq!(got, 15)
}

#[test]
fn parse_input_test() {
    assert_eq!(
        Round::from_str("C X").unwrap(),
        Round {
            opponent: Play::C,
            response: Response::X
        }
    )
}
