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

use std::fmt::Display;

pub struct Duration(std::time::Duration);

impl Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = self.0;
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

        let dur = major as f64 + (minor - major * 1000) as f64 / 1000.0;
        write!(f, "{}{}", dur, unit)
    }
}

impl PartialEq<std::time::Duration> for Duration { 
    fn eq(&self, other: &std::time::Duration) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<std::time::Duration> for Duration {
    fn partial_cmp(&self, other: &std::time::Duration) -> Option<std::cmp::Ordering> {
       self.0.partial_cmp(other)
    }
}

impl From<std::time::Duration> for Duration {
    fn from(d: std::time::Duration) -> Duration {
        Duration(d)
    }
}

impl Into<std::time::Duration> for Duration {
    fn into(self) -> std::time::Duration {
       self.0 
    }
}
