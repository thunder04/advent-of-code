import { readFileSync } from 'node:fs'

const data = readFileSync('./2021/resources/day4.txt', { encoding: 'utf8' }).split('\n')
    , numbers = data[0].split(',').map(Number).sort((a, b) => a > b)
    , cards = [];

for (let i = 1, l = data.length; i < l; ++i) {
    if (data[i] != '') {
        cards.push(
            [data[i], data[++i], data[++i], data[++i], data[++i]]
                .map(line => line.trim().split(/\s+/).map(Number))
        );
    }
}

function checkWon(data) {
    for (let y = 0; y < 5; ++y) if (data[y][0] == -1 && data[y][1] == -1 && data[y][2] == -1 && data[y][3] == -1 && data[y][4] == -1) return true
    for (let x = 0; x < 5; ++x) if (data[0][x] == -1 && data[1][x] == -1 && data[2][x] == -1 && data[3][x] == -1 && data[4][x] == -1) return true
    return false
}

function calculate(breakOnWin) {
    let wonCard, wonNumber, sum = 0

    loop:
    for (const number of numbers) {
        for (let i = 0, l = cards.length; i < l; ++i) {
            const card = cards[i];

            for (let y = 0; y < 5; ++y) {
                for (let x = 0; x < 5; ++x) {
                    if (card[y][x] == number) card[y][x] = -1;
                }
            }

            if (checkWon(card)) {
                wonNumber = number;
                wonCard = card;

                if (breakOnWin) break loop;
                cards.splice(i, 1);
                --l;
            }
        }
    }

    for (let y = 0; y < 5; ++y) {
        for (let x = 0; x < 5; ++x) {
            if (wonCard[y][x] != -1) sum += wonCard[y][x]
        }
    }

    return sum * wonNumber
}

export function part1() { return calculate(true) }
export function part2() { return calculate(false) }