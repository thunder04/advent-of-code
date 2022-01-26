enum SearchMethod {
    LeastCommon,
    MostCommon,
}

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
    let calculate = |method: SearchMethod| {
        let mut vector = unsafe { std::mem::transmute::<&str, &[u8]>(input) }
            .split(|b| *b == 10)
            .collect::<Vec<_>>();
        let mut counter = [0_u16; 12];
        let mut pos: usize = 0;

        while vector.len() > 1 {
            for line in &vector {
                if line[pos] == 49 {
                    counter[pos] += 1;
                }
            }

            let ones = counter[pos];
            let common_bit: u8 = if if matches!(method, SearchMethod::MostCommon) {
                ones << 1 >= vector.len() as u16
            } else {
                ones << 1 < vector.len() as u16
            } {
                49
            } else {
                48
            };

            vector.retain(|line| line[pos] == common_bit);
            pos += 1;
        }

        i64::from_str_radix(unsafe { std::str::from_utf8_unchecked(vector[0]) }, 2).unwrap()
    };

    let oxygen_generator_rating: i64 = calculate(SearchMethod::MostCommon);
    let co2_scrubber_rating: i64 = calculate(SearchMethod::LeastCommon);

    oxygen_generator_rating * co2_scrubber_rating
}
