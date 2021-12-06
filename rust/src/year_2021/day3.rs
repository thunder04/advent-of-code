#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut epsilon: i64 = 0;
    let mut gamma: i64 = 0;

    for i in 0..12 {
        let mut zeros = 0;
        let mut ones = 0;

        for line in input.lines() {
            match line.as_bytes()[i] {
                48 => zeros = zeros + 1,
                49 => ones = ones + 1,
                _ => {}
            }
        }

        if ones > zeros {
            gamma = gamma | 1 << (11 - i)
        } else {
            epsilon = epsilon | 1 << (11 - i)
        }
    }

    epsilon * gamma
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    0
}
