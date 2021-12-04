use easybench::bench;
mod year_2021;

fn main() {
    let stats = bench(|| year_2021::day1::part1());

    println!(
        "Iterations: {} ({} ns/iter - {} ms/iter)",
        stats.iterations,
        stats.ns_per_iter,
        stats.ns_per_iter / 1_000_000.
    );
}
