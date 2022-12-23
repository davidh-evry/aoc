use std::{
    collections::{BinaryHeap, HashMap},
    vec,
};

fn main() {
    let path = "res/day16.txt";
    let (valves, start) = parse_valves(path);
    println!("{}", search(&valves, 30, start, 1));
    println!("{}", search(&valves, 26, start, 2));
}

fn search(valves: &[Valve], steps: u32, start: usize, players: u32) -> u32 {
    let find_neighbours = if players == 2 {
        find_neighbours_2
    } else {
        find_neighbours_1
    };
    let mut costs = HashMap::new();
    let mut heap = BinaryHeap::new();
    let all_closed_flow_valves = (0..valves.len())
        .map(|i| valves[i].rate > 0)
        .collect::<Vec<_>>();
    costs.insert(([start, start], all_closed_flow_valves.clone()), steps);
    heap.push(State {
        flow: 0,
        minute: [0, if players == 2 { 0 } else { steps }],
        closed_flow_valves: all_closed_flow_valves.clone(),
        pos: [start, start],
    });
    let shortest_paths = floyd_warshall(valves);
    let mut max = 0;
    while let Some(state) = heap.pop() {
        if let Some(cost) = costs.get(&(state.pos, state.closed_flow_valves.clone())) {
            if state.minute[0] + state.minute[1] > *cost {
                continue;
            }
        }
        let neighbours = find_neighbours(valves, &state, steps, &shortest_paths);
        if neighbours.is_empty() {
            max = max.max(state.flow);
        }
        for next in neighbours {
            let prev = costs.get(&(next.pos, next.closed_flow_valves.clone()));
            let cost = next.minute[0] + next.minute[1];
            if prev.filter(|c| **c <= cost).is_none() {
                costs.insert((next.pos, next.closed_flow_valves.clone()), cost);
                heap.push(next);
            }
        }
    }
    return max;
}

fn floyd_warshall(valves: &[Valve]) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![u32::MAX / 2; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        for n in &valves[i].neighbours {
            matrix[i][*n] = 1;
        }
        matrix[i][i] = 0;
    }
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                let len = matrix[i][k] + matrix[k][j];
                if len < matrix[i][j] {
                    matrix[i][j] = len;
                }
            }
        }
    }
    matrix
}
fn find_neighbours_1(
    valves: &[Valve],
    current: &State,
    steps: u32,
    shortest_paths: &Vec<Vec<u32>>,
) -> Vec<State> {
    let mut states = vec![];
    for i in 0..current.closed_flow_valves.len() {
        if !current.closed_flow_valves[i] {
            continue;
        }
        if let Some(state) = get_state([i, 1], &current, steps, shortest_paths, valves) {
            states.push(state);
        }
    }
    states
}

fn find_neighbours_2(
    valves: &[Valve],
    current: &State,
    steps: u32,
    shortest_paths: &Vec<Vec<u32>>,
) -> Vec<State> {
    let mut states = vec![];
    for i in 0..current.closed_flow_valves.len() - 1 {
        if !current.closed_flow_valves[i] {
            continue;
        }
        for j in i + 1..current.closed_flow_valves.len() {
            if !current.closed_flow_valves[j] {
                continue;
            }
            if let Some(state) = get_state([i, j], &current, steps, shortest_paths, valves) {
                states.push(state);
            }
            if let Some(state) = get_state([j, i], &current, steps, shortest_paths, valves) {
                states.push(state);
            }
        }
    }
    states
}

fn get_state(
    next: [usize; 2],
    current: &State,
    steps: u32,
    shortest_paths: &Vec<Vec<u32>>,
    valves: &[Valve],
) -> Option<State> {
    let mut pos = current.pos;
    let mut minute = current.minute;
    let mut flow = current.flow;
    let mut closed_flow_valves = current.closed_flow_valves.clone();
    for i in [0, 1] {
        let next_minute = shortest_paths[current.pos[i]][next[i]] + current.minute[i] + 1;
        if next_minute > steps {
            continue;
        }
        let valve = next[i];
        pos[i] = valve;
        minute[i] = next_minute;
        flow += valves[valve].rate * (steps - minute[i]);
        closed_flow_valves[valve] = false;
    }
    if pos == current.pos {
        return None;
    }

    Some(State {
        pos,
        minute,
        flow,
        closed_flow_valves,
    })
}

fn parse_valves(path: &str) -> (Vec<Valve>, usize) {
    let file_content = std::fs::read_to_string(path).unwrap();
    let lines = file_content.lines();
    let mut unfinished_valves = vec![];
    let mut map = HashMap::new();
    for (i, line) in lines.enumerate() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let name = split[1];
        map.insert(name.to_owned(), i);
        let rate = split[4][5..split[4].len() - 1].parse::<u32>().unwrap();
        let neighbours = split[9..]
            .iter()
            .map(|s| s.replace(",", "").to_owned())
            .collect::<Vec<_>>();
        unfinished_valves.push((name.to_owned(), rate, neighbours));
    }
    let mut valves = vec![
        Valve {
            rate: 0,
            neighbours: vec![],
            name: "".to_owned(),
        };
        unfinished_valves.len()
    ];
    for (name, rate, neighbours) in unfinished_valves {
        valves[*map.get(&name).unwrap()] = Valve {
            rate,
            neighbours: neighbours.iter().map(|n| *map.get(n).unwrap()).collect(),
            name: name.clone(),
        }
    }
    let start = *map.get("AA").unwrap();
    (valves, start)
}

#[derive(Debug, Clone)]
struct Valve {
    rate: u32,
    neighbours: Vec<usize>,
    #[allow(unused)]
    name: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct State {
    pos: [usize; 2],
    minute: [u32; 2],
    flow: u32,
    closed_flow_valves: Vec<bool>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.minute[0] + other.minute[1]).cmp(&(self.minute[0] + self.minute[1]))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
