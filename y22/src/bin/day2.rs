use std::collections::HashMap;

use y22::read_lines;

fn main() {
    let lines = read_lines("res/day2.txt");
    let games = lines
        .iter()
        .map(|line| (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap()))
        .collect::<Vec<_>>();
    print_score(&games, part_1_map());
    print_score(&games, part_2_map());
}

fn print_score(games: &[(char, char)], map: HashMap<(char, char), u32>) {
    let sum = games.iter().map(|t| map.get(t).unwrap()).sum::<u32>();
    println!("{sum}");
}

fn part_2_map() -> HashMap<(char, char), u32> {
    let mut map = HashMap::new();
    map.insert(('A', 'X'), 3u32);
    map.insert(('A', 'Y'), 4);
    map.insert(('A', 'Z'), 8);
    map.insert(('B', 'X'), 1);
    map.insert(('B', 'Y'), 5);
    map.insert(('B', 'Z'), 9);
    map.insert(('C', 'X'), 2);
    map.insert(('C', 'Y'), 6);
    map.insert(('C', 'Z'), 7);
    map
}

fn part_1_map() -> HashMap<(char, char), u32> {
    let mut map = HashMap::new();
    map.insert(('A', 'X'), 4u32);
    map.insert(('A', 'Y'), 8);
    map.insert(('A', 'Z'), 3);
    map.insert(('B', 'X'), 1);
    map.insert(('B', 'Y'), 5);
    map.insert(('B', 'Z'), 9);
    map.insert(('C', 'X'), 7);
    map.insert(('C', 'Y'), 2);
    map.insert(('C', 'Z'), 6);
    map
}
