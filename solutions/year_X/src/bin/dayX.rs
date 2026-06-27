use std::env;

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_X/src/input/dayX.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dayX <input_file>");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let string_to_compute = match args[1].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[1])?,
    };

    Ok(())
}
