use std::{
    env, fs,
    io::{self, Read},
    u128,
};

// fish are only [0, 8]
fn parse(input: Vec<u8>) -> Vec<u8> {
    input.iter().fold(vec![], |mut fish, &digit| match digit {
        b'0'..=b'8' => {
            fish.push(digit - b'0');
            fish
        }
        b',' | b'\n' => fish,
        _ => panic!("invalid input."),
    })
}

const PART_1_CNT: u64 = 80;
const PART_2_CNT: u64 = 256;

fn m_idx<const RLEN: usize>(x: usize, y: usize) -> usize {
    x * RLEN + y
}

fn setup_resi_mat<const RLEN: usize, const DIM: usize>() -> [u128; DIM] {
    let mut residue = [0u128; DIM];
    for idx in 0..RLEN {
        residue[m_idx::<RLEN>(idx, idx)] = 1;
    }
    residue
}

fn mult_mat<const RLEN: usize, const DIM: usize>(
    lhs: &[u128; DIM],
    rhs: &[u128; DIM],
) -> [u128; DIM] {
    let mut result_set = [0u128; DIM];

    for i in 0..RLEN {
        for j in 0..RLEN {
            for k in 0..RLEN {
                result_set[m_idx::<RLEN>(i, j)] +=
                    lhs[m_idx::<RLEN>(i, k)] * rhs[m_idx::<RLEN>(k, j)];
            }
        }
    }
    result_set
}

fn pow_mat<const RLEN: usize, const DIM: usize>(matrix: &[u128; DIM], exp: u64) -> [u128; DIM] {
    let mut residue = setup_resi_mat::<RLEN, DIM>();
    let mut matrix_clone = matrix.map(|x| x);
    let mut ex = exp;

    while ex - 1 != 0 {
        if ex % 2 != 0 {
            residue = mult_mat::<RLEN, DIM>(&residue, &matrix_clone);
        }
        matrix_clone = mult_mat::<RLEN, DIM>(&matrix_clone, &matrix_clone);
        ex /= 2;
    }
    mult_mat::<RLEN, DIM>(&matrix_clone, &residue)
}

fn solve_sum<const RLEN: usize>(exp_mat: &[u128], fishy: &[u128]) -> u128 {
    let mut sum = 0;
    for i in 0..RLEN {
        for j in 0..RLEN {
            sum += exp_mat[m_idx::<RLEN>(i, j)] * fishy[j] as u128;
        }
    }
    sum
}

fn solve(fish: Vec<u8>, custom_inp: u64) -> (u128, u128, u128) {
    let fishcnt = fish.iter().fold([0u128; 9], |mut acc, &f| {
        acc[f as usize] += 1;
        acc
    });
    #[rustfmt::skip]
    let recur_mat = [
        0, 1, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0
    ];

    let part1_exp_mat = pow_mat::<9, 81>(&recur_mat, PART_1_CNT);
    let part1 = solve_sum::<9>(&part1_exp_mat, &fishcnt);

    let part2_exp_mat = pow_mat::<9, 81>(&recur_mat, PART_2_CNT);
    let part2 = solve_sum::<9>(&part2_exp_mat, &fishcnt);

    let custom = if custom_inp != 0 {
        let custom_exp_mat = pow_mat::<9, 81>(&recur_mat, custom_inp);
        solve_sum::<9>(&custom_exp_mat, &fishcnt)
    } else {
        0
    };

    (part1, part2, custom)
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

    let custom_input = match env::args().nth(2) {
        Some(val) => val.parse::<u64>().unwrap(),
        None => 0u64,
    };

    let fish = parse(input);
    let (p1, p2, c) = solve(fish, custom_input);
    println!("Part1 {}, Part2 {}, Custom {}", p1, p2, c);
    Ok(())
}
