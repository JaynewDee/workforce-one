/*
   Options:
   - view all departments
   - view all roles,
   - view all employees,
   - add a department,
   - add a role,
   - add an employee,
   - update an employee role
*/

use std::io::prelude::*;
use std::io::stdout;

#[derive(Debug)]
pub struct ArgOptions {
    view: Option<String>,
    add: Option<String>,
}

impl ArgOptions {
    pub fn new() -> Self {
        let mut view = None;
        let mut add = None;

        let args = std::env::args().collect::<Vec<String>>();

        for (idx, val) in args.iter().enumerate() {
            if val == "v" || val == "add" {
                if idx < args.len() {
                    let table = &args[idx + 1];

                    if val == "v" {
                        view = Some(table);
                    }
                    if val == "add" {
                        add = Some(table);
                    }
                }
            } else {
                println!("Unknown argument(s)");
            }
        }

        Self {
            view: view.cloned(),
            add: add.cloned(),
        }
    }

    pub fn view(&self) -> Option<String> {
        self.view.clone()
    }

    pub fn add(&self) -> Option<String> {
        self.add.clone()
    }

    fn flush_out() {
        stdout()
            .flush()
            .expect("Should have flushed stdout stream ... ")
    }
}

#[derive(Debug)]
pub struct PromptHandler {
    options: ArgOptions,
}

impl From<ArgOptions> for PromptHandler {
    fn from(options: ArgOptions) -> Self {
        PromptHandler { options }
    }
}
