use crate::db::{Department, Employee, Role};
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
pub fn parse_args() {
    let args: Vec<String> = std::env::args().collect();
    // `v` flag for `view` option
    if args.iter().any(|arg| arg == "v") {
        println!("Contains `value` flag");

        let arg_index = args.iter().position(|item| item == "v").unwrap() + 1;

        println!("{arg_index}");

        if arg_index < args.len() {
            println!("View argument: {:#?}", args[arg_index]);
        } else {
            println!("No argument was passed to `view` flag ... ")
        }
    } else {
        println!("Unknown argument(s)")
    }
}
