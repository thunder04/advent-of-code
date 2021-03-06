///
/// TODO: Try removing the numbers instead of marking them with -1
///
use itertools::Itertools;

fn check_if_won(card: [[i8; 5]; 5]) -> bool {
    for i in 0..5 {
        if card[i][0] == -1
            && card[i][1] == -1
            && card[i][2] == -1
            && card[i][3] == -1
            && card[i][4] == -1
        {
            return true;
        }
    }

    for j in 0..5 {
        if card[0][j] == -1
            && card[1][j] == -1
            && card[2][j] == -1
            && card[3][j] == -1
            && card[4][j] == -1
        {
            return true;
        }
    }

    false
}

//TODO: Avoid allocating a vector
//TODO: Serialize them into a different format
fn parse_input(input: &str) -> (Vec<i8>, Vec<[[i8; 5]; 5]>) {
    let mut iter = input.lines();
    let iter_ref = iter.by_ref();

    let numbers = iter_ref
        .take(1)
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i8>().unwrap())
        .collect::<Vec<_>>();

    let cards = iter_ref
        .filter(|str| *str != "")
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            let mut card = [[0_i8; 5]; 5];
            let mut iter = chunk.map(|str| {
                str.split_ascii_whitespace()
                    .map(|x| x.parse::<i8>().unwrap())
            });

            for i in 0..5 {
                let mut row = iter.next().unwrap();
                for j in 0..5 {
                    card[i][j] = row.next().unwrap();
                }
            }

            card
        })
        .collect::<Vec<_>>();

    (numbers, cards)
}

fn calculate_winner(card: [[i8; 5]; 5], number: i64) -> i64 {
    let mut sum: i64 = 0;

    for i in 0..5 {
        for j in 0..5 {
            if card[i][j] != -1 {
                sum += card[i][j] as i64;
            }
        }
    }

    return sum * number;
}

fn mark_card(card: &mut [[i8; 5]; 5], number: i8) {
    for i in 0..5 {
        for j in 0..5 {
            if number == card[i][j] {
                card[i][j] = -1;
            }
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let (numbers, ref mut cards) = parse_input(input);

    for num in numbers {
        for card in cards.iter_mut() {
            mark_card(card, num);

            if check_if_won(*card) {
                return calculate_winner(*card, num as i64);
            }
        }
    }

    panic!("WTF Happened")
}

#[allow(dead_code)]
pub fn part2(_: &str) -> i64 {
    0
}
