use std::{fs, num::ParseIntError, str::FromStr};

fn main() {
    let contents = fs::read_to_string("./data").expect("failed to read file");
    let tree_grid = TreeGrid::from_str(&contents).unwrap();
    println!("visible trees {}", tree_grid.count_visible_trees());

    println!("max scenic score {}", tree_grid.max_scenic_score());
}

type Grid = Vec<Vec<u32>>;

struct TreeGrid {
    trees: Grid,
}

impl TreeGrid {
    fn max_scenic_score(&self) -> usize {
        let mut max_score = 0;
        for row in 1..self.trees.len() {
            for col in 1..self.trees[0].len() {
                let score = self.tree_scenic_score(row, col);

                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }
    fn tree_scenic_score(&self, row: usize, col: usize) -> usize {
        fn top_score(grid: &Grid, row: usize, col: usize) -> usize {
            let mut counter = 0;
            let cur_height = grid[row][col];
            for r in (0..row).rev() {
                counter += 1;

                if cur_height <= grid[r][col] {
                    break;
                }
            }

            counter
        }

        fn bottom_score(grid: &Grid, row: usize, col: usize) -> usize {
            let mut counter = 0;
            let cur_height = grid[row][col];
            for r in row + 1..grid.len() {
                counter += 1;

                // println!("comparing height {} to {}", cur_height, grid[r][col]);
                if cur_height <= grid[r][col] {
                    break;
                }
            }

            counter
        }

        fn left_score(grid: &Grid, row: usize, col: usize) -> usize {
            let mut counter = 0;
            let cur_height = grid[row][col];
            for c in (0..col).rev() {
                counter += 1;

                // println!("comparing left height {} to {}", cur_height, grid[r][col]);
                if cur_height <= grid[row][c] {
                    break;
                }
            }

            counter
        }

        fn right_score(grid: &Grid, row: usize, col: usize) -> usize {
            let mut counter = 0;
            let cur_height = grid[row][col];
            for c in col + 1..grid.len() {
                counter += 1;

                if cur_height <= grid[row][c] {
                    break;
                }
            }

            counter
        }

        top_score(&self.trees, row, col)
            * bottom_score(&self.trees, row, col)
            * left_score(&self.trees, row, col)
            * right_score(&self.trees, row, col)
    }
    fn count_visible_trees(&self) -> usize {
        let mut visible_counter = 0;
        for row in 0..self.trees.len() {
            for col in 0..self.trees[0].len() {
                if self.tree_is_visible(row, col) {
                    visible_counter += 1;
                }
            }
        }
        visible_counter
    }

    fn tree_is_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 {
            return true;
        }
        fn visible_from_left(grid: &Grid, row: usize, col: usize) -> bool {
            for i in (0..col).rev() {
                if grid[row][col] <= grid[row][i] {
                    return false;
                }
            }
            true
        }

        fn visible_from_right(grid: &Grid, row: usize, col: usize) -> bool {
            for i in col + 1..grid[0].len() {
                if grid[row][col] <= grid[row][i] {
                    return false;
                }
            }
            true
        }

        fn visible_from_top(grid: &Grid, row: usize, col: usize) -> bool {
            for i in (0..row).rev() {
                if grid[row][col] <= grid[i][col] {
                    return false;
                }
            }
            true
        }

        fn visible_from_bottom(grid: &Grid, row: usize, col: usize) -> bool {
            for i in (row + 1..grid.len()).rev() {
                if grid[row][col] <= grid[i][col] {
                    return false;
                }
            }
            true
        }

        visible_from_left(&self.trees, row, col)
            || visible_from_top(&self.trees, row, col)
            || visible_from_right(&self.trees, row, col)
            || visible_from_bottom(&self.trees, row, col)
    }
}

impl FromStr for TreeGrid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TreeGrid {
            trees: s
                .split_terminator("\n")
                .map(|row| {
                    row.chars().fold(vec![], |mut a, c| {
                        a.push(c.to_digit(10).unwrap());
                        a
                    })
                })
                .collect(),
        })
    }
}

#[test]
fn scenic_score_test() {
    let input = TreeGrid::from_str(
        "30373
25512
65332
33549
35390
",
    )
    .unwrap();

    assert_eq!(input.tree_scenic_score(1, 2), 4);
    assert_eq!(input.tree_scenic_score(3, 2), 8);

    assert_eq!(input.max_scenic_score(), 8);
}

#[test]
fn count_outside_trees() {
    let input = TreeGrid::from_str(
        "30373
25512
65332
33549
35390
",
    )
    .unwrap();

    assert_eq!(input.count_visible_trees(), 21);
}

#[test]
fn tree_is_visible_test() {
    let input = TreeGrid::from_str(
        "30373
25512
65332
33549
35390",
    )
    .unwrap();

    assert_eq!(input.tree_is_visible(1, 1), true);
    assert_eq!(input.tree_is_visible(2, 2), false);
    assert_eq!(input.tree_is_visible(1, 2), true);
    assert_eq!(input.tree_is_visible(2, 1), true);

    assert_eq!(
        TreeGrid::from_str(
            "999
989
919"
        )
        .unwrap()
        .tree_is_visible(2, 1),
        true
    );
}

#[test]
fn grid_from_str_test() {
    let input = "30373
25512
65332
33549
35390";

    let want = vec![
        vec![3, 0, 3, 7, 3],
        vec![2, 5, 5, 1, 2],
        vec![6, 5, 3, 3, 2],
        vec![3, 3, 5, 4, 9],
        vec![3, 5, 3, 9, 0],
    ];

    assert_eq!(TreeGrid::from_str(input).unwrap().trees, want)
}
