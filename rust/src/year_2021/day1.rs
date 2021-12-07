fn solution(input: &str, windows_size: usize, last_index: usize) -> i64 {
    let mut counter: i64 = 0;

    for chunk in input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .windows(windows_size)
    {
        if chunk[last_index] > chunk[0] {
            counter = counter + 1;
        }
    }

    counter
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    solution(input, 2, 1)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    solution(input, 4, 3)
}
