use std::env;

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day1.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day1 <input_file>");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let string_to_compute = match args[1].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[1])?,
    };
    let mut floor = 0;
    let mut counter = 0;
    let mut first_bassement = false;

    for char in string_to_compute.chars().into_iter() {
        counter += 1;
        if char == '(' {
            floor += 1;
        } else if char == ')' {
            floor -= 1;
        }
        if (floor == -1) & !first_bassement {
            println!("Reached bassement for the first time at instruction {counter}");
            first_bassement = true;
        }
    }

    println!("In {floor} floor");
    Ok(())
}
