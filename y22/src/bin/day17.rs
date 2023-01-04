fn main() {
    let (jet, rocks) = parse_input("res/day17.txt");
    println!("{}", run_sim(&rocks, &jet, 2022));
    println!("{}", run_sim(&rocks, &jet, 1000000000000));
}

fn run_sim(rocks: &Vec<Vec<Vec<char>>>, jet: &Vec<char>, max_steps: usize) -> usize {
    let mut board = vec![['.'; 7]; 10];
    let mut jet_index = 0;
    let mut rock_index = 0;
    let mut max_height = 0;
    let mut step = 0;
    let mut is_cycle_found = false;
    let mut cycle_height = 0;
    let look_back = 10;
    let mut seen_indices = vec![(usize::MAX, 0, vec![['.'; 7]; look_back]); jet.len()];
    while step < max_steps {
        let rock = &rocks[rock_index];
        let mut x = 7 - rock[0].len() - 2;
        let mut y = max_height + 3;
        for _ in board.len()..y + rock.len() {
            board.push(['.'; 7]);
        }
        loop {
            if jet[jet_index] == '<' {
                if x + rock[0].len() < 7 && can_move(rock, &board, y, x, 1, 0) {
                    x += 1;
                }
            } else {
                if x > 0 && can_move(rock, &board, y, x, -1, 0) {
                    x -= 1;
                }
            }
            jet_index = (jet_index + 1) % jet.len();
            if y == 0 || !can_move(rock, &board, y, x, 0, -1) {
                break;
            }
            y -= 1;
        }
        for i in 0..rock.len() {
            for j in 0..rock[0].len() {
                if rock[i][j] == '#' {
                    board[i + y][j + x] = rock[i][j];
                }
            }
        }
        max_height = max_height.max(y + rock.len());
        rock_index = (rock_index + 1) % rocks.len();
        step += 1;
        if !is_cycle_found && rock_index == 0 && step > 10 {
            let (prev_step, prev_max_heigh, prev_board) = &seen_indices[jet_index];
            let look_back_start = max_height - look_back;
            if *prev_step != usize::MAX && *prev_board == board[look_back_start..max_height] {
                let cycle_length = step - prev_step;
                let remaining_steps = max_steps - step;
                let height_diff = max_height - prev_max_heigh;
                let remaining_cycles = remaining_steps / cycle_length;
                cycle_height = remaining_cycles * height_diff;
                step += remaining_cycles * cycle_length;
                is_cycle_found = true;
            }
            let board_look_back = board[look_back_start..max_height].to_vec();
            seen_indices[jet_index] = (step, max_height, board_look_back);
        }
    }
    let height = max_height + cycle_height;
    height
}

fn can_move(
    rock: &Vec<Vec<char>>,
    board: &Vec<[char; 7]>,
    y: usize,
    x: usize,
    dx: i32,
    dy: i32,
) -> bool {
    rock.iter().enumerate().all(|(i, line)| {
        line.iter().enumerate().all(|(j, c)| {
            let cy = (y as i32 + i as i32 + dy) as usize;
            let cx = (x as i32 + j as i32 + dx) as usize;
            *c == '.' || board[cy][cx] == '.'
        })
    })
}

fn parse_input(path: &str) -> (Vec<char>, Vec<Vec<Vec<char>>>) {
    let jet = std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .chars()
        .collect::<Vec<_>>();
    let rocks = "####
    
    .#.
    ###
    .#.
    
    ..#
    ..#
    ###
    
    #
    #
    #
    #
    
    ##
    ##"
    .split("    \n")
    .map(new_rock)
    .collect();
    (jet, rocks)
}

fn new_rock(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .rev()
        .map(|l| l.trim().chars().rev().collect())
        .collect()
}
