#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut vector = input
        .split(',')
        .map(|str| str.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let half_len = vector.len() / 2;

    vector.sort_unstable();

    let avg = if vector.len() & 1 == 0 {
        (vector[half_len] + vector[half_len + 1]) / 2
    } else {
        vector[half_len]
    };

    (vector[0..half_len].iter().fold(0, |acc, &x| acc + avg - x)
        + vector[half_len..].iter().fold(0, |acc, &x| acc + x - avg)) as i64
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    0
}
