import { readFileSync } from 'node:fs'

const data = readFileSync('./2021/resources/day2.txt', { encoding: 'utf8' }).split('\n')

export function part1() {
    let horizontal_position = 0, depth = 0;

    for (const order of data) {
        let [command, amount] = order.split(' ')
        amount = +amount

        switch (command) {
            case 'forward':
                horizontal_position += amount;
                break;
            case 'down':
                depth += amount;
                break;
            case 'up':
                depth -= amount;
                break;
        }
    }

    return horizontal_position * depth;
}

export function part2() {
    let horizontal_position = 0, aim = 0, depth = 0;

    for (const order of data) {
        let [command, amount] = order.split(' ')
        amount = +amount

        switch (command) {
            case 'forward':
                horizontal_position += amount;
                depth += aim * amount;
                break;
            case 'down':
                aim += amount;
                break;
            case 'up':
                aim -= amount;
                break;
        }
    }

    return horizontal_position * depth;
}