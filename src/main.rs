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
use std::{collections::HashMap, path::{Path, PathBuf}};

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

        #[arg(value_parser=clap::value_parser!(aoc2023::Part))]
        part: Option<aoc2023::Part>,

        #[arg(short = 'i')]
        input: Option<PathBuf>,
    },
}

fn get_default_input(day: aoc2023::Day) -> PathBuf {
    Path::new(DEFAULT_INPUT_FOLDER).join(format!("2023-12-{:02}", day.0))
}

fn run_day_solutions(day: aoc2023::Day, part: Option<aoc2023::Part>, input: Option<PathBuf>) -> () {
    let parts: Vec<aoc2023::Part> = match part {
        None => vec![aoc2023::Part::Part1, aoc2023::Part::Part2],
        Some(x) => vec![x],
    };

    let input = input.unwrap_or(get_default_input(day));

}

fn main() {
    let cli = CommandLineArgs::parse();

    match &cli.command {
        CommandArgs::All { input } => {}
        CommandArgs::Day { day, part, input } => {}
    }

    println!("Hello, world!");
}
