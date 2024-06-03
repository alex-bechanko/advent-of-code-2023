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

use crossterm::{ExecutableCommand, QueueableCommand};

use crate::{aoc, config, time};
use std::fmt::Display;
use std::io::stdout;
use std::io::Write;
use std::{sync::mpsc, thread, time::Duration};

struct WorkerResult {
    worker_id: usize,
    result: Result<(String, time::Duration), aoc::SolveError>,
}

struct Worker {
    id: usize,
    rx: mpsc::Receiver<(aoc::Day, aoc::Part, std::path::PathBuf)>,
    tx: mpsc::Sender<WorkerResult>,
}

impl Worker {
    fn run(&self, worker_id: usize) -> () {
        loop {
            let (day, part, input) = self.rx.recv().unwrap();

            let result = aoc::solve(day, part, &input);

            self.tx.send(WorkerResult { worker_id, result });
        }
    }
}

struct SolutionState {
    part_a_state: SolutionStatus,
    part_b_state: SolutionStatus,
    day: aoc::Day,

}

enum SolutionStatus {
    Waiting,
    Running,
    Done(WorkerResult),
}

#[derive(Debug, Clone)]
struct CliAnimation {
    frame_index: usize,
    frames: Vec<char>,
}

impl CliAnimation {
    fn update(&mut self) -> () {
        self.frame_index = (self.frame_index + 1) % self.frames.len()
    }
}

impl Display for CliAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.frames[self.frame_index])
    }
}

fn running_animation() -> CliAnimation {
    CliAnimation {
        frame_index: 0,
        frames: vec!['⠏', '⠛', '⠹', '⠼', '⠶', '⠧'],
    }
}

fn waiting_animation() -> CliAnimation {
    CliAnimation {
        frame_index: 0,
        frames: vec!['⠄', '⠄', '⠄', '⠄', ' ', ' ', ' ', ' '],
    }
}

struct App {
    config: config::Config,
    tick_rx: mpsc::Receiver<()>,

    solutions: Vec<SolutionState>,
    animations: CliAnimation,
}

fn print_results(app: &App) -> Result<(), std::io::Error> {
    stdout().execute(crossterm::cursor::Hide)?;

    for sol in app.solutions.iter() {
        println!("{}.{}", sol.day, aoc::Part::A);
        println!("  .{}", aoc::Part::B);
    }

    stdout().execute(crossterm::cursor::MoveUp(50))?;

    Ok(())
}

pub fn run(config: config::Config) -> Result<(), std::io::Error> {
    let num_cores = config.num_workers()?;
    let num_days = 25;

    let (tick_tx, tick_rx) = mpsc::channel();

    let mut solutions = Vec::with_capacity(num_days);
    for d in aoc::Day::all() {
        solutions.push(SolutionState {
            part_a_state: SolutionStatus::Waiting,
            part_b_state: SolutionStatus::Waiting,
            day: d,
        });
    }

    let mut app = App {
        config,
        tick_rx,
        solutions,
        animations: waiting_animation(),
    };

    let tick_interval = app.config.tui_update_interval;

    thread::spawn(move || loop {
        thread::sleep(tick_interval);
        tick_tx.send(()).unwrap();
    });

    let mut count = 0;

    loop {
        if let Ok(_) = app.tick_rx.try_recv() {
            count += 1;
            print_results(&app)?;
            app.animations.update();
        }

        if count == 10 {
            break;
        }
    }

    Ok(())
}
