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

    let mut total_string_length_with_quotes: usize = 0;
    let mut total_string_length: usize = 0;

    for line in string_to_compute.lines() {
        total_string_length_with_quotes += line.len();

        let line = line.trim_matches('"').to_string();
        let mut pointer: usize = 0;
        let line_length = line.len();
        let mut string_length = line_length;

        while (pointer as isize) < (line_length as isize - 1) {
            if line.get(pointer..=pointer).unwrap() == "\\" {
                match line.get(pointer + 1..=pointer + 1).unwrap() {
                    "x" => {
                        string_length -= 3;
                        pointer += 3;
                    }
                    "\\" => {
                        string_length -= 1;
                        pointer += 1;
                    }
                    "\"" => {
                        string_length -= 1;
                        pointer += 1;
                    }
                    _ => (),
                }
            }
            pointer += 1;
        }
        total_string_length += string_length;
    }

    println!(
        "Total string length with quotes: {}",
        total_string_length_with_quotes
    );
    println!("Total string length: {}", total_string_length);
    println!(
        "Difference: {}",
        total_string_length_with_quotes - total_string_length
    );

    Ok(())
}
