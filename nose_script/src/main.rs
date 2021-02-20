use std::env;

mod command_parse;
mod common;
mod welcome;

mod commands {}

fn main() {
    let args= env::args().collect();

    welcome::nose_script_intro();
    command_parse::parse_command(args);
}