#![feature(array_windows)]
use std::{env, fs, io};

/// Where N - 1 is the window size
pub fn solve<const N: usize>(input: &[u64]) -> u64 {
    input
        .array_windows::<N>()
        .filter(|&w| w[0] < w[N - 1])
        .count() as u64
}

pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|i| i.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

pub fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap_or_else(|| "/dev/stdin".to_owned());
    let input = fs::read_to_string(filename)?;
    let input = input_generator(&input);
    println!("Part1 {}, Part2 {}", solve::<2>(&input), solve::<4>(&input));
    Ok(())
}
