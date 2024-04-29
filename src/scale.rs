use crate::convert::Frames;

use clap::ValueEnum;

use image::imageops::FilterType;
use image::{DynamicImage, Frame, RgbaImage};

#[derive(Clone, Copy, Default, ValueEnum)]
pub enum ScalingFilter {
    #[default]
    /// Lanczos with window 3
    Lanczos3,
    /// Gaussian Filter
    Gaussian,
    /// Cubic Filter
    CatmullRom,
    /// Linear Filter
    Triangle,
    /// Nearest Neighbor
    Nearest,
    /// Disable scaling
    None,
}

impl From<ScalingFilter> for FilterType {
    fn from(filter: ScalingFilter) -> Self {
        use FilterType as FT;
        use ScalingFilter as SF;

        match filter {
            SF::Nearest => FT::Nearest,
            SF::Triangle => FT::Triangle,
            SF::CatmullRom => FT::CatmullRom,
            SF::Gaussian => FT::Gaussian,
            SF::Lanczos3 => FT::Lanczos3,
            SF::None => unreachable!(),
        }
    }
}

pub struct ScalingOptions {
    pub filter: ScalingFilter,
    pub new_width: u32,
    pub new_height: u32,
}

pub trait Scale {
    fn scale(self, scale: ScalingOptions) -> Self;
}

impl Scale for Frames {
    fn scale(
        self,
        ScalingOptions {
            filter,
            new_width,
            new_height,
        }: ScalingOptions,
    ) -> Self {
        match filter {
            ScalingFilter::None => self,
            scaling_filter => {
                let frames = self.map(move |frame| {
                    let frame = frame?;
                    let delay = frame.delay();
                    let image = DynamicImage::from(frame.into_buffer());

                    let resized =
                        image.resize(new_width, new_height, FilterType::from(scaling_filter));
                    let frame = Frame::from_parts(RgbaImage::from(resized), 0, 0, delay);

                    Ok(frame)
                });
                Frames::new(Box::new(frames))
            }
        }
    }
}
