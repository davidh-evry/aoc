use std::collections::HashSet;

use y22::read_lines;

fn main() {
    let input = read_lines("res/day6.txt");
    let chars = input[0].chars().collect::<Vec<_>>();
    get_start(&chars, 4);
    get_start(&chars, 14);
}

fn get_start(chars: &Vec<char>, distinct: usize) {
    for i in distinct..chars.len() {
        let set: HashSet<&char> = HashSet::from_iter(&chars[i - distinct..i]);
        if set.len() == distinct {
            println!("{i}");
            break;
        }
    }
}
