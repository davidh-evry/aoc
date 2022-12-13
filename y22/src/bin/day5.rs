fn main() {
    let (stacks, commands) = parse("res/day5.txt");
    p1(commands.clone(), stacks.clone());
    p2(commands.clone(), stacks.clone());
}

fn p2(commands: Vec<(usize, usize, usize)>, mut stacks: Vec<Vec<char>>) {
    for (from, to, count) in commands {
        let len = stacks[from].len();
        let values = stacks[from].drain(len - count..).collect::<Vec<_>>();
        stacks[to].extend(values);
    }
    let word = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("{word}");
}

fn p1(commands: Vec<(usize, usize, usize)>, mut stacks: Vec<Vec<char>>) {
    for (from, to, count) in commands {
        for _ in 0..count {
            let value = stacks[from].pop().unwrap();
            stacks[to].push(value);
        }
    }
    let word = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("{word}");
}

fn parse(path: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let lines = file_content.lines().collect::<Vec<_>>();
    let mut stacks = vec![vec![]; 9];
    for line in &lines[0..8] {
        for (i, chars) in line.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            let c = chars[1];
            if c != ' ' {
                stacks[i].insert(0, c);
            }
        }
    }
    let mut commands = vec![];
    for line in &lines[10..] {
        let mut cmd = line.replace("move", "");
        cmd = cmd.replace("from", "");
        cmd = cmd.replace("to", "");
        let split = cmd.split_whitespace().collect::<Vec<_>>();
        commands.push((
            split[1].parse::<usize>().unwrap() - 1,
            split[2].parse::<usize>().unwrap() - 1,
            split[0].parse().unwrap(),
        ));
    }
    (stacks, commands)
}
