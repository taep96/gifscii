mod convert;
mod debug;
mod print;
mod scale;

use crate::convert::{FromFrames, PrintFrames};
use crate::print::Printer;
use crate::scale::{Scale, ScalingFilter, ScalingOptions};

use anyhow::Result;
use std::io::BufReader;

use clap::Parser;
use clio::Input;

use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;

#[derive(Parser)]
#[command(version)]
/// A CLI to animate GIFs in the terminal
struct Args {
    /// URL/stdin/path to the GIF file
    #[arg(value_parser)]
    input: Input,

    /// Loop the animation
    #[arg(short, long)]
    no_loop: bool,

    /// Scaling filter
    #[arg(short, long, value_enum, default_value_t)]
    filter: ScalingFilter,

    /// Print debug info - progress, frame time/rate, delays
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let cycle = !args.no_loop;
    let filter = args.filter;
    let debug_info = args.debug || cfg!(debug_assertions);

    let printer = Printer::new(debug_info)?;
    let compensate_height = if debug_info { 1 } else { 0 };
    let new_width = printer.t_columns as u32;
    let new_height = (printer.t_rows as u32 - compensate_height) * 2;

    println!("loading…");
    let reader = BufReader::new(args.input);
    let decoder = GifDecoder::new(reader)?;

    let options = ScalingOptions {
        filter,
        new_width,
        new_height,
    };
    let frames = decoder.into_frames().scale(options);
    let processed = PrintFrames::from_frames(frames);

    printer.print(processed, cycle)?;
    println!();

    Ok(())
}
