use std::collections::HashSet;

use y22::read_lines;

fn main() {
    let lines = read_lines("res/day8.txt");
    let mut grid = vec![];
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    }
    find_viewable_trees(&grid);
    find_best_scenic_score(&grid);
}

fn find_viewable_trees(grid: &Vec<Vec<u32>>) {
    let n = grid.len();
    let mut visible = HashSet::new();
    for i in 0..n {
        let mut highest = 0;
        for j in 0..n {
            let tree = grid[i][j];
            if tree > highest || i == 0 || i == n - 1 {
                visible.insert((i, j));
                highest = tree;
            }
        }
        highest = 0;
        for j in (0..n).rev() {
            let tree = grid[i][j];
            if tree > highest {
                visible.insert((i, j));
                highest = tree;
            }
        }
    }
    for j in 0..n {
        let mut highest = 0;
        for i in 0..n {
            let tree = grid[i][j];
            if tree > highest || j == 0 || j == n - 1 {
                visible.insert((i, j));
                highest = tree;
            }
        }
        highest = 0;
        for i in (0..n).rev() {
            let tree = grid[i][j];
            if tree > highest {
                visible.insert((i, j));
                highest = tree;
            }
        }
    }
    println!("{}", visible.len());
}

fn find_best_scenic_score(grid: &Vec<Vec<u32>>) {
    let n = grid.len();
    let mut max_score = 0;
    for i in 0..n {
        for j in 0..n {
            let tree = grid[i][j];
            let mut scores = [0; 4];
            for k in (0..j).rev() {
                scores[0] += 1;
                if grid[i][k] >= tree {
                    break;
                }
            }
            for k in j + 1..n {
                scores[1] += 1;
                if grid[i][k] >= tree {
                    break;
                }
            }
            for k in (0..i).rev() {
                scores[2] += 1;
                if grid[k][j] >= tree {
                    break;
                }
            }
            for k in i + 1..n {
                scores[3] += 1;
                if grid[k][j] >= tree {
                    break;
                }
            }
            let product = scores.into_iter().product::<i32>();
            if product > max_score {
                max_score = product;
            }
        }
    }
    println!("{}", max_score);
}
