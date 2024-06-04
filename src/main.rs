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

mod aoc;
mod time;

const WAITING_ANIMATION: [&str; 6] = [".  ", ".. ", "...", " ..", "  .", "   "];

fn run_part(day: aoc::Day, part: aoc::Part) -> Result<(), std::io::Error> {
    let input_folder = std::path::PathBuf::from("inputs");
    let input = aoc::day_file(&input_folder, day);
    let (ans_tx, ans_rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let res = aoc::solve(day, aoc::Part::A, &input);
        ans_tx.send(res).unwrap();
    });

    // loop while we wait for thread to return answer
    // update cli waiting animation for the solution as we do
    let mut animation_index: usize = 0;

    match part {
        aoc::Part::A => print!("{:02}.a  ", day),
        aoc::Part::B => print!("  .b  "),
    }

    loop {
        if let Ok(res) = ans_rx.try_recv() {
            match res {
                Ok((answer, elapsed)) => println!("{:10}  {}", elapsed, answer),
                Err(err) => println!("{:10}  {}", "n/a", err),
            }

            break;
        } else {
            std::io::stdout().execute(crossterm::cursor::SavePosition)?;
            print!("{}", WAITING_ANIMATION[animation_index]);
            animation_index = (animation_index + 1) % WAITING_ANIMATION.len();
            std::io::stdout().execute(crossterm::cursor::RestorePosition)?;
        }
    }

    Ok(())
}

fn run() -> Result<(), std::io::Error> {
    std::io::stdout().execute(crossterm::cursor::Hide)?;

    for day in aoc::Day::all() {
        run_part(day, aoc::Part::A)?;
        run_part(day, aoc::Part::B)?;
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    run()
}
