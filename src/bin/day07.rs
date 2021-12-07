#![feature(int_abs_diff)]
use std::{
    env, fs,
    io::{self, Read},
};

fn fold_decimal(acc: u32, chr: &u8) -> u32 {
    acc * 10 + (chr - b'0') as u32
}

fn parse(input: Vec<u8>) -> Vec<u32> {
    let crabs = input
        .split(|&chr| chr == b',' || chr == b'\n')
        .into_iter()
        .fold(Vec::<u32>::new(), |mut acc, digits| {
            if digits.is_empty() {
                acc
            } else {
                acc.push(digits.iter().fold(0u32, fold_decimal));
                acc
            }
        });
    crabs
}

/// 0.5x^2 + 0.5x
/// or
/// 0.5x(x+1)
/// or
/// (x(x+1))/2
fn p2_cost(abs_s: u32) -> u32 {
    (abs_s * (abs_s + 1)) / 2
}

fn solve(crabs: &mut Vec<u32>) -> (u32, u32) {
    let is_odd = crabs.len() % 2 == 1;
    let med_pos = crabs.len() / 2;
    let median = if is_odd {
        let (_, select, _) = crabs.select_nth_unstable_by(med_pos + 1, |&lhs, rhs| lhs.cmp(rhs));
        *select
    } else {
        let (smaller, rhs_m, _) = crabs.select_nth_unstable_by(med_pos, |&lhs, rhs| lhs.cmp(rhs));
        let (_, lhs_m, _) =
            smaller.select_nth_unstable_by(smaller.len() - 1, |&lhs, rhs| lhs.cmp(rhs));
        (*lhs_m + *rhs_m) / 2
    };
    let part1 = crabs.iter().map(|&crab| crab.abs_diff(median)).sum::<u32>();

    let mean = crabs.iter().sum::<u32>() / crabs.len() as u32;
    let part2 = crabs
        .iter()
        .map(|&crab| p2_cost(crab.abs_diff(mean)))
        .sum::<u32>();
    (part1, part2)
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
    let mut crabs = parse(input);
    let (p1, p2) = solve(&mut crabs);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
