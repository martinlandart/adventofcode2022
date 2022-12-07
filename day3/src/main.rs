use std::{char, collections::HashSet, fs};

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data").expect("failed to read file");

    let rucksacks: Vec<&str> = contents.split("\n").filter(|s| s.len() > 0).collect();

    println!(
        "sum of common item priorities {}",
        sum_priorities(&rucksacks)
    );

    let groups: Vec<Vec<&str>> = rucksacks.chunks(3).map(|s| s.into()).collect();

    println!("sum of badges priorities {}", sum_badges_priorities(groups))
}

#[derive(Debug, PartialEq)]
struct Item {
    itype: char,
}

impl Item {
    fn priority(&self) -> u32 {
        let ascii = i64::from(self.itype as u32);
        if ascii > 48 * 2 {
            (ascii - 48 - 48) as u32
        } else if ascii > 48 {
            (ascii - 38) as u32
        } else {
            ascii as u32
        }
    }
}

fn find_badge(group: &[&str]) -> Item {
    let groups: Vec<HashSet<char>> = group
        .iter()
        .map(|g| g.chars())
        .map(|chars| {
            let mut group_set = HashSet::new();
            chars.for_each(|c| {
                let _ = group_set.insert(c);
            });

            group_set
        })
        .collect();

    Item {
        itype: *groups[0]
            .iter()
            .find(|item| groups[1].contains(*item) && groups[2].contains(*item))
            .unwrap(),
    }
}

fn common_item(items: &str) -> char {
    let half_index = items.len() / 2;

    let mut first_compartment = HashSet::new();

    items[..half_index].chars().for_each(|item| {
        let _ = first_compartment.insert(item);
    });

    if let Some(common_item) = &items[half_index..]
        .chars()
        .find(|item| first_compartment.contains(item))
    {
        *common_item
    } else {
        panic!("{}", items)
    }
}

fn sum_badges_priorities(groups: Vec<Vec<&str>>) -> u32 {
    groups
        .iter()
        .map(|g| find_badge(g).priority())
        .reduce(|accum, current| accum + current)
        .unwrap()
}

fn sum_priorities(rucksacks: &Vec<&str>) -> u32 {
    rucksacks
        .iter()
        .map(|rucksack| {
            Item {
                itype: common_item(rucksack),
            }
            .priority()
        })
        .reduce(|accum, item| accum + item)
        .unwrap()
}

#[test]
fn sum_badges_priorities_test() {
    let input = vec![
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ],
        vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ],
    ];
    let want = 70;

    assert_eq!(sum_badges_priorities(input), want)
}

#[test]
fn sum_priorities_test() {
    let input = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    assert_eq!(sum_priorities(&input), 157)
}

#[test]
fn find_badge_test() {
    struct TestCase<'a> {
        input: Vec<&'a str>,
        want: char,
    }
    let inputs = vec![
        TestCase {
            input: vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ],
            want: 'r',
        },
        TestCase {
            input: vec![
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ],
            want: 'Z',
        },
    ];

    inputs
        .iter()
        .for_each(|tc| assert_eq!(find_badge(&tc.input).itype, tc.want))
}

#[test]
fn priorities_test() {
    assert_eq!(Item { itype: 'a' }.priority(), 1);
    assert_eq!(Item { itype: 'b' }.priority(), 2);
    assert_eq!(Item { itype: 'c' }.priority(), 3);
    assert_eq!(Item { itype: 'z' }.priority(), 26);

    assert_eq!(Item { itype: 'A' }.priority(), 27);
    assert_eq!(Item { itype: 'B' }.priority(), 28);
    assert_eq!(Item { itype: 'C' }.priority(), 29);
    assert_eq!(Item { itype: 'Z' }.priority(), 52);
}

#[test]
fn common_item_test() {
    assert_eq!(common_item("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    assert_eq!(common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
    assert_eq!(common_item("PmmdzqPrVvPwwTWBwg"), 'P');
    assert_eq!(common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
    assert_eq!(common_item("ttgJtRGJQctTZtZT"), 't');
    assert_eq!(common_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
}
