use std::{collections::VecDeque, io};

use advent_of_code_2021::read_input;

fn parse(input: &mut [u8]) -> (isize, isize) {
    let mut row_width = 0isize;
    input.iter_mut().enumerate().for_each(|(i, el)| {
        if row_width == 0 && *el == b'\n' {
            row_width = i as isize;
        } else if *el != b'\n' {
            *el -= b'0';
        }
    });
    (row_width, (input.len() - 1) as isize / (row_width + 1) + 1)
}

fn in_bounds(x: isize, y: isize, xmax: isize, ymax: isize) -> bool {
    if x < 0 || y < 0 {
        false
    } else {
        x < xmax && y < ymax
    }
}

fn xy(x: isize, y: isize, xmax: isize, ymax: isize) -> Option<isize> {
    if in_bounds(x, y, xmax, ymax) {
        let skip_factored_x = x + x / xmax;
        Some(skip_factored_x + y * (xmax + 1))
    } else {
        None
    }
}

fn neighbors(
    x: isize,
    y: isize,
    xmax: isize,
    ymax: isize,
) -> (Option<isize>, Option<isize>, Option<isize>, Option<isize>) {
    (
        xy(x, y - 1, xmax, ymax),
        xy(x + 1, y, xmax, ymax),
        xy(x, y + 1, xmax, ymax),
        xy(x - 1, y, xmax, ymax),
    )
}

fn solve(maze: Vec<u8>, xmax: isize, ymax: isize) -> (u64, u64) {
    let mut low_risk = 0u64;
    let mut lows = vec![];
    for y in 0..ymax {
        for x in 0..xmax {
            if let Some(position) = xy(x, y, xmax, ymax) {
                let node_value = maze[position as usize];
                let (north, east, south, west) = neighbors(x, y, xmax, ymax);
                let mut threshold = 0i32;
                let mut count = 0i32;

                if let Some(north) = north {
                    threshold += 1;
                    count += (node_value < maze[north as usize]) as i32;
                }
                if let Some(east) = east {
                    threshold += 1;
                    count += (node_value < maze[east as usize]) as i32;
                }
                if let Some(south) = south {
                    threshold += 1;
                    count += (node_value < maze[south as usize]) as i32;
                }
                if let Some(west) = west {
                    threshold += 1;
                    count += (node_value < maze[west as usize]) as i32;
                }

                if threshold == count {
                    lows.push((x, y, position));
                    low_risk += 1 + node_value as u64;
                }
            }
        }
    }

    let mut visited = vec![false; maze.len()];
    let mut largest_basins = [0u64, 0u64, 0u64];
    for (xlow, ylow, pos) in lows {
        if visited[pos as usize] {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back((xlow, ylow));
        let mut sum = 0;

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if let Some(idx) = xy(x, y, xmax, ymax) {
                if visited[idx as usize] {
                    continue;
                }

                let point_val = maze[idx as usize];
                if point_val == 9 {
                    continue;
                }

                visited[idx as usize] = true;
                sum += 1;

                let (n, e, s, w) = neighbors(x, y, xmax, ymax);

                if n.is_some() {
                    queue.push_back((x, y - 1));
                }

                if e.is_some() {
                    queue.push_back((x + 1, y));
                }

                if s.is_some() {
                    queue.push_back((x, y + 1));
                }

                if w.is_some() {
                    queue.push_back((x - 1, y));
                }
            }
        }

        if largest_basins[0] < sum {
            largest_basins[0] = sum;
            largest_basins.sort_unstable();
        }
    }
    (
        low_risk,
        largest_basins[0] * largest_basins[1] * largest_basins[2],
    )
}

pub fn main() -> io::Result<()> {
    let mut input = read_input()?;
    let (rowlen, collen) = parse(&mut input);
    let (p1, p2) = solve(input, rowlen, collen);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
