use std::{env, fs, io};
pub fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap_or_else(|| "/dev/stdin".to_owned());
    let input = fs::read_to_string(filename)?;
    Ok(())
}
