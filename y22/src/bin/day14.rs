fn main() {
    let (mut board, min_x) = parse_board("res/day14.txt");
    let max_y = board.len() - 1;
    board.push(vec!['.'; board[0].len()]);
    let mut source = 500 - min_x;
    let mut sand_count = 0;
    let mut sand_count_before_overflow = 0;
    while board[0][source] == '.' {
        let [x, y] = fall(&mut source, &mut board);
        if y >= max_y && sand_count_before_overflow == 0 {
            sand_count_before_overflow = sand_count;
        }
        sand_count += 1;
        board[y][x] = 'o';
    }
    println!("{sand_count_before_overflow}");
    println!("{sand_count}");
}

fn fall(source: &mut usize, board: &mut Vec<Vec<char>>) -> [usize; 2] {
    let mut x = *source;
    for next_y in 1..board.len() {
        if board[next_y][x] == '.' {
            continue;
        }
        if x as isize - 1 < 0 {
            for row in board.iter_mut() {
                row.insert(0, '.');
            }
            *source += 1;
            x += 1;
        }
        if board[next_y][x - 1] == '.' {
            x -= 1;
            continue;
        }
        if x + 1 == board[0].len() {
            for row in board.iter_mut() {
                row.push('.');
            }
        }
        if board[next_y][x + 1] == '.' {
            x += 1;
            continue;
        }
        return [x, next_y - 1];
    }
    [x, board.len() - 1]
}

fn parse_board(path: &str) -> (Vec<Vec<char>>, usize) {
    let paths = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| p.split_once(",").unwrap())
                .map(|(x, y)| [x.parse().unwrap(), y.parse().unwrap()])
                .collect::<Vec<[usize; 2]>>()
        })
        .collect::<Vec<_>>();
    let mut min_x = usize::MAX;
    let mut max = [usize::MIN; 2];
    for path in &paths {
        for [x, y] in path {
            min_x = min_x.min(*x);
            max = [max[0].max(*x), max[1].max(*y)];
        }
    }
    let mut board = vec![vec!['.'; max[0] - min_x + 1]; max[1] + 1];
    for path in &paths {
        for i in 0..path.len() - 1 {
            let [x1, y1] = path[i];
            let [x2, y2] = path[i + 1];
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    board[y][x - min_x] = '#';
                }
            }
        }
    }
    (board, min_x)
}
