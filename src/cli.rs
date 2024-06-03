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

use crate::aoc;
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "aoc")]
#[command(author = "Alex Bechanko")]
#[command(about = "Compute and time Advent of Code 2023 solutions")]
pub struct CommandLineArgs {
    #[command(subcommand)]
    pub command: CommandArgs,
}

#[derive(Debug, Subcommand)]
pub enum CommandArgs {
    #[command(about = "Run and time every day's solutions")]
    All(AllArgs),

    #[command(about = "Run and time a specific day's solutions")]
    Day(DayArgs),
}

#[derive(Debug, Args)]
pub struct AllArgs {
    #[arg(long)]
    pub input_folder: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DayArgs {
    #[arg(value_parser=clap::value_parser!(aoc::Day))]
    pub day: aoc::Day,

    #[arg(long)]
    pub input_file: Option<PathBuf>,
}
