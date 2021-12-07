#![feature(int_abs_diff)]
use std::{
    env, fs,
    io::{self, Read},
};

fn fold_decimal(acc: u64, chr: u8) -> u64 {
    acc * 10 + (chr - b'0') as u64
}

fn parse(input: Vec<u8>) -> Vec<u64> {
    let mut crabs = vec![];
    let mut num = 0u64;
    let last = *input.last().unwrap();
    for digit in input {
        match digit {
            b'0'..=b'9' => {
                num = fold_decimal(num, digit);
            }
            b',' | b'\n' => {
                crabs.push(num);
                num = 0u64;
            }
            _ => (),
        }
    }
    if (b'0'..=b'9').contains(&last) {
        crabs.push(num);
    }
    crabs
}

/// 0.5x^2 + 0.5x
/// or
/// 0.5x(x+1)
/// or
/// (x(x+1))/2
fn p2_cost(abs_s: u64) -> u64 {
    (abs_s * (abs_s + 1)) / 2
}

fn solve(crabs: &mut Vec<u64>) -> (u64, u64) {
    let is_odd = crabs.len() % 2 == 1;
    let med_pos = crabs.len() / 2;
    let median = if is_odd {
        let (_, select, _) = crabs.select_nth_unstable(med_pos + 1);
        *select
    } else {
        let (smaller, rhs_m, _) = crabs.select_nth_unstable(med_pos);
        let (_, lhs_m, _) = smaller.select_nth_unstable(smaller.len() - 1);
        (*lhs_m + *rhs_m) / 2
    };
    let part1 = crabs.iter().map(|&crab| crab.abs_diff(median)).sum::<u64>();

    let mean = crabs.iter().sum::<u64>() / crabs.len() as u64;
    let part2_guess1 = crabs
        .iter()
        .map(|&crab| p2_cost(crab.abs_diff(mean)))
        .sum::<u64>();
    let part2_guess2 = crabs
        .iter()
        .map(|&crab| p2_cost(crab.abs_diff(mean + 1)))
        .sum::<u64>();
    (part1, part2_guess1.min(part2_guess2))
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
