#![allow(dead_code, unused_variables, unused_assignments)]

use easybench::bench;
mod year_2021;

fn main() {
    let file = std::fs::read_to_string("src/year_2021/resources/day10.txt").unwrap();
    let input = file.as_str();

    println!("Solution: {}", year_2021::day10::part1(&input));

    if true {
        let stats = bench(|| year_2021::day10::part1(&input));

        println!(
            "Iterations: {}\n{} ns/iter\n{} μs/iter\n{} ms/iter",
            stats.iterations,
            stats.ns_per_iter,
            stats.ns_per_iter / 1_000.,
            stats.ns_per_iter / 1_000_000.
        );
    }
}
