use std::{num::ParseIntError, str::FromStr};

fn main() {
    println!("Hello, world!");
}

type Grid = Vec<Vec<u32>>;

struct TreeGrid {
    trees: Grid,
}

impl TreeGrid {
    fn count_visible_trees(&self) -> usize {
        // 'outer: for x in 0..grid.len() - 1 {
        //     'inner: for y in 0..grid[0].len() - 1 {
        //         // is_visible(x, y)
        //
        //         // if x == y {
        //         //     break 'outer;
        //         // }
        //     }
        // }

        self.trees.len() * 2 + self.trees[0].len() * 2 - 4
    }

    fn tree_is_visible(&self, row: usize, col: usize) -> bool {
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
                .split("\n")
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
fn count_outside_trees() {
    let input = TreeGrid::from_str(
        "30373
25512
65332
33549
35390",
    )
    .unwrap();

    assert_eq!(input.count_visible_trees(), 16);
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
