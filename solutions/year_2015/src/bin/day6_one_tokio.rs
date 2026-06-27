use std::env;

use input_file_lib::get_file_content_to_string;
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), String> {
    let default_path: String = String::from("./solutions/year_2015/src/input/day6.txt");
    //println!("there");
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
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    for line in string_to_compute.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();
        //println!("{:?}", instruction);
        if (instruction.len() != 5) & (instruction.len() != 4) {
            //println!("there");
            continue;
        }
        match instruction[0] {
            "turn" => {
                match instruction[1] {
                    "on" => {
                        turn_on(&mut grid, &instruction[2], &instruction[4])
                    },
                    "off" => {
                        turn_off(&mut grid, &instruction[2], &instruction[4])
                    },
                    _ => {continue;}
                }
            },
            "toggle" => {
                toggle(&mut grid, &instruction[1], &instruction[3]);
            },
            _ => {continue;}
        }
    }

    let mut handles = Vec::new();
    for i in 0..1000 {
        //println!("{i}");
        let line = grid[i].clone();
        handles.push(spawn(count(line)));
    }
    
    let mut count_light: u32 = 0;
    for handle in handles {
        count_light += handle.await.unwrap();
    }

    println!("{} lights on", count_light);

    Ok(())
}


fn turn_on(grid: &mut Vec<Vec<bool>>, start: &str, end: &str) {
    //println!("Turn on");
    let start: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
    let end: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();
    for x in start[0]..=end[0] {
        for y in start[1]..=end[1] {
            grid[x][y] = true;
        }
    }
}

fn turn_off(grid: &mut Vec<Vec<bool>>, start: &str, end: &str) {
    //println!("Turn off");
    let start: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
    let end: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();
    for x in start[0]..=end[0] {
        for y in start[1]..=end[1] {
            grid[x][y] = false;
        }
    }
}

fn toggle(grid: &mut Vec<Vec<bool>>, start: &str, end: &str) {
    //println!("Toggle");
    let start: Vec<usize> = start.split(',').map(|x| x.parse().unwrap()).collect();
    let end: Vec<usize> = end.split(',').map(|x| x.parse().unwrap()).collect();
    for x in start[0]..=end[0] {
        for y in start[1]..=end[1] {
            grid[x][y] = !grid[x][y];
        }
    }
}

async fn count(line: Vec<bool>) -> u32 {
    let mut counter: u32 = 0;
    for light in line {
        if light == true {
            counter += 1;
        }
    }
    counter
}