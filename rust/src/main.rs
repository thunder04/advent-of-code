use easybench::bench;
mod year_2021;

fn main() {
    let file = std::fs::read_to_string("src/year_2021/resources/day2.txt").unwrap();
    let input = file.as_str();

    println!("Solution: {}", year_2021::day2::part2(&input));

    let stats = bench(|| year_2021::day2::part2(&input));

    println!(
        "Iterations: {} ({} ns/iter - {} ms/iter)",
        stats.iterations,
        stats.ns_per_iter,
        stats.ns_per_iter / 1_000_000.
    );
}
