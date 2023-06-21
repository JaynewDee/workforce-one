mod io;

use io::parse_args;

pub fn main() -> Result<String, String> {
    Ok(parse_args())
}
