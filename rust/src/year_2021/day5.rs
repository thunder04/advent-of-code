#[allow(dead_code)]
pub fn part1(mut input: &str) -> i64 {
    input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    let lines = input.lines().map(|line| {
        let mut chunk = [[0_i16; 2]; 2];
        for (i, xy) in line
            .split(" -> ")
            .map(|coords| {
                let mut points = [0_i16; 2];
                for (i, point) in coords
                    .split(',')
                    .map(|n| n.parse::<i16>().unwrap())
                    .enumerate()
                {
                    points[i] = point;
                }

                points
            })
            .enumerate()
        {
            chunk[i] = xy;
        }

        chunk
    });

    lines.count();

    0
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    0
}
