/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use std::io::Write;
use std::path::PathBuf;
use std::{env::temp_dir, io, process::Command};
use std::{fs, process};

fn remove_file(path: &PathBuf) {
    #[allow(unused_must_use)]
    {
        fs::remove_file(path);
    }
}

fn exit_with_status(status: i32, path: &PathBuf) -> ! {
    remove_file(path);
    process::exit(status);
}

fn main() {
    // acquire a temp file path to write aoc-cli output to.
    // aoc-cli expects this file not to be present - delete just in case.
    let mut tmp_file_path = temp_dir();
    tmp_file_path.push("aoc_input_tmp");
    remove_file(&tmp_file_path);

    let args = match advent_of_code::parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    };

    let day_padded = format!("{:02}", args.day);
    let suffix = format!("{}_{}", args.year, day_padded);
    let inputs_path = format!("src/inputs/{}.txt", suffix);
    let puzzles_path = format!("src/puzzles/{}.md", suffix);

    // check if aoc binary exists and is callable.
    if Command::new("aoc").arg("-V").output().is_err() {
        eprintln!("command \"aoc\" not found or not callable. Try running \"cargo install aoc-cli\" to install it.");
        exit_with_status(1, &tmp_file_path);
    }

    let mut download_args = vec![];

    download_args.append(&mut vec![
        "download".into(),
        "--year".into(),
        args.year.to_string(),
        "--day".into(),
        args.day.to_string(),
        "--input-file".into(),
        tmp_file_path.to_string_lossy().to_string(),
    ]);

    println!("Downloading input with > aoc {}", download_args.join(" "));

    match Command::new("aoc").args(download_args).output() {
        Ok(cmd_output) => {
            io::stdout()
                .write_all(&cmd_output.stdout)
                .expect("could not write cmd stdout to pipe.");
            io::stderr()
                .write_all(&cmd_output.stderr)
                .expect("could not write cmd stderr to pipe.");
            if !cmd_output.status.success() {
                exit_with_status(1, &tmp_file_path);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }

    match fs::copy(&tmp_file_path, &inputs_path) {
        Ok(_) => {
            println!("---");
            println!("🎄 Successfully wrote input to \"{}\".", &inputs_path);
        }
        Err(e) => {
            eprintln!("could not copy downloaded input to input file: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }

    remove_file(&tmp_file_path);
    let mut puzzle_args = vec![];

    puzzle_args.append(&mut vec![
        "read".into(),
        "--year".into(),
        args.year.to_string(),
        "--day".into(),
        args.day.to_string(),
        "--puzzle-file".into(),
        tmp_file_path.to_string_lossy().to_string(),
    ]);

    println!("Downloading puzzle with > aoc {}", puzzle_args.join(" "));

    match Command::new("aoc").args(puzzle_args).output() {
        Ok(cmd_output) => {
            io::stdout()
                .write_all(&cmd_output.stdout)
                .expect("could not write cmd stdout to pipe.");
            io::stderr()
                .write_all(&cmd_output.stderr)
                .expect("could not write cmd stderr to pipe.");
            if !cmd_output.status.success() {
                exit_with_status(1, &tmp_file_path);
            }
        }
        Err(e) => {
            eprintln!("failed to spawn aoc-cli: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }

    match fs::copy(&tmp_file_path, &puzzles_path) {
        Ok(_) => {
            println!("---");
            println!("🎄 Successfully wrote puzzle to \"{}\".", &puzzles_path);
        }
        Err(e) => {
            eprintln!("could not copy downloaded puzzle to puzzle file: {}", e);
            exit_with_status(1, &tmp_file_path);
        }
    }
}
