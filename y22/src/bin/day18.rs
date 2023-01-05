use std::{
    collections::{HashSet, VecDeque},
    vec,
};

const NEIGHBOURS: [[i32; 3]; 6] = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
];

fn main() {
    let board = parse_board("res/day18.txt");
    calculate_surface_area(&board);
    calculate_external_surface_area(&board);
}

fn calculate_surface_area(board: &Vec<Vec<Vec<usize>>>) {
    let size = get_size(&board);
    let mut sum = 0;
    for z in 0..size[2] {
        for y in 0..size[1] {
            for x in 0..size[0] {
                if board[z as usize][y as usize][x as usize] == 0 {
                    continue;
                }
                for n in NEIGHBOURS.map(|n| [n[0] + x, n[1] + y, n[2] + z]) {
                    let [x, y, z] = n.map(|u| u as usize);
                    sum += 1 - board[z][y][x];
                }
            }
        }
    }
    println!("{sum}")
}

fn calculate_external_surface_area(board: &Vec<Vec<Vec<usize>>>) {
    let size = get_size(board);
    let mut sum = 0;
    let mut visited = HashSet::new();
    let mut nodes = VecDeque::from([[0, 0, 0]]);
    while !nodes.is_empty() {
        let node = nodes.pop_front().unwrap();
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for n in NEIGHBOURS.map(|n| [0, 1, 2].map(|i| node[i] + n[i])) {
            if (0..3).any(|i| n[i] < 0 || n[i] >= size[i]) {
                continue;
            }
            let [x, y, z] = n.map(|u| u as usize);
            let val = board[z][y][x];
            if val == 1 {
                sum += 1
            } else if !visited.contains(&n) {
                nodes.push_back(n);
            }
        }
    }
    println!("{sum}");
}

fn get_size(b: &Vec<Vec<Vec<usize>>>) -> [i32; 3] {
    [b[0][0].len() as i32, b[0].len() as i32, b.len() as i32]
}

fn parse_board(path: &str) -> Vec<Vec<Vec<usize>>> {
    let coords: Vec<[usize; 3]> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<usize>().unwrap() + 1)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let max = coords
        .iter()
        .fold([0, 0, 0], |acc, c| [0, 1, 2].map(|i| c[i].max(acc[i])));
    let mut board = vec![vec![vec![0usize; max[0] + 2]; max[1] + 2]; max[2] + 2];
    for [x, y, z] in coords {
        board[z][y][x] = 1;
    }
    board
}
