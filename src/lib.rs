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

#[derive(Debug, Clone)]
pub enum Day {
    Day(usize),
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
        s.parse::<usize>().map_err(|_| ParseDayError).map(|x| Self::Day(x))
    }
}


#[derive(Debug, Clone)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub struct ParsePartError;
impl std::error::Error for ParsePartError {}

impl std::fmt::Display for ParsePartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse part from a string")
    }
}

impl std::str::FromStr for Part {
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
