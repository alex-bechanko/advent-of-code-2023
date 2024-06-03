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

use std::io;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

pub struct Config {
    pub inputs_folder: PathBuf,

    pub total_goal_time: Duration,

    pub solution_goal_time: Duration,
    pub solution_goal_time_green_threshold: Duration,
    pub solution_goal_time_yellow_threshold: Duration,

    pub tui_update_interval: Duration,
    worker_amount: WorkerAmount,
}

const DEFAULT_INPUTS_FOLDER: &str = "inputs";

const TOTAL_GOAL_TIME_MS: u64 = 60000;

const SOLUTION_GOAL_TIME_MS: u64 = 1000;
const SOLUTION_GOAL_TIME_GREEN_THRESHOLD_MS: u64 = 200;
const SOLUTION_GOAL_TIME_YELLOW_THRESHOLD_MS: u64 = 500;

const TUI_UPDATE_INTERVAL_MS: u64 = 16;

pub enum WorkerAmount{
    SetAmount(NonZeroUsize),
    NumCores,
}

const MAX_THREADS: WorkerAmount = WorkerAmount::NumCores;


impl Config {
    pub fn num_workers(&self) -> io::Result<NonZeroUsize> {
        match self.worker_amount {
            WorkerAmount::NumCores => std::thread::available_parallelism(),
            WorkerAmount::SetAmount(x) => Ok(x),
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        Config {
            inputs_folder: PathBuf::from_str(DEFAULT_INPUTS_FOLDER).unwrap(),
            total_goal_time: Duration::from_millis(TOTAL_GOAL_TIME_MS),
            solution_goal_time: Duration::from_millis(SOLUTION_GOAL_TIME_MS),
            solution_goal_time_green_threshold: Duration::from_millis(
                SOLUTION_GOAL_TIME_GREEN_THRESHOLD_MS,
            ),
            solution_goal_time_yellow_threshold: Duration::from_millis(
                SOLUTION_GOAL_TIME_YELLOW_THRESHOLD_MS,
            ),
            tui_update_interval: Duration::from_millis(TUI_UPDATE_INTERVAL_MS),
            worker_amount: MAX_THREADS
        }
    }
}
