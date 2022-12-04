use std::ops::Range;

use y22::read_lines;

fn main() {
    let pairs = parse("res/day4.txt");
    count_fully_contains(&pairs);
    count_any_overlap(&pairs);
}

fn count_any_overlap(pairs: &[(Range<u32>, Range<u32>)]) {
    let count = pairs
        .iter()
        .filter(|(a, b)| a.start.max(b.start) <= a.end.min(b.end))
        .count();
    println!("{count}");
}

fn count_fully_contains(pairs: &[(Range<u32>, Range<u32>)]) {
    let count = pairs
        .iter()
        .filter(|(a, b)| fully_contains(a, b) || fully_contains(b, a))
        .count();
    println!("{count}");
}

fn fully_contains(a: &Range<u32>, b: &Range<u32>) -> bool {
    a.start >= b.start && a.end <= b.end
}

fn parse(path: &str) -> Vec<(Range<u32>, Range<u32>)> {
    read_lines(path)
        .into_iter()
        .map(|line| line.split(",").map(String::from).collect::<Vec<_>>())
        .map(|s| (get_range(&s[0]), get_range(&s[1])))
        .collect()
}

fn get_range(str: &str) -> Range<u32> {
    let split = str.split("-").collect::<Vec<_>>();
    split[0].parse::<u32>().unwrap()..split[1].parse::<u32>().unwrap()
}
