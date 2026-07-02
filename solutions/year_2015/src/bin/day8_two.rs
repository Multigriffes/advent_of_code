use std::env;

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day8.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day8 <input_file>");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let string_to_compute = match args[1].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[1])?,
    };

    let mut total_string_length: usize = 0;
    let mut total_string_length_with_escape: usize = 0;

    for line in string_to_compute.lines() {
        total_string_length += line.len();
        total_string_length_with_escape += line.len();

        for char in line.chars() {
            if char == '"' || char == '\\' {
                total_string_length_with_escape += 1
            }
        }

        total_string_length_with_escape += 2;
    }

    println!("Total string length: {}", total_string_length);
    println!(
        "Total string length with escape: {}",
        total_string_length_with_escape
    );
    println!(
        "Difference: {}",
        total_string_length_with_escape - total_string_length
    );

    Ok(())
}
