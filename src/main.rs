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

const DEFAULT_INPUT_FOLDER: &str = "inputs";

use clap::{Parser, Subcommand};
use std::{borrow::Borrow, path::{Path, PathBuf}, time::Duration};

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
    All {
        #[arg(short = 'i')]
        input: Option<PathBuf>,
    },
    Day {
        #[arg(value_parser=clap::value_parser!(aoc2023::Day))]
        day: aoc2023::Day,

        #[arg(value_parser=clap::value_parser!(aoc2023::DayPart))]
        part: Option<aoc2023::DayPart>,

        #[arg(short = 'i')]
        input: Option<PathBuf>,
    },
}

fn get_default_input(day: &aoc2023::Day) -> PathBuf {
    Path::new(DEFAULT_INPUT_FOLDER).join(format!("2023-12-{:02}", day.to_usize()))
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

fn run_day_solutions(day: aoc2023::Day, part: Option<aoc2023::DayPart>, input: Option<PathBuf>) -> () {
    let parts: Vec<aoc2023::DayPart> = match part {
        None => vec![aoc2023::DayPart::Part1, aoc2023::DayPart::Part2],
        Some(x) => vec![x],
    };

    let input_path = input.unwrap_or(get_default_input(&day));
    let Ok(data) = std::fs::read_to_string(&input_path) else {
        println!("Failed to read input file {}", input_path.display());
        return;
    };

    let mut total_duration = Duration::ZERO;

    let aoc = aoc2023::AoC2023::new();

    for p in parts  {
        let (answer, duration) = match aoc.run_solver(day.clone(), p.clone(), &data) {
            Ok(x) => x,
            Err(e) => {
                println!("Error occurred running solver: {}", e);
                return;
            },
        };

        println!("Day {} Solution {}: {} ..... {}", day, p, answer, format_duration(&duration));

        total_duration = total_duration + duration;

        
    }
}

fn main() {
    let cli = CommandLineArgs::parse();

    match &cli.command {
        CommandArgs::All { input } => {}
        CommandArgs::Day { day, part, input } => {}
    }

    println!("Hello, world!");
}
