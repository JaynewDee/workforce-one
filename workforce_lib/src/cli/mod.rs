mod io;

use io::{ArgOptions, PromptHandler};

pub fn main() -> Result<PromptHandler, String> {
    let options = ArgOptions::new();
    let prompt_handler = PromptHandler::from(options);

    Ok(prompt_handler)
}
