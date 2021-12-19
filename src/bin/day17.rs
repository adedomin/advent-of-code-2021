use std::io;

use advent_of_code_2021::{fold_decimal, read_input, AoCTokenizer, Token};

#[derive(Debug)]
struct Points {
    xmax: i32,
    ymax: i32,
    xmin: i32,
    ymin: i32,
}

enum Parse {
    Nothing,
    X,
    Y,
}

fn parse(input: Vec<u8>) -> Points {
    let (point, _, _) = AoCTokenizer::new(&input).fold(
        (
            Points {
                xmax: i32::MIN,
                ymax: i32::MIN,
                xmin: i32::MAX,
                ymin: i32::MAX,
            },
            Parse::Nothing,
            1,
        ),
        |(mut acc, state, neg), tok| match tok {
            Token::Something(str) if str == b"x" => (acc, Parse::X, 1),
            Token::Something(str) if str == b"y" => (acc, Parse::Y, 1),
            Token::Delimiter(del) if del == b'-' => (acc, state, -1),
            Token::Something(num) if num != b"target" && num != b"area" => {
                let n = if neg == -1 {
                    neg * num.iter().fold(0i32, fold_decimal)
                } else {
                    num.iter().fold(0i32, fold_decimal)
                };
                if let Parse::X = state {
                    if acc.xmax < n {
                        acc.xmax = n;
                    }
                    if acc.xmin > n {
                        acc.xmin = n
                    }
                } else if let Parse::Y = state {
                    if acc.ymax < n {
                        acc.ymax = n;
                    }
                    if acc.ymin > n {
                        acc.ymin = n
                    }
                }
                (acc, state, 1)
            }
            _ => (acc, state, 1),
        },
    );
    point
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let area_points = parse(input);
    // Take the lowest area and knowing that y velocity decreases by
    // -1, we're basically just looking for the highest y that will fall into this point.
    let p1 = (area_points.ymin.abs() * (area_points.ymin.abs() - 1)) / 2;
    println!("Part1 {:?}, Part2 {}", p1, 0);
    Ok(())
}
