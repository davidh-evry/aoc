fn main() {
    let file_content = std::fs::read_to_string("res/day1.txt").unwrap();
    let mut elves = vec![];
    let mut elf = vec![];
    for line in file_content.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = vec![]
        } else {
            elf.push(line.parse::<u32>().unwrap())
        }
    }
    println!("Max: {:?}", find_max(&elves));
    println!("Max: {:?}", find_max(&elves).iter().sum::<u32>());
}

fn find_max(elves: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut maxes = vec![0, 0, 0];
    for elf in elves {
        let cals: u32 = elf.iter().sum();
        for i in 0..maxes.len() {
            if cals > maxes[i] {
                maxes.insert(i, cals);
                maxes.pop();
                break;
            }
        }
    }
    return maxes;
}
