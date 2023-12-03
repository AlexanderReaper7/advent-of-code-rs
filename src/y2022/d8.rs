//! --- Day 8: Treetop Tree House ---
#![allow(unused_variables)] // TODO: remove this once implemented
#![allow(unreachable_code)] // TODO: remove this once implemented
use std::{str::FromStr, fmt::Display};

struct TreeMap{
    /// y, x
    trees: Vec<Vec<u8>>,
}
impl TreeMap {
    /// Checks if this tree is visible or hidden
    /// 
    /// A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. 
    /// Only considers trees in the same row or column; that is, only look up, down, left, or right from any given tree.
    /// All of the trees around the edge of the grid are visible
    /// # Arguments
    /// * `tree` - the tree to check in the form (y, x)
    /// # Returns
    /// true if visible, false if hidden
    fn is_visible(&self, tree: (usize, usize)) -> bool {
        let (y, x) = tree;
        // if the tree is at the edge, it is visible
        if y == 0 || y == self.trees.len() - 1 || x == 0 || x == self.trees[y].len() - 1 {
            return true;
        }
        let tree_height = self.trees[y][x];
        // check up
        for i in (0..y).rev() {
            // if the tree is taller or equal than the current tree, it is hidden in this direction
            if self.trees[i][x] >= tree_height {
                break;
            }
            // if the tree is shorter than the current tree and we are at the edge, it is visible
            if i == 0 {
                return true;
            }
        }
        // check down
        for i in y+1..self.trees.len() {
            if self.trees[i][x] >= tree_height {
                break;
            }
            if i == self.trees.len() - 1 {
                return true;
            }
        }
        // check left
        for i in (0..x).rev() {
            if self.trees[i][x] >= tree_height {
                break;
            }
            if i == 0 {
                return true;
            }
        }
        // check right
        for i in x+1..self.trees[y].len() {
            if self.trees[i][x] >= tree_height {
                break;
            }
            if i == self.trees[y].len() - 1 {
                return true;
            }
        }
        // if we get here, the tree is hidden in all directions
        false
    } 
}
impl FromStr for TreeMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = s.lines().next().unwrap().len();
        let mut trees = Vec::with_capacity(size);
        for line in s.lines() {
            let mut row = Vec::with_capacity(size);
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
            trees.push(row);
        }
        Ok(TreeMap{trees})
    }
}
impl Display for TreeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.trees {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: String) -> String {
    let map = TreeMap::from_str(&input).unwrap();
    println!("\n\n{}\n", map);
    // skip and add the edges
    let edge_visible_trees = (map.trees[0].len() * 2) + (map.trees.len() * 2) - 4;
    let mut interior_visible_trees = 0;
    for y in 1..map.trees.len()-1 {
        for x in 1..map.trees[0].len()-1 {
            if map.is_visible((y, x)) {
                interior_visible_trees += 1;
            }
        }
    }
    println!("With {} trees visible on the edge and another {} visible in the interior, a total of {} trees are visible in this arrangement.", edge_visible_trees, interior_visible_trees, edge_visible_trees + interior_visible_trees);
    todo!("incomplete/incorrect solution");
    (edge_visible_trees + interior_visible_trees).to_string()
}

pub fn part2(input: String) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = 
"30373
25512
65332
33549
35390";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT.to_string()), "21".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT.to_string()), "".to_string());
        unimplemented!();
    }
}
