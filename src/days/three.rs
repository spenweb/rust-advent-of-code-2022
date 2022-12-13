use crate::utils;

pub fn part_one() {
    let input_file = "./input/3.txt";
    let mut total: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = utils::read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(datum) = line {
                let (first_half, second_half) = datum.split_at(datum.len() / 2);
                // Find matching character
                let mut same: u8 = 0;
                for x in first_half.as_bytes() {
                    for y in second_half.as_bytes() {
                        if x == y {
                            same = x.clone();
                        }
                    }
                }
                if same == 0 {
                    panic!("Didn't find match")
                }

                // println!("same: {same}");

                total += get_priority(same)
                // let option = String::from(same as char);
                // println!("same: {option}, value: {value}, first: {first_half}, second: {second_half}");
            }
        }
    }

    println!("max: {total}");
}

pub fn part_two() {
    let input_file = "./input/3.txt";
    let mut total: u32 = 0;
    let mut group: [String; 3] = [String::from(""), String::from(""), String::from("")];
    let mut count = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = utils::read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(datum) = line {
                let bucket = count % 3;
                group[bucket] = datum;

                if bucket == 2 {
                    // Find matching character
                    let mut same: u8 = 0;
                    'outer: for x in group[0].as_bytes() {
                        for y in group[1].as_bytes() {
                            for z in group[2].as_bytes() {
                                if x == y && x == z {
                                    same = x.clone();
                                    break 'outer;
                                }
                            }
                        }
                    }

                    if same == 0 {
                        panic!("Didn't find match")
                    }

                    // println!("same: {same}");

                    total += get_priority(same);
                    // let option = String::from(same as char);
                    // println!("same: {option}, value: {value}, first: {first_half}, second: {second_half}");
                }
                count += 1;
            }
        }
    }

    println!("max: {total}");
}

/// Get priority of the letter after adjusting
/// Lowercase item types a through z have priorities 1 through 26.
/// Uppercase item types A through Z have priorities 27 through 52.
fn get_priority(same: u8) -> u32 {
    if same < 91 {
        return same as u32 - 65 + 1 + 26;
    } else {
        return same as u32 - 97 + 1;
    }
}
