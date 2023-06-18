mod io;

use io::parse_args;

pub fn main() -> Result<(), String> {
    parse_args();

    Ok(())
}
