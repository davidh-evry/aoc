use std::collections::BinaryHeap;

use y22::read_lines;

fn main() {
    let lines = read_lines("res/day12.txt");
    let chars = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = vec![vec![0; chars[0].len()]; chars.len()];
    let mut start = [0, 0];
    let mut end = [0, 0];
    for y in 0..chars.len() {
        for x in 0..chars[0].len() {
            let c = chars[y][x];
            grid[y][x] = c as i32 - 96;
            if c == 'S' {
                start = [x, y];
                grid[y][x] = 1;
            } else if c == 'E' {
                end = [x, y];
                grid[y][x] = 26;
            }
        }
    }
    println!("{}", dijkstra(&grid, &start, &end, |_, _| 1));
    println!("{}", dijkstra(&grid, &start, &end, p2_cost_increase));
}

fn p2_cost_increase(cost: usize, value: i32) -> usize {
    if cost == 0 && value == 1 {
        0
    } else {
        1
    }
}

fn dijkstra<F>(grid: &[Vec<i32>], start: &[usize; 2], end: &[usize; 2], cost_increase: F) -> usize
where
    F: Fn(usize, i32) -> usize,
{
    let mut costs = (0..grid.len())
        .map(|_| (0..grid[0].len()).map(|_| usize::MAX).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    costs[start[1]][start[0]] = 0;
    heap.push(State {
        cost: 0,
        position: *start,
    });
    while let Some(State { cost, position }) = heap.pop() {
        if position == *end {
            return cost;
        }
        if cost > costs[position[1]][position[0]] {
            continue;
        }
        for neighbour in find_neighbours(&grid, &position) {
            let state = State {
                cost: cost + cost_increase(cost, grid[neighbour[1]][neighbour[0]]),
                position: neighbour,
            };

            if state.cost < costs[state.position[1]][state.position[0]] {
                costs[state.position[1]][state.position[0]] = state.cost;
                heap.push(state);
            }
        }
    }
    panic!("Could not find the end!");
}

fn find_neighbours(grid: &[Vec<i32>], position: &[usize; 2]) -> Vec<[usize; 2]> {
    const NEIGHBOUR_POS: [[isize; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];
    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    NEIGHBOUR_POS
        .iter()
        .map(|n| [n[0] + position[0] as isize, n[1] + position[1] as isize])
        .filter(|n| n[0] >= 0 && n[0] < width && n[1] >= 0 && n[1] < height)
        .map(|n| [n[0] as usize, n[1] as usize])
        .filter(|n| grid[n[1]][n[0]] - grid[position[1]][position[0]] <= 1)
        .collect()
}

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    position: [usize; 2],
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
