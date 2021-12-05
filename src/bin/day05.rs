use std::{
    collections::HashSet,
    env, fs,
    io::{self, Read},
};

fn fold_decimal(acc: i32, chr: u8) -> i32 {
    acc * 10 + (chr - b'0') as i32
}

fn add_points(
    point_set_p1: &mut HashSet<(i32, i32)>,
    point_set: &mut HashSet<(i32, i32)>,
    result_set_p1: &mut HashSet<(i32, i32)>,
    result_set: &mut HashSet<(i32, i32)>,
    start: (i32, i32),
    end: (i32, i32),
) {
    let (mut x1, mut y1) = start;
    let (x2, y2) = end;
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let slopex = if x1 < x2 { 1 } else { -1 };
    let slopey = if y1 < y2 { 1 } else { -1 };
    let flat = x1 == x2 || y1 == y2;
    let mut error = dx - dy;

    loop {
        if point_set.contains(&(x1, y1)) {
            result_set.insert((x1, y1));
        } else {
            point_set.insert((x1, y1));
        }

        // Part1 specific
        if flat && point_set_p1.contains(&(x1, y1)) {
            result_set_p1.insert((x1, y1));
        } else if flat {
            point_set_p1.insert((x1, y1));
        }

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

fn parse_and_solve(input: Vec<u8>) -> (usize, usize) {
    let mut point_set_p1 = HashSet::<(i32, i32)>::new();
    let mut point_set = HashSet::<(i32, i32)>::new();
    // only x1 = x2 || y1 = y2
    let mut result_set_p1 = HashSet::<(i32, i32)>::new();
    let mut result_set = HashSet::<(i32, i32)>::new();
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
                    &mut point_set_p1,
                    &mut point_set,
                    &mut result_set_p1,
                    &mut result_set,
                    (start_end[0], start_end[1]),
                    (start_end[2], start_end[3]),
                );
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
    (result_set_p1.len(), result_set.len())
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
    let (p1, p2) = parse_and_solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
