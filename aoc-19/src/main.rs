mod days;
mod input_reader;
use days::{runner_factory, Days, Parts};

fn main() {
    println!("Hello, world!");
    let (day, part, path) = read_user_input();
    let user_specified_day = str_to_day_enum(day.as_str()).unwrap();
    let user_specified_part = str_to_part(part.as_str()).unwrap();

    let runner = runner_factory(&user_specified_day, &user_specified_part, path.as_str());
    let runner_output = runner.run();

    println!("Result: {}", runner_output);
}

fn str_to_day_enum(name: &str) -> Option<Days> {
    match name.trim().to_lowercase().as_str() {
        "day1" => Some(Days::Day1),
        _ => None,
    }
}

fn str_to_part(part_str: &str) -> Option<Parts> {
    match part_str.trim().to_lowercase().as_str() {
        "part1" => Some(Parts::Part1),
        "part2" => Some(Parts::Part2),
        _ => None,
    }
}

fn read_user_input() -> (String, String, String) {
    use std::io::{stdin, stdout, Write};

    let mut day = String::new();
    print!("Please enter day which you wish to run (e.g. day1): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut day)
        .expect("Did not enter a correct string");
    if let Some('\n') = day.chars().next_back() {
        day.pop();
    }
    if let Some('\r') = day.chars().next_back() {
        day.pop();
    }

    let mut part = String::new();
    print!("Please enter which part you wish to run (e.g. part1): ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut part)
        .expect("Did not enter a correct string");
    if let Some('\n') = part.chars().next_back() {
        part.pop();
    }
    if let Some('\r') = part.chars().next_back() {
        part.pop();
    }

    let mut path = String::new();
    print!("Please enter location of file with this days input: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut path)
        .expect("Did not enter a correct string");
    if let Some('\n') = path.chars().next_back() {
        path.pop();
    }
    if let Some('\r') = path.chars().next_back() {
        path.pop();
    }
    (day, part, path)
}
