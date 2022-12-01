use advent_of_code::{run_solution, ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};

fn main() {
    let total: f64 = (2015..=2022)
        .map(|year| {
            let sub_total: f64 = (1..=25).map(|day| run_solution(year, day)).sum();
            sub_total
        })
        .sum();

    println!(
        "{}Total:{} {}{:.2}ms{}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total, ANSI_RESET
    );
}
