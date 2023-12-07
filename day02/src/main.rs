use std::{ fs::File, collections::HashMap };

use utils::{ read_input, create_output_file };

fn normalize_data(input: Vec<String>) -> Vec<(String, String)> {
    input
        .iter()
        .map(|line| {
            let parts = line
                .split(" ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            (parts[0].clone(), parts[1].clone())
        })
        .collect::<Vec<(String, String)>>()
}

fn part1(input: Vec<String>, mut _output_file: &File) {
    let shapes: HashMap<&str, u32> = HashMap::from_iter(
        vec![("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]
    );

    let mut total_score = 0;
    for (elf, you) in normalize_data(input) {
        let elf = *shapes.get(elf.as_str()).unwrap();
        let you = *shapes.get(you.as_str()).unwrap();
        if elf == you {
            // draw
            total_score += you + 3;
        } else if elf == 1 {
            total_score += you + (if you == 2 { 6 } else { 0 });
        } else if elf == 2 {
            total_score += you + (if you == 3 { 6 } else { 0 });
        } else if elf == 3 {
            total_score += you + (if you == 1 { 6 } else { 0 });
        }
    }

    println!("{total_score}")
}

fn part2(input: Vec<String>, mut _output_file: &File) {
    let shapes: HashMap<&str, u32> = HashMap::from_iter(
        vec![("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]
    );

    let mut total_score = 0;
    for (elf, you) in normalize_data(input) {
        let elf = *shapes.get(elf.as_str()).unwrap();
        let need_to = *shapes.get(you.as_str()).unwrap();
        if need_to == 1 {
            // need to lose
            total_score += 0 + (if elf == 1 { 3 } else if elf == 2 { 1 } else { 2 });
        } else if need_to == 2 {
            // need to draw
            total_score += 3 + elf;
        } else {
            // need to win
            total_score += 6 + (if elf == 1 { 2 } else if elf == 2 { 3 } else { 1 });
        }
    }

    println!("{total_score}")
}
fn main() {
    let day = 2;
    let input = read_input(day);

    let output_file = create_output_file(day);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
}
