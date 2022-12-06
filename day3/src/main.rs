use std::{char, collections::HashSet, fs};

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data").expect("failed to read file");

    let rucksacks: Vec<&str> = contents.split("\n").filter(|s| s.len() > 0).collect();

    println!(
        "sum of common item priorities {}",
        sum_priorities(rucksacks)
    )
}

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

fn sum_priorities(rucksacks: Vec<&str>) -> u32 {
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
fn sum_priorities_test() {
    let input = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    assert_eq!(sum_priorities(input), 157)
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
    assert_eq!(common_item("ttgJtRGJQctTZtZ"), 't');
    assert_eq!(common_item("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
}
