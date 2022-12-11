use std::env;

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
                        one_1()
                    }
                    if arg == "1.2" {
                        one_2()
                    }
                    if arg == "2.1" {
                        two_1()
                    }
                }
            }
            println!("")
        }
        _ => println!("Too many args"),
    }
}

fn one_1() {
    let input_file = "./input/1.txt";
    let mut max = 0;
    let mut current = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = utils::read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(datum) = line {
                if let Ok(number) = datum.parse::<i32>() {
                    current += number;
                }
                if datum == "" {
                    if current > max {
                        max = current;
                    }
                    current = 0;
                }
            }
        }
    }

    println!("max: {max}");
}

fn one_2() {
    let input_file = "./input/1.txt";
    let mut elements: Vec<i32> = vec![];
    let mut current = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = utils::read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(datum) = line {
                if let Ok(number) = datum.parse::<i32>() {
                    current += number;
                }
                if datum == "" {
                    elements.push(current);
                    current = 0;
                }
            }
        }
    }

    elements.sort();
    // TODO: Grab from end of vector instead of having to reverse the whole vector
    elements.reverse();

    let x = &elements[..3];
    let total_top_3: i32 = x.iter().sum();

    println!("total top 3: {total_top_3}");
}

fn two_1() {
    let input_file = "./input/2.txt";
    let mut total: i32 = 0;
    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                // Get opponent's play
                let mut bytes = datum.bytes();
                let opponent_play = bytes.next().expect("first column missing");
                bytes.next();
                let my_play = bytes.next().expect("missing second column");

                let points = get_rps_points(opponent_play, my_play);

                total += points as i32;
            }
        }
    }

    println!("Total: {total}");
}

/**
 * Score rock-paper-scissors round
 */
fn get_rps_points(opponent_play: u8, my_play: u8) -> i8 {
    // Get opponent's integer value after taking offset
    let opponent: i8 = opponent_play as i8 - 65; // ("A" = 0, "B" = 1, "C" = 2)

    // Get my integer value after taking offset
    let mine: i8 = my_play as i8 - 65 - 26 + 3; // ("X" = 0, "Y" = 1, "Z" = 2)

    // Find the difference between the two after adjusting the values so that the opponent's play value to 0
    let diff: i8 = (mine - opponent + 3) % 3;

    // If my updated value subtracted by opponent's is:
    // 0: draw
    // 1: I win
    // 2: opponent wins
    // 0 points for lose, 3 points for draw, 6 points for win
    let match_points = ((diff + 1) % 3) * 3;

    let object_points = mine + 1;

    let total = match_points + object_points;

    // println!("opponent: {opponent}, mine: {mine}, diff: {diff}, total: {total}");

    return total;
}
