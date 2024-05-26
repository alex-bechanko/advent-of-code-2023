/*
* Advent of Code 2023 Solutions Copyright (C) 2024 Alex Bechanko
* <alexbechanko@gmail.com>
*
* This program is free software: you can redistribute it and/or modify it under
* the terms of the GNU General Public License as published by the Free Software
* Foundation, either version 3 of the License, or (at your option) any later
* version.
*
* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
* FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
* details.
*
* You should have received a copy of the GNU General Public License along with
* this program.  If not, see <https://www.gnu.org/licenses/>.
*/
mod aoc;

use clap::{Args, Parser, Subcommand};
use std::{path::PathBuf, str::FromStr, time::Duration};

const DEFAULT_FOLDER: &str = "inputs";

#[derive(Debug, Parser)]
#[command(name = "aoc")]
#[command(author = "Alex Bechanko")]
#[command(about = "Compute and time Advent of Code 2023 solutions")]
struct CommandLineArgs {
    #[command(subcommand)]
    command: CommandArgs,
}

#[derive(Debug, Subcommand)]
enum CommandArgs {
    #[command(about = "Run and time every day's solutions")]
    All(AllArgs),

    #[command(about = "Run and time a specific day's solutions")]
    Day(DayArgs),
}

#[derive(Debug, Args)]
struct AllArgs {
    #[arg(long)]
    input_folder: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct DayArgs {
    #[arg(value_parser=clap::value_parser!(aoc::SolutionDay))]
    day: aoc::SolutionDay,

    #[arg(long)]
    input_file: Option<PathBuf>,
}

fn format_duration(d: &Duration) -> String {
    let secs = u128::from(d.as_secs());
    let millis = d.as_millis();
    let micros = d.as_millis();
    let nanos = d.as_nanos();

    let (major, minor, unit) = if secs > 0 {
        (secs, millis, "s")
    } else if millis > 0 {
        (millis, micros, "ms")
    } else if micros > 0 {
        (micros, nanos, "μs")
    } else {
        (nanos, nanos * 1000, "ns")
    };

    let duration = major as f64 + (minor - major * 1000) as f64 / 1000.0;
    format!("{:2} {}", duration, unit)
}

fn run_day(args: DayArgs) -> () {
    let default_folder = PathBuf::from_str(DEFAULT_FOLDER).unwrap();

    let input_file = args
        .input_file
        .unwrap_or(aoc::day_file(&default_folder, args.day));

    match aoc::solve_day(args.day, &input_file) {
        Ok(ans) => {
            println!(
                "Day {:02} Part A Solution {} .... {}",
                args.day,
                ans.part_a_result,
                format_duration(&ans.part_a_duration)
            );
            println!(
                "Day {:02} Part B Solution {} .... {}",
                args.day,
                ans.part_b_result,
                format_duration(&ans.part_b_duration)
            );
        }
        Err(e) => {
            println!("Day {:02} Error: {}", args.day, e);
        }
    }
}

fn run_all(args: AllArgs) -> () {
    let default_folder = PathBuf::from_str(DEFAULT_FOLDER).unwrap();
    let input_folder = args.input_folder.unwrap_or(default_folder);

    let mut total_duration = std::time::Duration::ZERO;
    for d in 1..26 {
        let day = aoc::SolutionDay::new(d).unwrap();
        let input = aoc::day_file(&input_folder, day);

        match aoc::solve_day(day, &input) {
            Err(e) => println!("Day {} Error: {}", day, e),
            Ok(res) => {
                total_duration += res.part_a_duration + res.part_b_duration;
                println!(
                    "Day {} Part A: {} .... {}",
                    day,
                    res.part_a_result,
                    format_duration(&res.part_a_duration)
                );
                println!(
                    "       Part B: {} .... {}",
                    res.part_b_result,
                    format_duration(&res.part_b_duration)
                );
                
            }
        }
    }
    println!("Total Duration .... {}", format_duration(&total_duration));
}

fn main() {
    let _cli = CommandLineArgs::parse();
    match _cli.command {
        CommandArgs::Day(args) => run_day(args),
        CommandArgs::All(args) => run_all(args),
    }
}
