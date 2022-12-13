use itertools::Itertools;
use qu::ick_use::*;
use std::fmt;

const INPUT: &str = include_str!("../input/8");

pub fn first() -> Result<usize> {
    let input = TreeGrid::parse(INPUT)?;
    Ok((0..input.width)
        .cartesian_product(0..input.height())
        .filter(|(x, y)| input.tree_visible(*x, *y))
        .count())
}

pub fn second() -> Result<usize> {
    let input = TreeGrid::parse(INPUT)?;
    Ok((0..input.width)
        .cartesian_product(0..input.height())
        .map(|(x, y)| input.trees_seen(x, y))
        .max()
        .unwrap_or(0))
}

struct TreeGrid {
    trees: Vec<u8>,
    width: usize,
}

impl TreeGrid {
    fn parse(i: &str) -> Result<Self> {
        let mut lines = i.lines();
        let mut trees = vec![];

        let first = lines.next().context("no input data")?;
        for ch in first.bytes() {
            ensure!(ch >= b'0' && ch <= b'9', "tree heights should be digits");
            trees.push(ch - b'0');
        }
        let width = trees.len();
        for line in lines {
            let line = line.as_bytes();
            ensure!(
                line.len() == width,
                "found line that didn't match length of first line"
            );
            for ch in line.iter().copied() {
                ensure!(ch >= b'0' && ch <= b'9', "tree heights should be digits");
                trees.push(ch - b'0');
            }
        }
        assert_eq!(trees.len() % width, 0);
        Ok(TreeGrid { trees, width })
    }

    fn tree_height_at(&self, x: usize, y: usize) -> u8 {
        self.trees[y * self.width + x]
    }

    fn height(&self) -> usize {
        self.trees.len() / self.width
    }

    /// Is the tree at (x, y) visible from the edge.
    fn tree_visible(&self, x: usize, y: usize) -> bool {
        // Are x and y in bounds.
        assert!(x < self.width && y * self.width < self.trees.len());

        let height = self.tree_height_at(x, y);

        // x direction
        (0..x).all(|x_ix| self.tree_height_at(x_ix, y) < height)
            || (x + 1..self.width).all(|x_ix| self.tree_height_at(x_ix, y) < height)
            || (0..y).all(|y_ix| self.tree_height_at(x, y_ix) < height)
            || (y + 1..self.height()).all(|y_ix| self.tree_height_at(x, y_ix) < height)
    }

    /// How many trees can the tree at x, y see?
    fn trees_seen(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width && y * self.width < self.trees.len());

        let height = self.tree_height_at(x, y);
        let x_less_taller = (0..x)
            .rev()
            .find(|x_ix| self.tree_height_at(*x_ix, y) >= height)
            .unwrap_or(0);
        let x_less_count = x - x_less_taller;
        let x_greater_taller = (x + 1..self.width)
            .find(|x_ix| self.tree_height_at(*x_ix, y) >= height)
            .unwrap_or(self.width - 1);
        let x_greater_count = x_greater_taller - x;
        let y_less_taller = (0..y)
            .rev()
            .find(|y_ix| self.tree_height_at(x, *y_ix) >= height)
            .unwrap_or(0);
        let y_less_count = y - y_less_taller;
        let y_greater_taller = (y + 1..self.height())
            .find(|y_ix| self.tree_height_at(x, *y_ix) >= height)
            .unwrap_or(self.height() - 1);
        let y_greater_count = y_greater_taller - y;
        x_less_count * x_greater_count * y_less_count * y_greater_count
    }
}

impl fmt::Debug for TreeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, ch) in self.trees.iter().copied().enumerate() {
            write!(f, "{}", ch)?;
            if (idx + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[test]
pub fn test_parse() {
    let input = TreeGrid::parse(INPUT).unwrap();
    assert_eq!(input.tree_height_at(0, 0), 0);
    assert_eq!(input.tree_height_at(0, 2), 0);
}
