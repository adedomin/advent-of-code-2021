use std::{env, fs, io};

fn parse_and_solve(input: String) -> (u64, u64) {
    let (_, hor, aim, dep) =
        input
            .split_ascii_whitespace()
            .fold((b'f', 0, 0, 0), |(dir, hor, aim, dep), tok| {
                let pre = *tok.as_bytes().get(0).unwrap();
                match pre {
                    b'f' | b'd' | b'u' => (pre, hor, aim, dep),
                    _ => {
                        let vec = tok.parse::<u64>().unwrap();
                        match dir {
                            b'f' => (dir, hor + vec, aim, dep + vec * aim),
                            b'd' => (dir, hor, aim + vec, dep),
                            b'u' => (dir, hor, aim - vec, dep),
                            _ => unreachable!(),
                        }
                    }
                }
            });
    (hor * aim /* part 1 */, hor * dep /* part 2 */)
}

pub fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap_or_else(|| "/dev/stdin".to_owned());
    let input = fs::read_to_string(filename)?;
    let (p1, p2) = parse_and_solve(input);
    println!("Part1 {}, Part2 {}", p1, p2);
    Ok(())
}
