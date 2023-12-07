## Please find the problem description here

[Advent of Code 2022 - Day01](https://adventofcode.com/2022/day/1)

## Solution

After reading the input line by line, for this particular problem, I rejoined the input lines, then split but the character `\n\n` to split the inventory for each Elf, then for each Elf we will parse the food kCal numbers into u64, we assume the input is valid number strings so we can just unwrap the result from parsing.

For part 1, we simply go through each Elf food inventory, sum it up and compare to the current maximum value of kCal (initially u64::MIN, which is the minimum of unsigned 64-bit integer in Rust).

For part 2, we keep track only top three numbers as we iterating through each Elf inventory, for each Elf, we would push the total kCal in its inventory to the stack vector, sort it, then pop out the lowest one.
