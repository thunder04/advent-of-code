#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut epsilon: i64 = 0;
    let mut gamma: i64 = 0;

    (0..12).rev().for_each(|i| {
        let mut zeros = 0;
        let mut ones = 0;

        for line in input.lines() {
            if i32::from_str_radix(line, 2).unwrap() & (1 << i) == 0 {
                zeros += 1
            } else {
                ones += 1
            }
        }

        if ones >= zeros {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    });

    epsilon * gamma
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    0
}
