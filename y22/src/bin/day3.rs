use std::collections::HashSet;

use y22::read_lines;

fn main() {
    let lines = read_lines("res/day3.txt");
    let rucksacks = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let sum = rucksacks
        .iter()
        .map(split)
        .map(|(l, r)| *l.intersection(&r).next().unwrap())
        .map(get_priority)
        .sum::<u32>();
    println!("{sum}");
    print_group_priority(rucksacks);
}

fn print_group_priority(rucksacks: Vec<Vec<char>>) {
    let mut sum = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let mut intersect: HashSet<&char> = HashSet::from_iter(&rucksacks[i]);
        for j in 1..=2 {
            intersect = intersect
                .intersection(&HashSet::from_iter(&rucksacks[i + j]))
                .map(|c| *c)
                .collect::<HashSet<_>>();
        }
        sum += get_priority(**intersect.iter().next().unwrap());
    }
    println!("{sum}");
}

fn split(chars: &Vec<char>) -> (HashSet<char>, HashSet<char>) {
    let (left, right) = chars.split_at(chars.len() / 2);
    (
        HashSet::from_iter(left.to_owned()),
        HashSet::from_iter(right.to_owned()),
    )
}

fn get_priority(c: char) -> u32 {
    let s = if c > 'Z' { 'a' as u32 } else { 'A' as u32 - 26 };
    c as u32 - s + 1
}
