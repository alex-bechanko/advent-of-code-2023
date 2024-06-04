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

use crossterm::ExecutableCommand;

use crate::spinner::{self, Spinner};
use crate::{aoc, config, time};
use std::fmt::Display;
use std::{sync::mpsc, thread};

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

enum PartStatus {
    Waiting(Spinner),
    Running(Spinner),
    Done(WorkerResult)
}

impl PartStatus {
    fn tick(&mut self) -> () {
        match self {
            Self::Waiting(a) => a.tick(),
            Self::Running(a) => a.tick(),
            _ => ()
        }
    }
}

impl Display for PartStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Waiting(anim) => write!(f, "{}", anim),
            Self::Running(anim) => write!(f, "{}", anim),
            Self::Done(_) => write!(f, "✓"),
        }
    }
}


struct SolutionStatus {
    part_a: PartStatus,
    part_b: PartStatus,
    day: aoc::Day,
}

impl SolutionStatus {
    fn tick(&mut self) -> () {
        self.part_a.tick();
        self.part_b.tick();
    }
}

impl Display for SolutionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}.a {}\n  .b {}", self.day, self.part_a, self.part_b)
    }
}


struct App {
    config: config::Config,
    tick_rx: mpsc::Receiver<()>,

    solutions: Vec<SolutionStatus>,
}

fn print_results(app: &App) -> Result<(), std::io::Error> {

    for sol in app.solutions.iter() {
        println!("{}", sol);
    }

    Ok(())
}

fn init_terminal() -> Result<(), std::io::Error> {
    std::io::stdout().execute(crossterm::cursor::Hide)?;

    Ok(())
}

fn reset_terminal() -> Result<(), std::io::Error> {
    std::io::stdout().execute(crossterm::cursor::MoveUp(50))?;

    Ok(())
}

pub fn run(config: config::Config) -> Result<(), std::io::Error> {
    let num_cores = config.num_workers()?;
    let num_days = 25;

    let (tick_tx, tick_rx) = mpsc::channel();

    let mut solutions = Vec::with_capacity(num_days);
    for d in aoc::Day::all() {
        solutions.push(SolutionStatus {
            day: d,
            part_a: PartStatus::Waiting(spinner::waiting()),
            part_b: PartStatus::Waiting(spinner::waiting()),
        });
    }

    let mut app = App {
        config,
        tick_rx,
        solutions,
    };

    let tick_interval = app.config.tui_update_interval;

    thread::spawn(move || loop {
        thread::sleep(tick_interval);
        tick_tx.send(()).unwrap();
    });

    let mut count = 0;

    init_terminal()?;
    print_results(&app)?;

    loop {
        if let Ok(_) = app.tick_rx.try_recv() {
            count += 1;
            reset_terminal()?;
            print_results(&app)?;

            for sol in app.solutions.iter_mut() {
                sol.tick();
            } 

        }

        if count == 10 {
            break;
        }
    }

    Ok(())
}
