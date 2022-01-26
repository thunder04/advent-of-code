#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut stack: Vec<char> = vec![];
    let mut score: i64 = 0;

    for line in input.lines() {
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    let top = stack.pop().unwrap();

                    if (top == '(' && char != ')')
                        || (top == '{' && char != '}')
                        || (top == '[' && char != ']')
                        || (top == '<' && char != '>')
                    {
                        match char {
                            ')' => score += 3,
                            ']' => score += 57,
                            '}' => score += 1197,
                            '>' => score += 25137,
                            _ => {}
                        };

                        break;
                    }
                }
                _ => {}
            }
        }

        stack.clear()
    }

    score
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut stack: Vec<char> = vec![];

    let iter = input.lines().filter(|&line| {
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => match stack.last().unwrap() {
                    '(' if char != ')' => return false,
                    '{' if char != '}' => return false,
                    '[' if char != ']' => return false,
                    '<' if char != '>' => return false,
                    _ => {
                        stack.pop();
                    }
                },

                _ => {}
            }
        }

        stack.clear();
        true
    });

    let mut stack: Vec<char> = vec![];
    let mut scores: Vec<i64> = vec![];

    for line in iter {
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push(char),
                ')' | ']' | '}' | '>' => {
                    stack.pop();
                }
                _ => {}
            }
        }

        let mut score: i64 = 0;
        for char in stack.drain(..).rev() {
            score *= 5;
            match char {
                '(' => score += 1,
                '[' => score += 2,
                '{' => score += 3,
                '<' => score += 4,
                _ => {}
            };
        }

        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}
