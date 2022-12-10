<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code {year}

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2022/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2022/day/10) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

## Usage

### Scaffold a day

```sh
# example: `cargo scaffold 2022 1`
cargo scaffold <year> <day>

# output:
# Created module "src/bin/2022_01.rs"
# Created empty input file "src/inputs/2022_01.txt"
# Created empty puzzle file "src/inputs/2022_01.txt"
# Created empty example file "src/examples/2022_01.txt"
# ---
# 🎄 Type `cargo solve 2022 1` to run your solution.
```

Individual solutions live in the `./src/bin/` directory as separate binaries.

Every [solution](https://github.com/fspoettel/advent-of-code-rust/blob/main/src/bin/scaffold.rs#L11-L41) has _unit tests_ referencing its _example_ file. Use these unit tests to develop and debug your solution against the example input. For some puzzles, it might be easier to forgo the example file and hardcode inputs into the tests.

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 2022 1`
cargo download <year> <day>

# output:
# Downloading input with > aoc download --year 2022 --day 1 --input-file /tmp/aoc_input_tmp
# Loaded session cookie from "/home/danny/.adventofcode.session".
# Downloading input for day 1, 2022...
# Saving puzzle input to "/tmp/aoc_input_tmp"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/2022_01.txt".
# Downloading puzzle with > aoc read --year 2022 --day 1 --puzzle-file /tmp/aoc_input_tmp
# <omitted>
# Saving puzzle description to "/tmp/aoc_input_tmp"...
# Done!
# Loaded session cookie from "/home/danny/.adventofcode.session".
# ---
# 🎄 Successfully wrote puzzle to "src/puzzles/2022_01.md".
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve 2022 1`
cargo solve <year> <day>

# output:
# ----------
# | Day 1 |
# ----------
# 🎄 Part 1 🎄
# 71506 (elapsed: 72.05µs)
# 🎄 Part 2 🎄
# 209603 (elapsed: 76.01µs)
# Total: 0.15ms
```

`solve` is an alias for `cargo run`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
# ----------
# | Day 1 |
# ----------
# 🎄 Part 1 🎄
# 71506 (elapsed: 72.05µs)
# 🎄 Part 2 🎄
# 209603 (elapsed: 76.01µs)
# <...other days...>
# Total: 0.15ms
```

`all` is an alias for `cargo run --bin all --`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against the example input

```sh
cargo test
```

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

## Optional

### Automatically track ⭐️ progress in the readme

This template includes [a Github action](https://github.com/k2bd/advent-readme-stars) that automatically updates the readme with your advent of code progress.

To enable it, complete the following steps:

#### 1. Create a private leaderboard

Go to the leaderboard page of the year you want to track and click _Private Leaderboard_. If you have not created a leaderboard yet, create one by clicking _Create It_. Your leaderboard should be accessible under `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

-   `AOC_ENABLED`: This variable controls whether the workflow is enabled. Set it to `true` to enable the progress tracker.
-   `AOC_USER_ID`: Go to [this page](https://adventofcode.com/settings) and copy your user id. It's the number behind the `#` symbol in the first name option. Example: `3031`
-   `AOC_YEAR`: the year you want to track. Example: `2021`
-   `AOC_SESSION`: an active session[^1] for the advent of code website. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie.

## Useful crates

-   [itertools](https://crates.io/crates/itertools): Extends iterators with extra methods and adaptors. Frequently useful for aoc puzzles.
-   [regex](https://crates.io/crates/regex): Official regular expressions implementation for Rust.

A curated list of popular crates can be found on [blessed.rs](https://blessed.rs/crates).

Do you have aoc-specific crate recommendations? [Share them!](https://github.com/fspoettel/advent-of-code-rust/edit/main/README.md)

## Common pitfalls

-   **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Footnotes

[^1]: The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
