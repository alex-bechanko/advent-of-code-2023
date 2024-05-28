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

type Solution<'a> = &'a dyn Fn(&str) -> Result<String, Box<dyn std::error::Error>>;

type Day<'a> = (Solution<'a>, Solution<'a>);

const DAYS: [Day; 0] = [];

#[derive(Debug)]
pub enum RunError {
    DayNotImplemented(SolutionDay),
    FileError(std::path::PathBuf),
    SolutionError(SolutionDay, SolutionPart, Box<dyn std::error::Error>),
}

impl std::error::Error for RunError {}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::DayNotImplemented(d) => write!(f, "Day {} not implemented", d),
            RunError::FileError(path) => {
                write!(f, "Failed to read file {}", path.to_string_lossy())
            }
            RunError::SolutionError(day, part, err) => {
                write!(f, "Solution for day {} part {} failed: {}", day, part, err)
            }
        }
    }
}

#[derive(Debug)]
pub enum SolutionDayError {
    InvalidDay(usize),
    ParseFailure(String)
}

impl std::error::Error for SolutionDayError {}

impl std::fmt::Display for SolutionDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailure(s) => write!(f, "Failed to parse '{}' into a day", s),
            Self::InvalidDay(d) => write!(f, "Day value {} must be between 1 and 25 inclusive", d),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SolutionDay(usize);

impl SolutionDay {
    pub fn new(d: usize) -> Result<SolutionDay, SolutionDayError> {
        if d <= 0 || d > 25 {
            Err(SolutionDayError::InvalidDay(d))
        } else {
            Ok(SolutionDay(d as usize))
        }
    }
}

impl std::fmt::Display for SolutionDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for SolutionDay {
    type Err = SolutionDayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s
            .parse()
            .map_err(|_| SolutionDayError::ParseFailure(s.to_string()))?;

        if d <= 0 || d > 25 {
            Err(SolutionDayError::ParseFailure(s.to_string()))
        } else {
            Ok(SolutionDay(d))
        }
    }
}

#[derive(Debug)]
pub enum SolutionPart {
    PartA,
    PartB,
}

#[derive(Debug)]
pub struct SolutionPartParseFailure;

impl std::fmt::Display for SolutionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            SolutionPart::PartA => "A",
            SolutionPart::PartB => "B",
        };
        write!(f, "{}", p)
    }
}

pub struct DayResult {
    pub part_a_result: String,
    pub part_a_duration: std::time::Duration,

    pub part_b_result: String,
    pub part_b_duration: std::time::Duration,
}

pub fn solve_day(day: SolutionDay, input: &std::path::Path) -> Result<DayResult, RunError> {
    let Some((part_a, part_b)) = DAYS.get(day.0 - 1) else {
        return Err(RunError::DayNotImplemented(day.clone()));
    };

    let Ok(data) = std::fs::read_to_string(input) else {
        return Err(RunError::FileError(input.to_path_buf()));
    };

    let part_a_start = std::time::Instant::now();
    let part_a_result =
        part_a(&data).map_err(|e| RunError::SolutionError(day.clone(), SolutionPart::PartA, e))?;
    let part_a_duration = part_a_start.elapsed();

    let part_b_start = std::time::Instant::now();
    let part_b_result =
        part_b(&data).map_err(|e| RunError::SolutionError(day.clone(), SolutionPart::PartB, e))?;
    let part_b_duration = part_b_start.elapsed();

    Ok(DayResult {
        part_a_result,
        part_a_duration,
        part_b_result,
        part_b_duration,
    })
}

pub fn day_file(input_folder: &std::path::Path, day: SolutionDay) -> std::path::PathBuf {
    input_folder.join(format!("2023-12-{:02}.txt", day.0))
}
