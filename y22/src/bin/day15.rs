use std::{collections::HashSet, ops::Range};

fn main() {
    let file_content = std::fs::read_to_string("res/day15.txt").unwrap();
    let mut sensors = vec![];
    for line in file_content.lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();
        let sx = split[2][2..split[2].len() - 1].parse::<i32>().unwrap();
        let sy = split[3][2..split[3].len() - 1].parse::<i32>().unwrap();
        let bx = split[8][2..split[8].len() - 1].parse::<i32>().unwrap();
        let by = split[9][2..split[9].len()].parse::<i32>().unwrap();
        let reach = sx.abs_diff(bx) + sy.abs_diff(by);
        sensors.push(Sensor {
            pos: [sx, sy],
            beacon: [bx, by],
            reach,
        });
    }
    find_no_beacons(2000000, &sensors);
    find_beacon(&sensors, 0, 4000000);
}

fn find_no_beacons(y_level: i32, sensors: &[Sensor]) {
    let row = get_row(sensors, y_level);
    let beacons_in_row = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b[1] == y_level)
        .collect::<HashSet<_>>();
    let beacon_count = beacons_in_row.len() as i32;
    let sum = row.iter().map(|r| r.end - r.start + 1).sum::<i32>() - beacon_count;
    println!("{sum}");
}

fn find_beacon(sensors: &[Sensor], min: i32, max: i32) {
    for y_level in min..=max {
        let row = get_row(sensors, y_level);
        for i in 1..row.len() {
            let (prev, range) = (&row[i - 1], &row[i]);
            let gap = prev.end + 1..range.start - 1;
            if gap.start <= gap.end && gap.start >= min && gap.end <= max {
                println!("{}", gap.start as u64 * 4000000 + y_level as u64);
                return;
            }
        }
    }
}

fn get_row(sensors: &[Sensor], y_level: i32) -> Vec<Range<i32>> {
    let mut row = vec![];
    for s in sensors {
        let y_distance = s.pos[1].abs_diff(y_level);
        if s.reach < y_distance {
            continue;
        }
        let row_reach = (s.reach - y_distance) as i32;
        add_range(&mut row, s.pos[0], row_reach);
    }
    row
}

fn add_range(row: &mut Vec<Range<i32>>, x: i32, row_reach: i32) {
    // can optimize with bin search
    let mut new_range = x - row_reach..x + row_reach;
    let len = row.len();
    for i in 0..len {
        let range = row[i].clone();
        if new_range.end < range.start {
            row.insert(i, new_range);
            return;
        }
        if new_range.start > range.end {
            continue;
        }
        row[i] = new_range.start.min(range.start)..range.end;
        if new_range.end <= range.end {
            return;
        }
        new_range = range.end + 1..new_range.end;
    }
    if let Some(range) = row.last() {
        if range.end + 1 == new_range.start {
            row[len - 1] = range.start..new_range.end;
            return;
        }
    }
    row.push(new_range);
}

#[derive(Debug)]
struct Sensor {
    pos: [i32; 2],
    beacon: [i32; 2],
    reach: u32,
}
