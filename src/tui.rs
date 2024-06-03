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

use crate::{aoc, config, time};
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

enum SolutionStatus {
    Waiting,
    Running,
    Done(WorkerResult),
}

struct App {
    config: config::Config,
    tick_rx: mpsc::Receiver<()>,

    solutions: Vec<WorkerResult>,
}

pub fn run(config: config::Config) -> Result<(), std::io::Error> {
    let num_cores = config.num_workers()?;
    let num_days = 25;

    let (tick_tx, tick_rx) = mpsc::channel();

    let solutions = Vec::with_capacity(num_days);

    let app = App {
        config,
        tick_rx,
        solutions,
    };

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        tick_tx.send(()).unwrap();
    });

    let mut count = 0;

    loop {
        if let Ok(_) = app.tick_rx.try_recv() {
            println!("tick!");
            count += 1;
        }

        if count == 10 {
            break;
        }
    }

    Ok(())
}
