use std::env;

use input_file_lib::get_file_content_to_string;
use md5::{Digest, Md5};

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day4.txt");
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        println!("Usage: day4 [option] <input_file>");
        println!("Options: --six for six zeroes else 5");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let six_zeroes: bool;
    let file_path_index: usize;
    match args[1].as_str() {
        "--six" => {
            six_zeroes = true;
            file_path_index = 2;
        },
        _ => {
            six_zeroes = false;
            file_path_index = 1;
        }
    }

    let string_to_compute = match args[file_path_index].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[file_path_index])?,
    };
    let mut counter: u32 = 1;
    let mut hasher = Md5::new();
    let secret = string_to_compute;
    let mut string_to_hash = String::new();
    loop {
        string_to_hash.clear();
        string_to_hash.push_str(&secret);
        string_to_hash.push_str(&counter.to_string());

        hasher.reset();
        hasher.update(string_to_hash.as_bytes());
        let hash = hasher.clone().finalize();
        if six_zeroes {
            if hash[..3] == [0, 0, 0] {
                //println!("{:?}", hash);
                println!("{}", counter);
                break;
            }
        }else
        if hash[..2] == [0, 0] {
            if hash[2] < 16 {
                //println!("{:?}", hash);
                println!("{}", counter);
                break;
            }
        }
        counter += 1
    }

    Ok(())
}