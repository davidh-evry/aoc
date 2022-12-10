use y22::read_lines;

fn main() {
    let lines = read_lines("res/day10.txt");
    let step = 40;
    let mut next = 20;
    let mut clock = 1;
    let mut x_reg = 1;
    let mut sum = 0;
    let mut screen = vec![['.'; 40]; 6];
    for line in lines {
        let is_noop = line == "noop";
        let clock_ticks = if is_noop { 1 } else { 2 };
        for _ in 0..clock_ticks {
            let i = clock - 1;
            let (x, y) = (i % step, i / step);
            if x as i32 >= x_reg - 1 && x as i32 <= x_reg + 1 {
                screen[y][x] = '#';
            }
            if clock == next {
                let val = x_reg * next as i32;
                sum += val;
                next += step;
            }
            clock += 1;
        }
        if !is_noop {
            let (_, r) = line.split_once(" ").unwrap();
            x_reg += r.parse::<i32>().unwrap();
        }
    }
    println!("{sum}");
    for scan_line in screen {
        println!("{}", String::from_iter(scan_line));
    }
}
