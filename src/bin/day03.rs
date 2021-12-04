use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read},
};

use itertools::Itertools;

#[derive(PartialEq)]
enum Sensor {
    O2,
    CO2,
}

fn parse_and_build_ds(input: Vec<u8>) -> ([i32; 32], HashMap<(u32, usize), i64>, usize) {
    input
        .iter()
        .rev()
        .group_by(|&&chr| chr != b'\n')
        .into_iter()
        .filter(|(grp_type, _)| *grp_type)
        .map(|(_, digits)| {
            digits.fold((0u32, 0usize), |(acc, idx), &digit| {
                (acc | ((digit - b'0') as u32) << idx, idx + 1)
            })
        })
        .fold(
            ([0i32; 32], HashMap::new(), 0usize),
            |(mut common_bits, mut prefix_map, _), (n, bitsiz)| {
                let offset = common_bits.len() - bitsiz;
                let mut prefix = 0u32;
                for pos in offset..common_bits.len() {
                    let shift_by = (common_bits.len() - 1) - pos;
                    if n & (1 << shift_by) != 0 {
                        common_bits[pos] += 1;
                        prefix |= 1 << shift_by;
                    } else {
                        common_bits[pos] -= 1;
                    }
                    let key = (prefix, shift_by);
                    match prefix_map.get_mut(&key) {
                        Some(refer) => *refer += 1i64,
                        None => {
                            prefix_map.insert(key, 1i64);
                        }
                    }
                }
                (common_bits, prefix_map, offset)
            },
        )
}

fn find_sensor_val(
    stype: Sensor,
    prefix_map: &HashMap<(u32, usize), i64>,
    prefix: u32,
    pos: usize,
) -> u32 {
    if pos == 0 {
        return prefix;
    }

    let num_ones = *prefix_map
        .get(&(prefix | (1 << (pos - 1)), pos - 1))
        .unwrap_or(&0);
    let num_zeros = *prefix_map.get(&(prefix, pos - 1)).unwrap_or(&0);

    if num_ones == 0 {
        find_sensor_val(stype, prefix_map, prefix, pos - 1)
    } else if num_zeros == 0 {
        find_sensor_val(stype, prefix_map, prefix | (1 << (pos - 1)), pos - 1)
    } else {
        let new_pre = match stype {
            Sensor::O2 => {
                if num_ones - num_zeros >= 0 {
                    prefix | (1 << (pos - 1))
                } else {
                    prefix
                }
            }
            Sensor::CO2 => {
                if num_ones - num_zeros >= 0 {
                    prefix
                } else {
                    prefix | (1 << (pos - 1))
                }
            }
        };
        find_sensor_val(stype, prefix_map, new_pre, pos - 1)
    }
}

fn solve(input: Vec<u8>) -> (u32, u32) {
    let (common_bits, prefix_map, offset) = parse_and_build_ds(input);

    let common_bits = &common_bits[offset..];
    let bitsize = common_bits.len();
    let gamma = common_bits
        .iter()
        .enumerate()
        .fold(0u32, |acc, (idx, &discrim)| {
            if discrim >= 0 {
                acc | (1 << ((bitsize - 1) - idx))
            } else {
                acc
            }
        });
    let epsilon = gamma ^ (2u32.pow(bitsize as u32) - 1);

    let oxy_sensor = find_sensor_val(Sensor::O2, &prefix_map, 0, bitsize);
    let co2_sensor = find_sensor_val(Sensor::CO2, &prefix_map, 0, bitsize);
    (gamma * epsilon, oxy_sensor * co2_sensor)
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
    let (p1, p2) = solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
