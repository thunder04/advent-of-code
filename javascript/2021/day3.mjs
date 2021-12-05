import { readFileSync } from 'node:fs'

const data = readFileSync('./resources/day3.txt', { encoding: 'utf8' }).split('\n')
    .map(m => parseInt(m, 2));

export function part1() {
    let epsilon = 0, gamma = 0;

    for (let b = 11; b >= 0; --b) {
        let ones = 0, zeros = 0;

        for (const num of data) {
            if ((1 << b) & num) ++ones;
            else ++zeros;
        }

        if (ones > zeros) gamma |= 1 << b;
        else epsilon |= 1 << b;
    }

    return epsilon * gamma;
}

export function part2() {
    function iterate(arr, bitsLen, invertCheck) {
        for (let b = bitsLen - 1; b >= 0; --b) {
            let ones = 0, zeros = 0;

            for (const num of arr) {
                if ((1 << b) & num) ++ones;
                else ++zeros;
            }

            arr = arr.filter(num => {
                const res = invertCheck ? !!((1 << b) & num) : !((1 << b) & num);
                return ones >= zeros ? !res : res;
            })

            if (arr.length == 1) return arr[0];
        }
    }

    const oxygen_generator_rating = iterate(data, 12, false)
        , co2_scrubber_rating = iterate(data, 12, true);

    return oxygen_generator_rating * co2_scrubber_rating
}

console.log(part1())