mod io;

use io::ArgOptions;

pub fn main() -> Result<ArgOptions, String> {
    Ok(ArgOptions::new())
}
