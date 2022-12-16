use crate::utils;

pub fn part_one() {
    let input_file = "./input/4.txt";
    let mut total: u32 = 0;
    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                // Split assignment pair
                let assignments: Vec<&str> = datum.split(',').collect();
                let a1 = get_min_max_from_assignment(assignments[0]);
                let a2 = get_min_max_from_assignment(assignments[1]);

                if a1.0 == a2.0 && a1.1 == a2.1 {
                    // println!("same   - a1: {:?}, a2: {:?}, datum: {datum}", a1, a2);
                    total += 1;
                    continue;
                }

                // Check if the first assignment contains all of the second
                if a1.0 <= a2.0 && a1.1 >= a2.1 {
                    // println!("first  - a1: {:?}, a2: {:?}, datum: {datum}", a1, a2);
                    total += 1;
                    continue;
                }

                // Check if the second assignment contains all of the first
                if a2.0 <= a1.0 && a2.1 >= a1.1 {
                    // println!("second - a1: {:?}, a2: {:?}, datum: {datum}", a1, a2);
                    total += 1;
                    continue;
                }
            }
        }
    }

    println!("total: {total}");
}

fn get_min_max_from_assignment(assignments: &str) -> (u32, u32) {
    let sections: Vec<&str> = assignments.split('-').collect();
    if sections.len() != 2 {
        panic!("expected assignment to have two sections indicated for {assignments}")
    }

    let first: u32 = sections[0]
        .parse::<u32>()
        .expect(format!("section must be a number; got {}", sections[0]).as_str());
    let second: u32 = sections[1]
        .parse::<u32>()
        .expect(format!("section must be a number; got {}", sections[1]).as_str());

    // Consider how sections could possibly be listed with higher number first
    if first > second {
        return (second, first);
    } else {
        return (first, second);
    }
}
