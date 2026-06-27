use std::{collections::HashMap, env};

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day3.txt");
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) | (args.len() > 3) {
        println!("Usage: day3 [option] <input_file>");
        println!("Options: -r for robot santa");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let robot_santa: bool;
    let file_path_index: usize;// usize car le compilateur ne sait pas si une taille précise (u8) pourra être utilisé dans le vecteur dans la mesure où sa taille dépant du système
    match args[1].as_str() {
        "-r" => {
            robot_santa = true;
            file_path_index = 2;
        },
        _ => {
            robot_santa = false;
            file_path_index = 1;
        }
    }
    
    // usize car le compilateur ne sait pas si une taille précise (u8) pourra être utilisé dans le vecteur dans la mesure où sa taille dépant du système
    let string_to_compute = get_file_content_to_string(&args[file_path_index])?;
    let mut house_list: HashMap<[i32; 2], u8> = HashMap::new();

    // usize car le compilateur ne sait pas si une taille précise (u8) pourra être utilisé dans le vecteur dans la mesure où sa taille dépant du système
    let string_to_compute = match args[file_path_index].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[file_path_index])?,
    };
    let mut house_list: HashMap<[i32; 2], u8> = HashMap::new();

    let (mut x_santa, mut y_santa, mut x_robot, mut y_robot) = (0, 0, 0, 0);
    house_list.insert([0, 0], 1);
    let mut counter = 0;
    for char in string_to_compute.chars() {
        match char {
            '^' => {
                if robot_santa & ((counter % 2) == 1) {
                    y_robot += 1;
                    house_list.entry([x_robot, y_robot]).and_modify(|x| *x += 1).or_insert(1);
                } else {
                    y_santa += 1;
                    house_list.entry([x_santa, y_santa]).and_modify(|x| *x += 1).or_insert(1);
                }
            },
            'v' => {
                if robot_santa & ((counter % 2) == 1) {
                    y_robot -= 1;
                    house_list.entry([x_robot, y_robot]).and_modify(|x| *x += 1).or_insert(1);
                } else {
                    y_santa -= 1;
                    house_list.entry([x_santa, y_santa]).and_modify(|x| *x += 1).or_insert(1);
                }
            },
            '<' => {
                if robot_santa & ((counter % 2) == 1) {
                    x_robot -= 1;
                    house_list.entry([x_robot, y_robot]).and_modify(|x| *x += 1).or_insert(1);
                } else {
                    x_santa -= 1;
                    house_list.entry([x_santa, y_santa]).and_modify(|x| *x += 1).or_insert(1);
                }
            },
            '>' => {
                if robot_santa & ((counter % 2) == 1) {
                    x_robot += 1;
                    house_list.entry([x_robot, y_robot]).and_modify(|x| *x += 1).or_insert(1);
                } else {
                    x_santa += 1;
                    house_list.entry([x_santa, y_santa]).and_modify(|x| *x += 1).or_insert(1);
                }
            },
            _ => {continue;}
        }
        counter += 1;
    }

    println!("{} house with at least one present", house_list.len());

    Ok(())
}