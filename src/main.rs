use std::env;

pub mod days;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("Missing arg")
        }
        2 => {
            match args.get(1) {
                None => {
                    println!("Missing arg")
                }
                Some(arg) => {
                    println!("Got arg: {arg}");

                    if arg == "1.1" {
                        days::one::part_one()
                    }
                    if arg == "1.2" {
                        days::one::part_two()
                    }
                    if arg == "2.1" {
                        days::two::part_one()
                    }
                    if arg == "2.2" {
                        days::two::part_two()
                    }
                    if arg == "3.1" {
                        days::three::part_one()
                    }
                    if arg == "3.2" {
                        days::three::part_two()
                    }
                    if arg == "4.0" {
                        days::four::part_one()
                    }
                    if arg == "4.1" {
                        days::four::part_two()
                    }
                    if arg == "5.0" {
                        days::five::part_one()
                    }
                    if arg == "5.1" {
                        days::five::part_two()
                    }
                }
            }
        }
        _ => println!("Too many args"),
    }
}
