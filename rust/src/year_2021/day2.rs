#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;

    for line in input.lines() {
        match &line[0..1] {
            "f" => horizontal_position += &line[8..].parse::<i64>().unwrap(),
            "d" => depth += &line[5..].parse::<i64>().unwrap(),
            "u" => depth -= &line[3..].parse::<i64>().unwrap(),
            _ => (),
        }
    }

    horizontal_position * depth
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut horizontal_position: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;

    for line in input.lines() {
        match &line[0..1] {
            "f" => {
                let x = &line[8..].parse::<i64>().unwrap();
                horizontal_position += x;
                depth += aim * x;
            }

            "d" => aim += &line[5..].parse::<i64>().unwrap(),
            "u" => aim -= &line[3..].parse::<i64>().unwrap(),
            _ => (),
        }
    }

    horizontal_position * depth
}
