use std::{fs, str::FromStr};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");

    let pairs: Vec<Pair> = contents
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|row| Pair::from_str(row).unwrap())
        .collect();
    println!(
        "fully contained pairs {}",
        count_fully_contained_pairs(&pairs)
    );
    println!("overlapping pairs {}", count_overlapping(&pairs));
}

#[derive(Debug, PartialEq, Eq)]
struct Pair(Range, Range);

impl Pair {
    fn fully_contains(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

#[derive(Debug)]
struct PairParseError;

impl FromStr for Pair {
    type Err = PairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<Range> = s
            .split(",")
            .map(|range_str| {
                let numbers: Vec<u32> = range_str
                    .split("-")
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect();
                Range(numbers[0], numbers[1])
            })
            .collect();

        Ok(Pair(ranges[0], ranges[1]))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range(u32, u32);

impl Range {
    fn contains(&self, range: &Range) -> bool {
        self.0 <= range.0 && self.1 >= range.1
    }

    fn overlaps(&self, range: &Range) -> bool {
        fn boundaries_contained(a: &Range, b: &Range) -> bool {
            (a.0 >= b.0 && a.1 <= b.1) || (a.1 >= b.0 && a.1 <= b.1)
        }

        boundaries_contained(self, range) || boundaries_contained(range, self)
    }
}

fn count_overlapping(pairs: &Vec<Pair>) -> usize {
    pairs.iter().filter(|p| p.overlaps()).count()
}

fn count_fully_contained_pairs(pairs: &Vec<Pair>) -> usize {
    pairs.iter().filter(|p| p.fully_contains()).count()
}

#[test]
fn overlaps_test() {
    let input = vec![
        Pair(Range(2, 4), Range(6, 8)),
        Pair(Range(2, 3), Range(4, 5)),
        Pair(Range(5, 7), Range(7, 9)),
        Pair(Range(2, 8), Range(3, 7)),
        Pair(Range(6, 6), Range(4, 6)),
        Pair(Range(2, 6), Range(4, 8)),
    ];

    assert_eq!(4, count_overlapping(&input))
}

#[test]
fn pair_from_str() {
    let input = "30-31,2-31";

    assert_eq!(
        Pair(Range(30, 31), Range(2, 31)),
        Pair::from_str(input).unwrap()
    );
}

#[test]
fn count_fully_contained_pairs_test() {
    let input = vec![
        Pair(Range(2, 4), Range(6, 8)),
        Pair(Range(2, 3), Range(4, 5)),
        Pair(Range(5, 7), Range(7, 9)),
        Pair(Range(2, 8), Range(3, 7)),
        Pair(Range(6, 6), Range(4, 6)),
        Pair(Range(2, 6), Range(4, 8)),
    ];

    assert_eq!(2, count_fully_contained_pairs(&input))
}

#[test]
fn pair_fully_contains_test() {
    struct TestCase {
        input: Pair,
        want: bool,
    }
    let test_cases = vec![
        TestCase {
            input: Pair(Range(2, 4), Range(6, 8)),
            want: false,
        },
        TestCase {
            input: Pair(Range(2, 3), Range(4, 5)),
            want: false,
        },
        TestCase {
            input: Pair(Range(5, 7), Range(7, 9)),
            want: false,
        },
        TestCase {
            input: Pair(Range(2, 8), Range(3, 7)),
            want: true,
        },
        TestCase {
            input: Pair(Range(6, 6), Range(4, 6)),
            want: true,
        },
        TestCase {
            input: Pair(Range(2, 6), Range(4, 8)),
            want: false,
        },
    ];

    test_cases
        .iter()
        .for_each(|tc| assert_eq!(tc.want, tc.input.fully_contains()))
}
