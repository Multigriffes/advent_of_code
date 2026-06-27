use std::env;

use input_file_lib::get_file_content_to_string;

fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day2.txt");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: day2 <input_file>");
        println!("input_file could be 'x' or 'default' for the default one");
        std::process::exit(1);
    }

    let string_to_compute = match args[1].to_lowercase().as_str() {
        "x" | "default" => get_file_content_to_string(&default_path)?,
        _ => get_file_content_to_string(&args[1])?,
    };
    let mut total_surface: u32 = 0;
    let mut total_ribbon: u32 = 0;
    for line in string_to_compute.lines() {
        let dimensions: Vec<u32> = line
            .split('x')
            .map(|x| x.parse::<u32>())
            .filter(|x| match x {
                Ok(_) => true,
                Err(_) => false,
            })
            .map(|x| match x {
                Ok(number) => number,
                Err(_) => panic!("Prout !"),
            })
            .collect();

        if dimensions.len() != 3 {
            continue;
        }

        // Surface of wrapping paper
        let mut total_surface_this_present: u32 = 0;
        let mut min_surface: u32 = u32::MAX;
        for i in 0..3 {
            let surface = dimensions[i] * dimensions[(i + 1) % 3];
            if surface < min_surface {
                min_surface = surface;
            }
            total_surface_this_present += surface * 2
        }
        total_surface_this_present += min_surface;
        total_surface += total_surface_this_present;

        // Surface of ribbon
        let bow_ribbon = dimensions.iter().fold(1, |acc, x| acc * x);
        let mut min_ribbon = u32::MAX;
        for i in 0..3 {
            let perimeter = (dimensions[i] * 2) + (dimensions[(i + 1) % 3] * 2);
            if perimeter < min_ribbon {
                min_ribbon = perimeter
            }
        }
        total_ribbon += bow_ribbon + min_ribbon;
    }

    println!("The total is {total_surface} square feet of wrapping paper");
    println!("And {total_ribbon} square feet for the ribbon");

    Ok(())
}
