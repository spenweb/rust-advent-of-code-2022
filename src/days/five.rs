use crate::utils;

pub fn part_one() {
    let input_file = "./input/5.txt";
    let mut stack_lines: Vec<String> = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    let mut stacks: Vec<Vec<char>> = vec![];

    let mut is_loading_stack = true;

    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                if datum.is_empty() {
                    if is_loading_stack {
                        stacks = parse_stack_lines(&stack_lines);
                        println!("Rotated matrix:");
                        print_matrix(&stacks);
                    }
                    is_loading_stack = false;
                } else {
                    if is_loading_stack {
                        stack_lines.push(datum);
                    } else {
                        instructions.push(parse_instruction_line(datum));
                    }
                }
            }
        }
    }

    // Move items in stacks
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let item = stacks[instruction.from as usize - 1].pop().unwrap();
            stacks[instruction.to as usize - 1].push(item);
        }
    }

    println!("\nTop crates:");
    for x in 0..stacks.len() {
        let top_crate = stacks[x].last().unwrap();
        print!("{top_crate}");
    }
    println!();
}

pub fn part_two() {
    let input_file = "./input/5.txt";
    let mut stack_lines: Vec<String> = vec![];
    let mut instructions: Vec<Instruction> = vec![];
    let mut stacks: Vec<Vec<char>> = vec![];

    let mut is_loading_stack = true;

    if let Ok(lines) = utils::read_lines(input_file) {
        for line in lines {
            if let Ok(datum) = line {
                if datum.is_empty() {
                    if is_loading_stack {
                        stacks = parse_stack_lines(&stack_lines);
                        println!("Rotated matrix:");
                        print_matrix(&stacks);
                    }
                    is_loading_stack = false;
                } else {
                    if is_loading_stack {
                        stack_lines.push(datum);
                    } else {
                        instructions.push(parse_instruction_line(datum));
                    }
                }
            }
        }
    }

    // Move items in stacks
    for instruction in instructions {
        let len = stacks[instruction.from as usize - 1].len();
        let mut items: Vec<char> = stacks[instruction.from as usize - 1]
            .splice((len - instruction.amount as usize).., [])
            .collect();
        stacks[instruction.to as usize - 1].append(&mut items);
    }

    println!("\nTop crates:");
    for x in 0..stacks.len() {
        let top_crate = stacks[x].last().unwrap();
        print!("{top_crate}");
    }
    println!();
}

fn parse_stack_lines(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut cleaned_matrix: Vec<Vec<char>> = vec![];
    for line in lines {
        let mut cleaned: Vec<char> = vec![];
        let mut count = 0;
        for x in line.chars() {
            if (count % 4) == 1 {
                cleaned.push(x);
            }
            count += 1;
        }
        cleaned_matrix.push(cleaned);
    }

    // Rotate matrix clockwise 90 degrees
    let mut rotated_matrix: Vec<Vec<char>> = vec![];
    // Set up the rotated matrix
    for _ in 0..cleaned_matrix[0].len() {
        rotated_matrix.push(vec![' '; cleaned_matrix.len()]);
    }
    for y in 1..cleaned_matrix.len() {
        for x in 0..cleaned_matrix[y].len() {
            let c = cleaned_matrix[cleaned_matrix.len() - y - 1][x];
            rotated_matrix[x][y - 1] = c;
        }
    }

    // Remove trailing space char
    for y in 0..rotated_matrix.len() {
        while rotated_matrix[y].ends_with(&[' ']) {
            rotated_matrix[y].pop();
        }
    }
    return rotated_matrix;
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let c = matrix[y][x];
            print!("{c}");
        }
        print!("\n");
    }
}

struct Instruction {
    amount: i32,
    from: i32,
    to: i32,
}

fn parse_instruction_line(line: String) -> Instruction {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() != 6 {
        panic!("Got instruction line without 6 parts: {line}");
    }

    return Instruction {
        amount: parts[1].parse().unwrap(),
        from: parts[3].parse().unwrap(),
        to: parts[5].parse().unwrap(),
    };
}
