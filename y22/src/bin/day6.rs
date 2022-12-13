use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("res/day6.txt").unwrap();
    let chars = input.chars().collect::<Vec<_>>();
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
