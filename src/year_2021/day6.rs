fn solution(input: &str, days: i32) -> i64 {
    let mut counter: [i64; 9] = [0; 9];

    for i in input.split(',').map(|str| str.parse::<u8>().unwrap()) {
        counter[i as usize] += 1;
    }

    for _ in 0..days {
        counter.rotate_left(1);
        counter[6] += counter[8];
    }

    counter.iter().fold(0, |a, x| a + *x)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    solution(input, 18)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    solution(input, 256)
}
