mod io;

use io::{ArgOptions, PromptHandler};

pub fn main() -> Result<PromptHandler, String> {
    let options = ArgOptions::new();
    let prompt_handler = PromptHandler::from(options);
    dbg!(&prompt_handler);

    Ok(prompt_handler)
}
