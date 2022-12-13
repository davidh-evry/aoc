use std::collections::HashSet;

fn main() {
    let commands = std::fs::read_to_string("res/day9.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_once(" ")
                .map(|(l, r)| (l.to_owned(), r.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();
    run_sim(&commands, 2);
    run_sim(&commands, 10);
}

fn run_sim(commands: &[(String, usize)], knots: usize) {
    let mut knots = (0..knots).map(|_| [0i32; 2]).collect::<Vec<_>>();
    let mut visited = HashSet::new();
    for (dir, count) in commands {
        let (change_dir, sign) = match dir.as_str() {
            "R" => (0, 1),
            "U" => (1, 1),
            "L" => (0, -1),
            "D" => (1, -1),
            _ => panic!("Unknown direction: {dir}"),
        };
        for _ in 0..*count {
            let mut head = knots.first().unwrap().clone();
            head[change_dir] += sign;
            knots[0] = head;
            for i in 1..knots.len() {
                let head = knots[i - 1].clone();
                let tail = &mut knots[i];
                let diff = [0, 1].map(|i| head[i] - tail[i]);
                for (j, o) in [(0, 1), (1, 0)] {
                    if diff[j].abs() < 2 {
                        continue;
                    }
                    tail[j] += diff[j] / 2;
                    tail[o] += diff[o].signum();
                    break;
                }
            }
            visited.insert(knots.last().unwrap().clone());
        }
    }
    println!("{}", visited.len());
}
