use std::{
    collections::HashSet,
    fmt::Debug,
    io,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use advent_of_code_2021::read_input;

type Vec2D<T> = Vec<Vec<T>>;

const BORDER: u8 = 255;

fn parse(input: Vec<u8>) -> Octopi {
    let row_width = input.iter().position(|&chr| chr == b'\n').unwrap();
    let col_len = ((input.len() - 1) / (row_width + 1)) + 1;

    let mut octopi = vec![vec![BORDER; row_width + 2]; col_len + 2];

    let mut i = 1usize;
    let mut j = 1usize;
    input.iter().for_each(|&el| {
        if el == b'\n' {
            i = 1;
            j += 1;
        } else if el != b'\n' {
            octopi[j][i] = el - b'0';
            i += 1;
        }
    });
    Octopi(octopi)
}

struct Octopi(Vec2D<u8>);

impl<Idx> Index<Idx> for Octopi
where
    Idx: SliceIndex<[std::vec::Vec<u8>]>,
{
    type Output = <Idx as SliceIndex<[Vec<u8>]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx> IndexMut<Idx> for Octopi
where
    Idx: SliceIndex<[std::vec::Vec<u8>]>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl Octopi {
    fn height(&self) -> usize {
        self.0.len() - 1
    }

    fn width(&self) -> usize {
        self.0[0].len() - 1
    }
}

impl Debug for Octopi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "= Octopi Map: Width {}, Height {}\n",
                self.width() - 1,
                self.height() - 1,
            )
            .as_str(),
        )?;
        for y in 1..self.height() {
            let mut line = String::new();
            for x in 1..self.width() {
                let octopi = self[y][x] + b'0';
                if octopi == b'0' {
                    line.push('*')
                } else {
                    line.push(' ');
                }
                line.push(octopi as char);
            }
            line.push('\n');
            f.write_str(&line)?;
        }
        Ok(())
    }
}

fn irridate_neighbors(
    octopi: &mut Octopi,
    flashset: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) {
    let mut stack = vec![(x, y)];

    while !stack.is_empty() {
        let (x, y) = stack.pop().unwrap();
        let octopus = &mut octopi[y][x];
        if *octopus == BORDER || flashset.contains(&(x, y)) {
            continue;
        }

        *octopus += 1;
        if *octopus == 10 {
            *octopus = 0;
            flashset.insert((x, y));
            stack.push((x, y - 1)); // north
            stack.push((x + 1, y - 1)); // north-east
            stack.push((x + 1, y)); // east
            stack.push((x + 1, y + 1)); // south-east
            stack.push((x, y + 1)); // south
            stack.push((x - 1, y + 1)); // south-west
            stack.push((x - 1, y)); // west
            stack.push((x - 1, y - 1)); // north-west
        }
    }
}

fn solve(mut octopi: Octopi) -> (usize, usize) {
    let mut run = 0usize;
    let mut flashes = 0usize;

    let mut flashset = HashSet::new();
    // println!("run  -0: {:?}", octopi);
    while flashset.len() != ((octopi.height() - 1) * (octopi.width() - 1)) {
        flashset.clear();
        for y in 1..octopi.height() {
            for x in 1..octopi.width() {
                let octopus = &mut octopi[y][x];
                if *octopus == 9 {
                    irridate_neighbors(&mut octopi, &mut flashset, x, y);
                } else if !flashset.contains(&(x, y)) {
                    *octopus += 1;
                }
            }
        }
        if run < 100 {
            flashes += flashset.len();
        }
        run += 1;
        // println!("run {:03}: {:?}", _runs + 1, octopi);
    }
    (flashes, run)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let parsed = parse(input);
    let (p1, p2) = solve(parsed);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
