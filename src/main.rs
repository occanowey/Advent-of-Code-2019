use std::env;
use std::fs;
use std::io::ErrorKind;

mod day1;

fn main() -> Result<(), String> {
    // day1::part1()
    day1::part2()
}

fn load_input() -> Result<String, String> {
    let mut args = env::args();
    args.next(); // program name

    let input_path = args.next()
        .ok_or( format!( "Missing input path" ) )?;

    fs::read_to_string( &input_path )
        .map_err( |e| match e.kind() {
            ErrorKind::NotFound => format!( "Could not find input: {}", input_path ),
            _ => format!( "Could not read input: {}", e ),
         } )
}