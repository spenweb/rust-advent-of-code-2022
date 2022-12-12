use crate::utils;

pub fn part_one() {
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

pub fn part_two() {
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
