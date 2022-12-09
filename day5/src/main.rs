// [T]             [P]     [J]
// [F]     [S]     [T]     [R]     [B]
// [V]     [M] [H] [S]     [F]     [R]
// [Z]     [P] [Q] [B]     [S] [W] [P]
// [C]     [Q] [R] [D] [Z] [N] [H] [Q]
// [W] [B] [T] [F] [L] [T] [M] [F] [T]
// [S] [R] [Z] [V] [G] [R] [Q] [N] [Z]
// [Q] [Q] [B] [D] [J] [W] [H] [R] [J]
//  1   2   3   4   5   6   7   8   9

use std::fs;

fn main() {
    let mut cargo = Cargo {
        stacks: vec![
            Stack::from_crates(vec!['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T']),
            Stack::from_crates(vec!['Q', 'R', 'B']),
            Stack::from_crates(vec!['B', 'Z', 'T', 'Q', 'P', 'M', 'S']),
            Stack::from_crates(vec!['D', 'V', 'F', 'R', 'Q', 'H']),
            Stack::from_crates(vec!['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P']),
            Stack::from_crates(vec!['W', 'R', 'T', 'Z']),
            Stack::from_crates(vec!['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J']),
            Stack::from_crates(vec!['R', 'N', 'F', 'H', 'W']),
            Stack::from_crates(vec!['J', 'Z', 'T', 'Q', 'P', 'R', 'B']),
        ],
    };

    let contents = fs::read_to_string("./data").expect("failed to read file");

    let movements: Vec<(usize, usize, usize)> = contents
        .split("\n")
        .skip(10)
        .filter(|s| s.len() > 0)
        .map(|row| parse_movements(row))
        .collect();

    let mut cargo_copy = cargo.clone();
    movements
        .iter()
        .for_each(|movement| cargo.move_cargo_9000(movement.0, movement.1, movement.2));
    println!(
        "top crates 9000 {}",
        cargo.top_crates().iter().collect::<String>()
    );

    movements
        .iter()
        .for_each(|movement| cargo_copy.move_cargo_9001(movement.0, movement.1, movement.2));
    println!(
        "top crates 9001 {}",
        cargo_copy.top_crates().iter().collect::<String>()
    )
}

#[derive(Clone, Debug)]
struct Cargo<T> {
    stacks: Vec<Stack<T>>,
}

impl<T: Copy> Cargo<T> {
    fn move_cargo_9000(&mut self, number: usize, from: usize, to: usize) {
        let n = number;
        let popped: Vec<T> = (0..n).map(|_| self.stacks[from - 1].pop()).collect();

        (0..n).for_each(|i| self.stacks[to - 1].push(popped[i]));
    }

    fn move_cargo_9001(&mut self, number: usize, from: usize, to: usize) {
        let n = number;
        let popped: Vec<T> = (0..n).map(|_| self.stacks[from - 1].pop()).collect();

        (0..n).for_each(|i| self.stacks[to - 1].push(popped[n - i - 1]));
    }
    fn top_crates(&mut self) -> Vec<T> {
        self.stacks.iter().map(|stack| stack.peek()).collect()
    }
}

#[derive(Clone, Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T: Copy> Stack<T> {
    fn push(&mut self, item: T) {
        self.items.push(item)
    }
    fn pop(&mut self) -> T {
        self.items.pop().unwrap()
    }
    fn peek(&self) -> T {
        self.items[self.items.len() - 1]
    }

    fn from_crates(crates: Vec<T>) -> Stack<T> {
        Stack { items: crates }
    }
}

#[test]
fn move_cargo_9000_test() {
    let mut cargo = Cargo {
        stacks: vec![
            Stack {
                items: vec!['Z', 'N'],
            },
            Stack {
                items: vec!['M', 'C', 'D'],
            },
            Stack { items: vec!['P'] },
        ],
    };

    cargo.move_cargo_9000(1, 2, 1);
    cargo.move_cargo_9000(3, 1, 3);
    cargo.move_cargo_9000(2, 2, 1);
    cargo.move_cargo_9000(1, 1, 2);

    let got = cargo.top_crates();
    assert_eq!(got.len(), 3);

    let message: String = got.iter().collect();
    assert_eq!(message, "CMZ")
}

#[test]
fn move_cargo_9001_test() {
    let mut cargo = Cargo {
        stacks: vec![
            Stack {
                items: vec!['Z', 'N'],
            },
            Stack {
                items: vec!['M', 'C', 'D'],
            },
            Stack { items: vec!['P'] },
        ],
    };

    cargo.move_cargo_9001(1, 2, 1);
    cargo.move_cargo_9001(3, 1, 3);
    cargo.move_cargo_9001(2, 2, 1);
    cargo.move_cargo_9001(1, 1, 2);

    let got = cargo.top_crates();
    assert_eq!(got.len(), 3);

    let message: String = got.iter().collect();
    assert_eq!(message, "MCD")
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn pop() {
        let mut stack = Stack { items: vec![] };

        stack.push('A');

        assert_eq!('A', stack.pop())
    }

    #[test]
    fn peek() {
        let mut stack = Stack { items: vec![] };

        stack.push('A');
        stack.push('B');
        stack.push('C');

        assert_eq!('C', stack.peek())
    }
}

fn parse_movements(input: &str) -> (usize, usize, usize) {
    let number = input[4..input.find("f").unwrap()]
        .trim()
        .parse::<usize>()
        .unwrap();

    let from = input[input.find("from ").unwrap() + 5..input.find(" to").unwrap()]
        .parse::<usize>()
        .unwrap();

    let to = input[input.find("to ").unwrap() + 3..]
        .parse::<usize>()
        .unwrap();

    (number, from, to)
}

#[test]
fn parse_movements_test() {
    let input = "move 3 from 8 to 2";

    let want = (3, 8, 2);

    assert_eq!(parse_movements(input), want);
}
