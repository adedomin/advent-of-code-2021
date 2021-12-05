use std::{
    env, fs,
    io::{self, Read},
};

fn fold_decimal(acc: i32, chr: u8) -> i32 {
    acc * 10 + (chr - b'0') as i32
}

fn generate_coord(x: i32, y: i32, sx: i32) -> usize {
    (x + (y * (sx + 1))) as usize
}

fn add_points(
    flat_points: &mut Vec<(i32, i32)>,
    points: &mut Vec<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
) {
    let (mut x1, mut y1) = start;
    let (x2, y2) = end;

    if x1 == x2 {
        let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };
        for y in range {
            flat_points.push((x1, y));
        }
    } else if y1 == y2 {
        let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
        for x in range {
            flat_points.push((x, y1));
        }
    } else {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let slopex = if x1 < x2 { 1 } else { -1 };
        let slopey = if y1 < y2 { 1 } else { -1 };

        let mut error = dx - dy;
        loop {
            points.push((x1, y1));

            if x1 == x2 && y1 == y2 {
                break;
            }

            let error2 = error << 1;
            if error2 > -dy {
                error -= dy;
                x1 += slopex;
            }
            if error2 < dx {
                error += dx;
                y1 += slopey;
            }
        }
    }
}

fn parse(input: Vec<u8>) -> ((i32, i32), Vec<(i32, i32)>, Vec<(i32, i32)>) {
    // input length is a good "guesstimate"
    let mut flat_points = Vec::<(i32, i32)>::with_capacity(input.len() / 4);
    let mut points = Vec::<(i32, i32)>::with_capacity(input.len() / 4);

    let mut max_point = (0, 0);

    let mut curr_num = 0;
    let mut start_end = [0i32; 4];
    let mut idx_start_end = 0;
    for chr in input {
        match chr {
            b'0'..=b'9' => curr_num = fold_decimal(curr_num, chr),
            b',' | b'>' => {
                start_end[idx_start_end] = curr_num;
                idx_start_end += 1;
                curr_num = 0;
            }
            b'\n' if idx_start_end == 3 => {
                start_end[3] = curr_num;
                idx_start_end = 0;
                curr_num = 0;
                add_points(
                    &mut flat_points,
                    &mut points,
                    (start_end[0], start_end[1]),
                    (start_end[2], start_end[3]),
                );
                if start_end[0] > max_point.0 {
                    max_point.0 = start_end[0]
                }
                if start_end[2] > max_point.0 {
                    max_point.0 = start_end[2]
                }
                if start_end[1] > max_point.1 {
                    max_point.1 = start_end[0]
                }
                if start_end[3] > max_point.1 {
                    max_point.1 = start_end[2]
                }
                start_end[0] = 0;
                start_end[1] = 0;
                start_end[2] = 0;
                start_end[3] = 0;
            }
            b'\n' if idx_start_end > 0 => {
                panic!("Invalid user input.")
            }
            _ => (),
        }
    }
    (max_point, flat_points, points)
}

fn solve(
    max_point: (i32, i32),
    flat_points: Vec<(i32, i32)>,
    points: Vec<(i32, i32)>,
) -> (usize, usize) {
    let mut flat_num_intersect = 0;
    let mut num_intersect = 0;

    let mut graph = vec![0u16; (max_point.0 + 1) as usize * (max_point.1 + 1) as usize];
    for (x, y) in flat_points {
        let coord = generate_coord(x, y, max_point.0);
        graph[coord] += 1;
        if graph[coord] == 2 {
            flat_num_intersect += 1;
        }
    }
    for (x, y) in points {
        let coord = generate_coord(x, y, max_point.0);
        graph[coord] += 1;
        if graph[coord] == 2 {
            num_intersect += 1;
        }
    }
    (flat_num_intersect, num_intersect + flat_num_intersect)
}

pub fn main() -> io::Result<()> {
    let input = match env::args().nth(1) {
        Some(arg) => fs::read(arg)?,
        None => {
            let mut buf = vec![];
            io::stdin().lock().read_to_end(&mut buf)?;
            buf
        }
    };
    let (mp, fp, p) = parse(input);
    let (p1, p2) = solve(mp, fp, p);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
