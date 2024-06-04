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

use crate::time;

type SolutionFunc<'a> = &'a dyn Fn(&str) -> Result<String, Box<dyn std::error::Error>>;

type DayFuncs<'a> = (SolutionFunc<'a>, SolutionFunc<'a>);

const DAYS: [DayFuncs; 0] = [];

#[derive(Debug)]
pub enum SolveError {
    NotImplemented(Day, Part),
    FileError(std::path::PathBuf),
    SolutionError(Day, Part, Box<dyn std::error::Error>),
}

impl std::error::Error for SolveError {}

impl std::fmt::Display for SolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveError::NotImplemented(day, part) => write!(f, "Problem {}.{} not implemented", day, part),
            SolveError::FileError(path) => {
                write!(f, "Failed to read file {}", path.to_string_lossy())
            }
            SolveError::SolutionError(day, part, err) => {
                write!(f, "Solution for day {} part {} failed: {}", day, part, err)
            }
        }
    }
}

#[derive(Debug)]
pub enum DayError {
    InvalidDay(usize),
    ParseFailure(String)
}

impl std::error::Error for DayError {}

impl std::fmt::Display for DayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailure(s) => write!(f, "Failed to parse '{}' into a day", s),
            Self::InvalidDay(d) => write!(f, "Day value {} must be between 1 and 25 inclusive", d),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Day(usize);

impl Day {
    pub fn new(d: usize) -> Result<Day, DayError> {
        if d <= 0 || d > 25 {
            Err(DayError::InvalidDay(d))
        } else {
            Ok(Day(d as usize))
        }
    }

    pub fn all() -> Vec<Day> {
        (1..=25).map(|x| Day::new(x).unwrap()).collect()
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for Day {
    type Err = DayError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s
            .parse()
            .map_err(|_| DayError::ParseFailure(s.to_string()))?;

        if d <= 0 || d > 25 {
            Err(DayError::ParseFailure(s.to_string()))
        } else {
            Ok(Day(d))
        }
    }
}

impl Into<usize> for Day {
    fn into(self) -> usize {
        self.0 
    }
}

#[derive(Debug)]
pub enum Part {
    A,
    B,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            Part::A => "A",
            Part::B => "B",
        };
        write!(f, "{}", p)
    }
}

pub fn solve(day: Day, part: Part, input: &std::path::Path) -> Result<(String, time::Duration), SolveError> {
    let Some((part_a, part_b)) = DAYS.get(day.0 - 1) else {
        return Err(SolveError::NotImplemented(day, part));
    };

    let Ok(data) = std::fs::read_to_string(input) else {
        return Err(SolveError::FileError(input.to_path_buf()));
    };

    let part_func = match part {
        Part::A => part_a,
        Part::B => part_b,
    };

    let start = std::time::Instant::now();
    let result = part_func(&data).map_err(|e| SolveError::SolutionError(day, part, e))?;
    let duration = start.elapsed().into();

    Ok((result, duration))
}

pub fn day_file(input_folder: &std::path::Path, day: Day) -> std::path::PathBuf {
    input_folder.join(format!("2023-12-{:02}.txt", day.0))
}
