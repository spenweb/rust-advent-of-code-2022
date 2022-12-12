use crate::utils;

pub fn part_one() {
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

pub fn part_two() {
    let input_file = "./input/2.txt";
    let mut total = 0;
    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                // Get opponent's play
                let mut bytes = datum.bytes();
                let opponent_play = bytes.next().expect("first column missing");
                bytes.next();
                let my_play = bytes.next().expect("missing second column");

                let points = get_rps_points_strategy_two(opponent_play, my_play);

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

/**
 * Score rock-paper-scissors round using the full "encrypted strategy"
 */
fn get_rps_points_strategy_two(opponent_play: u8, my_play: u8) -> i8 {
    // Get opponent's integer value after taking offset
    let opponent: i8 = opponent_play as i8 - 65; // ("A" = 0, "B" = 1, "C" = 2)

    // "X" is lose, "Y" is draw, "Z" is win
    let outcome: i8 = my_play as i8 - 65 - 26 + 3; // ("X" = 0, "Y" = 1, "Z" = 2)
    let mine: i8;
    if outcome == 0 {
      mine = (opponent + 2) % 3;
    } else if outcome == 1 {
      mine = opponent;
    } else {
      mine = (opponent + 1) % 3;
    }

    let match_points = outcome * 3;

    let object_points = mine + 1;

    let total = match_points + object_points;

    // println!("opponent: {opponent}, mine: {mine}, diff: {diff}, total: {total}");

    return total;
}
