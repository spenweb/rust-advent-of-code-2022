use crate::utils;

pub fn part_one() {
    let input_file = "./input/4.txt";
    let mut total: u32 = 0;
    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                // Split assignment pair
                let assignments: Vec<&str> = datum.split(',').collect();
                let first = get_min_max_from_assignment(assignments[0]);
                let second = get_min_max_from_assignment(assignments[1]);

                if first.min == second.min && first.max == second.max {
                    total += 1;
                    continue;
                }

                // Check if the first assignment contains all of the second
                if first.min <= second.min && first.max >= second.max {
                    total += 1;
                    continue;
                }

                // Check if the second assignment contains all of the first
                if second.min <= first.min && second.max >= first.max {
                    total += 1;
                    continue;
                }
            }
        }
    }

    println!("total: {total}");
}

pub fn part_two() {
    let input_file = "./input/4.txt";
    let mut total: u32 = 0;
    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                // Split assignment pair
                let assignments: Vec<&str> = datum.split(',').collect();
                let first = get_min_max_from_assignment(assignments[0]);
                let second = get_min_max_from_assignment(assignments[1]);

                // Check if the first assignment min is between second's min and max (inclusively)
                if first.min >= second.min && first.min <= second.max {
                    total += 1;
                    continue;
                }

                // Check if the first assignment max is between second's min and max (inclusively)
                if first.max >= second.min && first.max <= second.max {
                    total += 1;
                    continue;
                }

                // Check if the second assignment min is between first's min and max (inclusively)
                if second.min >= first.min && second.min <= first.max {
                    total += 1;
                    continue;
                }

                // Check if the second assignment max is between first's min and max (inclusively)
                if second.max >= first.min && second.max <= first.max {
                    total += 1;
                    continue;
                }
            }
        }
    }

    println!("total: {total}");
}

fn get_min_max_from_assignment(assignments: &str) -> Assignment {
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
        return Assignment {
            min: second,
            max: first,
        };
    } else {
        return Assignment {
            min: first,
            max: second,
        };
    }
}

struct Assignment {
    min: u32,
    max: u32,
}
