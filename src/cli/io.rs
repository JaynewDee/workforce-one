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

use crate::db::{Department, Employee, Role};
use std::io::prelude::*;
use std::io::stdout;

type AddParams = Vec<String>;
pub struct ArgOptions {
    view: Option<String>,
    add: Option<AddParams>,
}

impl ArgOptions {
    pub fn new() -> ArgOptions {
        let mut view = None;
        let mut add = None;

        let args = std::env::args().collect::<Vec<String>>();

        // Check for view argument
        if args.iter().any(|arg| arg == "v") {
            // Get position of "v" flag in args vec
            let arg_index = args.iter().position(|item| item == "v").unwrap() + 1;

            // Check for argument following flag
            if arg_index < args.len() {
                // if index is len or greater, it doesn't exist in args vec
                view = Some(args[arg_index].clone());
            } else {
                println!("No argument was passed to `view` flag ... ")
            }
        } else {
            println!("Unknown argument(s)");
        }

        // Check for add argument
        // Check for values
        if args.iter().any(|arg| arg == "add") {
            let table = args.iter().position(|item| item == "add").unwrap() + 1;
        }

        Self { view, add }
    }

    pub fn view(&self) -> Option<String> {
        self.view.clone()
    }

    pub fn add(&self) -> Option<Vec<String>> {
        self.add.clone()
    }
    fn flush_out() {
        stdout()
            .flush()
            .expect("Should have flushed stdout stream ... ")
    }
}
