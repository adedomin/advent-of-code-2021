use std::io;

use advent_of_code_2021::read_input;

enum ParseState {
    SignalPatterns,
    Signals,
}

use itertools::Itertools;
use ParseState::{SignalPatterns, Signals};

fn next_line(chr: u8) -> ParseState {
    if chr == b'\n' {
        SignalPatterns
    } else {
        Signals
    }
}

const PIPE: u8 = 0b1000_0000;
const NPIPE: u8 = 0b0111_1111;

fn sidx(seg: u8) -> usize {
    (seg - b'a') as usize
}

///    __  a
/// g |_c| b
/// f |__| d
///    e
/// TRUTHS
/// 1 & !7 = a
/// 1 ^ 8 ^ 4 = abdef
/// 4 | a ^ 8 = ef
/// 7 ^ 8 ^ ef = cg
/// 4 | a = abcdg
/// 4 | a ^ 1 = acg
/// 4 | a ^ [6bit-num] = e | bef | cef
/// e ^ ef = f
/// cef ^ ef = c
/// bef | a ^ abdef = d
/// 7 ^ a ^ d = b
/// c ^ cg = g
fn solve(segments: &[u8]) -> u64 {
    // 0 6 9
    let mut sixers = [0u8; 3];
    let mut sixer_i = 0usize;
    // 2 3 5
    let mut fivers = [0u8; 3];
    let mut fiver_i = 0usize;

    let mut segment_digits = [0u8; 10];
    let mut segment_parts = [0u8; 7];

    let mut divider = 0usize;

    for (idx, &segment) in segments.iter().enumerate() {
        match segment.count_ones() {
            1 => divider = idx,
            2 => segment_digits[1] = segment,
            3 => segment_digits[7] = segment,
            4 => segment_digits[4] = segment,
            7 => segment_digits[8] = segment,
            5 if !fivers.contains(&segment) && fiver_i < fivers.len() => {
                fivers[fiver_i] = segment;
                fiver_i += 1;
            }
            5 => (),
            6 if !sixers.contains(&segment) && sixer_i < sixers.len() => {
                sixers[sixer_i] = segment;
                sixer_i += 1;
            }
            6 => (),
            _ => panic!("bad input"),
        }
    }

    segment_parts[sidx(b'a')] = segment_digits[1] ^ segment_digits[7] & NPIPE;
    if segment_parts[sidx(b'a')] == 0 {
        panic!("Invalid input");
    }
    let abdef = segment_digits[1] ^ segment_digits[8] ^ segment_digits[4] & NPIPE;
    let ef = segment_digits[4] ^ segment_parts[sidx(b'a')] ^ segment_digits[8] & NPIPE;
    let cg = segment_digits[7] ^ segment_digits[8] ^ ef & NPIPE;
    // let acg = segment_digits[4] | segment_parts[sidx(b'a')] ^ segment_digits[1] & NPIPE;

    for sixer in sixers {
        let four_a_xor_sixer = segment_digits[4] ^ segment_parts[sidx(b'a')] ^ sixer & NPIPE;
        if four_a_xor_sixer.count_ones() == 1 {
            segment_parts[sidx(b'e')] = four_a_xor_sixer;
        }
    }
    if segment_parts[sidx(b'e')] == 0 {
        panic!("Invalid input");
    }
    segment_parts[sidx(b'f')] = segment_parts[sidx(b'e')] ^ ef & NPIPE;

    for sixer in sixers {
        let four_a_xor_sixer = segment_digits[4] ^ segment_parts[sidx(b'a')] ^ sixer & NPIPE;
        if four_a_xor_sixer.count_ones() == 3 {
            let maybe_c = four_a_xor_sixer ^ ef & NPIPE;
            let maybe_g = maybe_c ^ cg & NPIPE;
            if maybe_g.count_ones() == 1 {
                segment_parts[sidx(b'c')] = maybe_c;
                segment_parts[sidx(b'g')] = maybe_g;
            } else {
                segment_parts[sidx(b'b')] = maybe_c;
            }
        }
    }
    if segment_parts[sidx(b'c')] == 0
        && segment_parts[sidx(b'g')] == 0
        && segment_parts[sidx(b'b')] == 0
    {
        panic!("Invalid input");
    }
    segment_parts[sidx(b'd')] =
        abdef ^ (segment_parts[sidx(b'a')] | segment_parts[sidx(b'b')] | ef);

    segment_digits[0] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'b')]
        | segment_parts[sidx(b'd')]
        | segment_parts[sidx(b'e')]
        | segment_parts[sidx(b'f')]
        | segment_parts[sidx(b'g')];
    // 1
    segment_digits[2] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'b')]
        | segment_parts[sidx(b'c')]
        | segment_parts[sidx(b'f')]
        | segment_parts[sidx(b'e')];
    segment_digits[3] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'b')]
        | segment_parts[sidx(b'c')]
        | segment_parts[sidx(b'd')]
        | segment_parts[sidx(b'e')];
    // 4
    segment_digits[5] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'g')]
        | segment_parts[sidx(b'c')]
        | segment_parts[sidx(b'd')]
        | segment_parts[sidx(b'e')];
    segment_digits[6] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'g')]
        | segment_parts[sidx(b'c')]
        | segment_parts[sidx(b'f')]
        | segment_parts[sidx(b'e')]
        | segment_parts[sidx(b'd')];
    // 7
    // 8
    segment_digits[9] = segment_parts[sidx(b'a')]
        | segment_parts[sidx(b'b')]
        | segment_parts[sidx(b'c')]
        | segment_parts[sidx(b'g')]
        | segment_parts[sidx(b'e')]
        | segment_parts[sidx(b'd')];

    let (_, display) = segments.split_at(divider);
    display.iter().skip(1).fold(0u64, |acc, &digit| {
        let decoded = segment_digits
            .iter()
            .find_position(|&&d| d == digit)
            .unwrap()
            .0 as u64;
        acc * 10 + decoded
    })
}

fn parse(mut input: Vec<u8>) -> (u64, u64) {
    let last = input.last().unwrap();
    // make sure it's line terminated on last statement
    if (b'a'..=b'g').contains(last) {
        input.push(b'\n');
    }
    let (numcnt, _, _) = input.iter().fold(
        (0u64, 0usize, SignalPatterns),
        |(cnt, digit_cnt, pstate), &digit| match pstate {
            SignalPatterns if digit == b'|' => (cnt, 0, Signals),
            SignalPatterns => (cnt, 0, pstate),
            Signals if (b'a'..=b'g').contains(&digit) => (cnt, digit_cnt + 1, pstate),
            Signals if (2..=4).contains(&digit_cnt) || digit_cnt == 7 => {
                (cnt + 1, 0, next_line(digit))
            }
            Signals => (cnt, 0, next_line(digit)),
        },
    );

    let (sum, _, _) = input.iter().fold(
        (0u64, Vec::<u8>::new(), 0u8),
        |(sum, mut segs, seg), &digit| match digit {
            b'|' => (sum, segs, PIPE),
            b'a'..=b'g' => (sum, segs, seg | (1 << (digit - b'a'))),
            b' ' if seg != 0u8 => {
                segs.push(seg);
                (sum, segs, 0)
            }
            b'\n' if seg != 0u8 => {
                segs.push(seg);
                let s = solve(&segs);
                segs.clear();
                (sum + s, segs, 0)
            }
            _ => (sum, segs, seg),
        },
    );

    (numcnt, sum)
}

pub fn main() -> io::Result<()> {
    let input = read_input()?;
    let (p1, p2) = parse(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
