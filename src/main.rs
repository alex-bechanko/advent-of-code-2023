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

use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name="aoc")]
#[command(author="Alex Bechanko")]
#[command(about="Compute and time Advent of Code 2023 solutions")]
struct CommandLineArgs {
    #[command(subcommand)]
    command: CommandArgs,
}

#[derive(Debug Subcommand)]
enum CommandArgs {
    RunAll(RunAllArgs),
    RunDay(RunDayArgs),
}

#[derive(Debug, Args)]
struct RunDayArgs {
    #[arg(value_parser=clap::value_parser!(aoc2023::Day))]
    day: aoc2023::Day,

    #[arg(value_parser=clap::value_parser!(aoc2023::Part))]
    part: Option<aoc2023::Part>,

    #[arg(short='i')]
    input: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct RunAllArgs {
    #[arg(short='i')]
    input: Option<PathBuf>,
}


fn main() {

    let cli = CommandLineArgs::parse();

    match 



    println!("Hello, world!");

}
