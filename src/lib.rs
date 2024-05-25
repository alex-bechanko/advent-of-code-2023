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

use std::{collections::HashMap, error::Error, fmt::Display, time::{Duration, Instant}};
type Solver<'a> = &'a dyn Fn(&str) -> Result<String, Box<dyn Error>>;

pub struct AoC2023<'a> {
    solutions: HashMap<(Day, DayPart), Solver<'a>>,
}

impl<'a> AoC2023<'a> {
    pub fn new() -> Self {
        let solutions = HashMap::new();

        Self { solutions }
    }

    pub fn run_solver(&self, d: Day, p: DayPart, input: &str) -> Result<(String, Duration), Aoc2023Error> {
        let solver = self.solutions.get(&(d, p)).ok_or(Aoc2023Error::NoSolver)?;

        let timer = Instant::now();
        let ans = solver(input).map_err(|e| Aoc2023Error::SolverError(e))?;
        let elapsed = timer.elapsed();

        Ok((ans, elapsed))
    }
}

#[derive(Debug)]
pub enum Aoc2023Error {
    NoSolver,
    SolverError(Box<dyn Error>),
}
impl std::error::Error for Aoc2023Error {}

impl std::fmt::Display for Aoc2023Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Aoc2023Error::NoSolver => write!(f, "No solver found"),
            Aoc2023Error::SolverError(e) => write!(f, "Error occurred running solver: {}", e),

        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Day {
    num: usize,
}

impl Day {
    pub fn new(value: usize) -> Option<Self> {
        if value <= 0 || value > 25 {
            None
        } else {
            Some(Self { num: value })
        }
    }

    pub fn to_usize(&self) -> usize {
        self.num
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.num.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct ParseDayError;
impl std::error::Error for ParseDayError {}

impl std::fmt::Display for ParseDayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse day from a string")
    }
}

impl std::str::FromStr for Day {
    type Err = ParseDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.parse::<usize>().map_err(|_| ParseDayError)?;

        Day::new(x).ok_or(ParseDayError)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DayPart {
    Part1,
    Part2,
}

impl Display for DayPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Part1 => write!(f, "1"),
            Self::Part2 => write!(f, "2"),
        }
    }
}

#[derive(Debug)]
pub struct ParsePartError;
impl std::error::Error for ParsePartError {}

impl std::fmt::Display for ParsePartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse part from a string")
    }
}

impl std::str::FromStr for DayPart {
    type Err = ParsePartError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "1" || s == "a" {
            Ok(Self::Part1)
        } else if s == "2" || s == "b" {
            Ok(Self::Part2)
        } else {
            Err(ParsePartError)
        }
    }
}
