use anyhow::Result;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crossterm::style::Color::Rgb;
use crossterm::style::{Attribute, Color, Colors, ResetColor, SetColors};
use crossterm::terminal::{Clear, ClearType};

const BLACK: Color = Rgb {
    r: 10,
    g: 10,
    b: 10,
};
const WHITE: Color = Rgb {
    r: 250,
    g: 250,
    b: 250,
};
const YELLOW: Color = Rgb {
    r: 245,
    g: 158,
    b: 11,
};
const RED: Color = Rgb {
    r: 185,
    g: 28,
    b: 28,
};

pub struct Timer {
    start: Instant,
    late: Option<Duration>,
}

impl Timer {
    pub fn start() -> Self {
        let start = Instant::now();
        Self { start, late: None }
    }

    pub fn sleep(&mut self, delay: Duration) {
        let elapsed = self.start.elapsed();
        if delay > elapsed {
            sleep(delay - elapsed)
        } else {
            self.late = Some(elapsed - delay);
        };
    }

    pub fn debug_info(&self, frame: usize, frames: usize) -> Result<()> {
        let frame_time = self.start.elapsed();
        let fps = 1.0 / frame_time.as_secs_f64();

        print!("{}", Clear(ClearType::CurrentLine));
        print!("{}", SetColors(Colors::new(YELLOW, BLACK)));
        print!("{}", Attribute::Bold);
        print!(
            "Frame [{:0width$}/{}] in {}ms ({:.2}fps) ",
            frame,
            frames,
            frame_time.as_millis(),
            fps,
            width = frames.to_string().len()
        );

        if let Some(late) = self.late {
            print!("{}", SetColors(Colors::new(WHITE, RED)));
            print!(" +{}ms late ", late.as_millis());
        }

        print!("{}", ResetColor);

        Ok(())
    }
}
