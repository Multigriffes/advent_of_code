use std::{env, error::Error};

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        println!("Usage: dayX [option] <input_file>");
        println!("Options: --arg for ");
	    std::process::exit(1);
    }

    let condition: bool;
    let file_path_index: usize;
    match args[1].as_str() {
        "--arg" => {
            condition = true;
            file_path_index = 2;
        },
        _ => {
            condition = false;
            file_path_index = 1;
        }
    }
    
    let string_to_compute = get_file_content_to_string(&args[file_path_index])?;

    Ok(())
}