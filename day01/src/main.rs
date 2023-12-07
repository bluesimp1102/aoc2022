use std::fs::File;

use utils::{ read_input, create_output_file };

fn normalize_data(input: Vec<String>) -> Vec<Vec<u64>> {
    input
        .join("\n")
        .split("\n\n")
        .map(|elf_inv|
            elf_inv
                .split("\n")
                .map(|food| food.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        )
        .collect()
}

fn part1(input: Vec<String>, mut _output_file: &File) {
    let mut max = u64::MIN;

    for (_, elf_inv) in normalize_data(input).iter().enumerate() {
        max = max.max(elf_inv.iter().sum::<u64>());
    }

    println!("max {}", max)
}

fn part2(input: Vec<String>, mut _output_file: &File) {
    let mut stack: Vec<u64> = Vec::new();
    for (_, elf_inv) in normalize_data(input).iter().enumerate() {
        let elf_total_kcal = elf_inv.iter().sum::<u64>();
        if stack.len() < 3 {
            stack.push(elf_total_kcal);
        } else {
            stack.push(elf_total_kcal);
            stack.sort();
            stack.remove(0);
        }
    }

    println!("{:?} - total: {}", stack, stack.iter().sum::<u64>())
}
fn main() {
    let day = 1;
    let input = read_input(day);

    let output_file = create_output_file(day);

    part1(input.clone(), &output_file);
    part2(input.clone(), &output_file);
}
