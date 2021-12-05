#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut counter: i64 = 0;

    (0..numbers.len() - 1).for_each(|i| {
        if numbers[i + 1] > numbers[i] {
            counter = counter + 1;
        }
    });

    counter
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut counter: i64 = 0;

    (0..numbers.len() - 3).for_each(|i| {
        if numbers[i + 1] + numbers[i + 2] + numbers[i + 3]
            > numbers[i] + numbers[i + 1] + numbers[i + 2]
        {
            counter = counter + 1;
        }
    });

    counter
}
