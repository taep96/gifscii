use crate::convert::{PrintFrame, PrintFrames};
use crate::debug::Timer;

use anyhow::Result;
use itertools::Either;

use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, terminal};

pub struct Printer {
    pub t_columns: u16,
    pub t_rows: u16,
    debug_info: bool,
}

impl Printer {
    pub fn new(debug_info: bool) -> Result<Self> {
        let (t_columns, t_rows) = terminal::size()?;

        print!("{}", Clear(ClearType::All));
        print!("{}", cursor::Hide);
        print!("{}", cursor::MoveTo(0, 0));

        ctrlc::set_handler(|| {
            print!("{}", cursor::Show);
            std::process::exit(0);
        })?;

        Ok(Self {
            t_columns,
            t_rows,
            debug_info,
        })
    }

    pub fn print(&self, frames: PrintFrames, cycle: bool) -> Result<()> {
        let frame_count = frames.len();
        let frames = frames.into_iter().enumerate();
        let mut frames = match cycle {
            true => Either::Left(frames.cycle()),
            false => Either::Right(frames),
        };

        frames.try_for_each(|(index, frame)| -> Result<()> {
            let mut timer = Timer::start();
            print!("{}", cursor::MoveTo(0, 0));

            let PrintFrame { frame, delay } = frame;
            print!("{}", frame);
            timer.sleep(delay);

            if self.debug_info {
                println!();
                timer.debug_info(index + 1, frame_count)?;
            }

            Ok(())
        })?;

        print!("{}", cursor::Show);
        Ok(())
    }
}
