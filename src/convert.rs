use std::time::Duration;

use image::Frame;

use crossterm::style::Color::Rgb;
use crossterm::style::Stylize;

pub type Frames = image::Frames<'static>;
pub type PrintFrames = Vec<PrintFrame>;
pub trait FromFrames {
    fn from_frames(frames: Frames) -> PrintFrames;
}

impl FromFrames for PrintFrames {
    fn from_frames(frames: Frames) -> PrintFrames {
        frames
            .map(|frame| PrintFrame::as_pixels(frame.expect("Couldn't get frame")))
            .collect()
    }
}

#[derive(Clone)]
pub struct PrintFrame {
    pub frame: String,
    pub delay: Duration,
}

const EMPTY: char = ' ';
const BOTTOM: char = '▄';
const TOP: char = '▀';

impl PrintFrame {
    fn as_pixels(frame: Frame) -> PrintFrame {
        let buffer = frame.buffer();

        let width = buffer.width();
        let height = buffer.height();
        let delay = Duration::from(frame.delay());

        let mut frame = String::new();
        for row in 0..height / 2 {
            for column in 0..width {
                let [r, g, b, a1] = buffer.get_pixel(column, row * 2).0;
                let top = Rgb { r, g, b };

                let [r, g, b, a2] = buffer.get_pixel(column, row * 2 + 1).0;
                let bottom = Rgb { r, g, b };

                // frame.push_str(&TOP.with(fg).on(bg).to_string());
                match (a1, a2) {
                    (0, 0) => frame.push(EMPTY),
                    (0, _) => frame.push_str(&BOTTOM.with(bottom).to_string()),
                    (_, 0) => frame.push_str(&TOP.with(top).to_string()),
                    (_, _) => frame.push_str(&TOP.with(top).on(bottom).to_string()),
                }
            }

            if row != height / 2 - 1 {
                frame.push('\n');
            }
        }

        PrintFrame { frame, delay }
    }
}
