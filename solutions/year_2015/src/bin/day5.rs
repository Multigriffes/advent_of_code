use std::{collections::HashMap, env};

use input_file_lib::get_file_content_to_string;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const PROHIBITED_PAIRS: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day5.txt");
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        println!("Usage: day5 [option] <input_file>");
        println!("Options: -r for the second set of rule");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let condition: bool;
    let file_path_index: usize;
    match args[1].as_str() {
        "-r" => {
            condition = true;
            file_path_index = 2;
        },
        _ => {
            condition = false;
            file_path_index = 1;
        }
    }

    let string_to_compute = match args[file_path_index].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[file_path_index])?,
    };

    let mut counter: u32 = 0;
    match condition {
        false => {
            for line in string_to_compute.lines() {
                if is_nice_line_first(line) {
                    counter += 1;
                }
            }
        },
        true => {
            for line in string_to_compute.lines() {
                if is_nice_line_second(line) {
                    counter += 1;
                }
            }    
        }
    }
    println!("There is {} nice string", counter);

    Ok(())
}

fn is_nice_line_first(line: &str) -> bool {
    let mut vowels_counter: u32 = 0;
    let mut has_prohibited_pair = false;
    let mut letter_twice: bool = false;

    let mut last_letter: char = '_';
    
    for letter in line.chars() {
        let mut current_pair = String::new();
        current_pair.push(last_letter);
        current_pair.push(letter);
        for prohibited_pair in PROHIBITED_PAIRS {
            if !has_prohibited_pair{
                has_prohibited_pair = prohibited_pair == current_pair.as_str();
            }
        }

        if !letter_twice {
            letter_twice = last_letter == letter;
        }

        for vowel in VOWELS {
            if letter == vowel {
                vowels_counter += 1;
            }
        }
        
        last_letter = letter;
    }

    letter_twice & (vowels_counter >= 3) & !has_prohibited_pair
}

fn is_nice_line_second(line: &str) -> bool {
    let mut letter_twice: bool = false;
    let pair_twice: bool;

    let mut last_letter: char = '_';
    let mut last_last_letter: char = '_';
    let mut pairs: HashMap<String, u8> = HashMap::new();
    let mut last_pair = String::new();
    let mut last_last_pair = String::new(); // cas "aaaa", le last_pair va empecher le comptage de la pair "aa"

    for letter in line.chars() {
        let mut current_pair = String::new();
        current_pair.push(last_letter);
        current_pair.push(letter);

        if (current_pair != last_pair) | (current_pair == last_last_pair) {
            pairs.entry(current_pair.clone())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        
        if !letter_twice {
            letter_twice = last_last_letter == letter;
        }

        last_last_letter = last_letter;
        last_letter = letter;
        last_last_pair = last_pair;
        last_pair = current_pair;
    }
    
    pair_twice = pairs.values()
        .map(|x| *x > 1)
        .fold(false, |acc, x| acc | x);

    pair_twice & letter_twice
}