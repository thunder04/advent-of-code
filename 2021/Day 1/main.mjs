import { readFileSync } from 'node:fs'

const data = readFileSync('./input.txt', { encoding: 'utf8' }).split('\n')
    .map(Number);

function part1() {
    let counter = 0;

    for (let i = 0, l = data.length - 1; i < l; ++i) {
        if (data[i + 1] > data[i]) ++counter;
    }

    return counter;
}

function part2() {
    let counter = 0;

    for (let i = 0, l = data.length - 2; i < l; ++i) {
        if (
            (data[i + 1] + data[i + 2] + data[i + 3]) > (data[i] + data[i + 1] + data[i + 2])
        ) ++counter;
    }

    return counter
}