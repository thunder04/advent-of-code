#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let vector = &input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut epsilon: i64 = 0;
    let mut gamma: i64 = 0;

    for i in 0..12 {
        let zeros = vector
            .iter()
            .filter(|bytes| bytes[i] == 48 /* 0 in ASCII */)
            .count();

        if vector.len() > zeros << 1 {
            gamma |= 1 << 11 >> i
        } else {
            epsilon |= 1 << 11 >> i
        }
    }

    epsilon * gamma
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    /*let mut epsilon: i64 = 0;
    let mut gamma: i64 = 0;

    for i in 0..12 {}*/
    0
}
