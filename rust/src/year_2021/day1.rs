use std::fs;

fn load_file() -> Vec<u16> {
    fs::read_to_string("src/year_2021/resources/day1.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse::<u16>().unwrap())
        .collect()
}

pub fn part1() -> u16 {
    let numbers = load_file();
    let mut counter: u16 = 0;

    (0..numbers.len() - 1).for_each(|i| {
        if numbers[i + 1] > numbers[i] {
            counter = counter + 1;
        }
    });

    counter
}

//Answer: 1702
//pub fn part2() {}
