use std::{cell::RefCell, collections::VecDeque};

fn main() {
    let file_content = std::fs::read_to_string("res/day11.txt").unwrap();
    let lines = file_content.lines().collect::<Vec<_>>();
    let monkeys = parse_monkeys(lines);
    println!("{}", run_sim(20, &monkeys, 3));
    println!("{}", run_sim(10000, &monkeys, 1));
}

fn run_sim(count: i32, original_monkeys: &Vec<Monkey>, relief: usize) -> usize {
    let monkeys = original_monkeys
        .iter()
        .cloned()
        .map(RefCell::new)
        .collect::<Vec<_>>();
    let mut inspect_counts = vec![0; monkeys.len()];
    let div_prod = original_monkeys
        .iter()
        .map(|m| m.divisor)
        .product::<usize>();
    for _ in 0..count {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys[i].borrow_mut();
            while !monkey.items.is_empty() {
                let mut item = monkey.items.pop_front().unwrap();
                monkey.operation.calculate(&mut item);
                inspect_counts[i] += 1;
                item %= div_prod;
                item /= relief;
                let index = monkey.get_throw_index(&item);
                monkeys[index].borrow_mut().items.push_back(item);
            }
        }
    }
    inspect_counts.sort_by(|a, b| b.cmp(a));
    inspect_counts.iter().take(2).product()
}

fn parse_monkeys(all_lines: Vec<&str>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    for lines in all_lines.chunks(7) {
        let items = lines[1][18..]
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = lines[2].into();
        let divisor = last_num(&lines[3]);
        let throw_true = last_num(&lines[4]);
        let throw_false = last_num(&lines[5]);
        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            throw_true,
            throw_false,
        });
    }
    monkeys
}

fn last_num(line: &str) -> usize {
    line.split_whitespace().last().unwrap().parse().unwrap()
}

#[derive(Debug, Clone)]
enum Operation {
    Plus(usize),
    Mult(usize),
    Square,
}
impl Operation {
    fn calculate(&self, item: &mut usize) {
        *item = match self {
            Operation::Plus(a) => *item + a,
            Operation::Mult(a) => *item * a,
            Operation::Square => *item * *item,
        }
    }
}

impl From<&str> for Operation {
    fn from(line: &str) -> Self {
        let operands = line[19..].split(" ").collect::<Vec<_>>();
        match operands[1] {
            "+" => Operation::Plus(operands[2].parse().unwrap()),
            "*" => {
                if operands[2] == "old" {
                    Operation::Square
                } else {
                    Operation::Mult(operands[2].parse().unwrap())
                }
            }
            _ => panic!("Unknown operation {:?}", operands),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisor: usize,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn get_throw_index(&self, item: &usize) -> usize {
        if item % self.divisor == 0 {
            self.throw_true
        } else {
            self.throw_false
        }
    }
}
