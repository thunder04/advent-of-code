import { readFileSync } from 'node:fs'

// [[x1, y1], [x2, y2]][500]
const data = readFileSync('./2021/resources/day5.txt', { encoding: 'utf8' }).split('\n')
    .map(line => line
        .split(' -> ')
        .map(coordinates => coordinates
            .split(',').map(Number)
        )
    )

export function part1() {
    console.log(data)
}

export function part2() {

}

console.log(`Answer: ${part1()}`)