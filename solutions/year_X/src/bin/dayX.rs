use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: dayX <input_file>");
	    std::process::exit(1);
    }

    Ok(())
}