use std::{collections::HashSet, fs};

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./data").expect("failed to read file");

    let mut grid = Grid {
        head: Point(0, 0),
        tail: vec![Point(0, 0); 9],
    };

    println!(
        "unique tail positions {}",
        unique_tail_positions(&mut grid, &contents)
    );
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Point(isize, isize);

fn unique_tail_positions(grid: &mut Grid, input: &str) -> usize {
    let mut map = HashSet::new();

    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .for_each(|line| {
            let times = line[2..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            for _ in 0..times {
                let direction = match line[0] {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    'D' => Direction::Down,
                    'U' => Direction::Up,
                    _ => Direction::Left,
                };
                grid.move_dir(direction);
                map.insert(grid.tail[grid.tail.len() - 1]);
            }
        });

    map.len()
}

fn are_touching(head: &Point, tail: &Point) -> bool {
    !((head.0 as i64) - (tail.0 as i64) > 1
        || (head.0 as i64) - (tail.0 as i64) < -1
        || (head.1 as i64) - (tail.1 as i64) > 1
        || (head.1 as i64) - (tail.1 as i64) < -1)
}

#[derive(Debug)]
struct Grid {
    head: Point,
    tail: Vec<Point>,
}

impl Grid {
    fn move_dir(&mut self, direction: Direction) {
        println!("moving {:?}", self);
        match direction {
            Direction::Left => self.head.1 -= 1,
            Direction::Right => self.head.1 += 1,
            Direction::Up => self.head.0 += 1,
            Direction::Down => self.head.0 -= 1,
        }

        fn move_tails(head: &mut Point, tail: &mut Point) {
            if !are_touching(&head, &tail) {
                tail.0 += (&head.0 - &tail.0).signum() * 1;
                tail.1 += (&head.1 - &tail.1).signum() * 1;
            }
        }

        fn move_tails_vec(idx: usize, tails: &mut Vec<Point>) {
            let headIdx = idx;
            let tailIdx = idx + 1;
            if !are_touching(&tails[headIdx], &tails[tailIdx]) {
                tails[tailIdx].0 += (&tails[headIdx].0 - &tails[tailIdx].0).signum() * 1;
                tails[tailIdx].1 += (&tails[headIdx].1 - &tails[tailIdx].1).signum() * 1;
            }
        }

        move_tails(&mut self.head, &mut self.tail[0]);

        for i in 0..self.tail.len() - 1 {
            move_tails_vec(i, &mut self.tail);
        }

        //
        // move_tails(self.head, &mut self.tail);
    }
}

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[test]
fn are_touching_test() {
    // Vertically and Horizontally
    assert_eq!(are_touching(&Point(0, 0), &Point(0, 0)), true);
    assert_eq!(are_touching(&Point(0, 1), &Point(0, 0)), true);
    assert_eq!(are_touching(&Point(0, 2), &Point(0, 0)), false);
    assert_eq!(are_touching(&Point(0, 2), &Point(0, 0)), false);
    assert_eq!(are_touching(&Point(0, 1), &Point(0, 2)), true);
    assert_eq!(are_touching(&Point(1, 0), &Point(0, 0)), true);
    assert_eq!(are_touching(&Point(2, 0), &Point(1, 0)), true);
    assert_eq!(are_touching(&Point(1, 0), &Point(2, 0)), true);

    // Diagonally
    assert_eq!(are_touching(&Point(0, 0), &Point(1, 1)), true);
    assert_eq!(are_touching(&Point(1, 1), &Point(2, 2)), true);
    assert_eq!(are_touching(&Point(0, 0), &Point(2, 2)), false);
}

#[test]
fn visited_once_test() {
    let mut grid = Grid {
        head: Point(0, 0),
        tail: vec![Point(0, 0)],
    };

    (0..4).for_each(|_| grid.move_dir(Direction::Right));
    (0..4).for_each(|_| grid.move_dir(Direction::Up));
}

#[test]
fn test_case() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    let mut grid = Grid {
        head: Point(0, 0),
        tail: vec![Point(0, 0)],
    };
    assert_eq!(unique_tail_positions(&mut grid, input), 13);
}

#[test]
fn multi_tailtest_case() {
    let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    let mut grid = Grid {
        head: Point(0, 0),
        tail: vec![Point(0, 0); 9],
    };
    assert_eq!(unique_tail_positions(&mut grid, input), 36);
}

#[test]
fn tail_follows() {
    let mut input = Grid {
        head: Point(0, 0),
        tail: vec![Point(0, 0)],
    };

    (0..4).for_each(|_| input.move_dir(Direction::Right));
    (0..4).for_each(|_| input.move_dir(Direction::Up));
    assert_eq!(input.head, Point(4, 4));
    assert_eq!(input.tail[0], Point(3, 4));

    input.move_dir(Direction::Left);
    input.move_dir(Direction::Left);
    input.move_dir(Direction::Left);

    assert_eq!(input.head, Point(4, 1));
    assert_eq!(input.tail[0], Point(4, 2));
}
