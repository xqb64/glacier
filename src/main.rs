mod colors;

use crate::colors::{Color, NORD_AURORA, NORD_FROST, NORD_POLAR_NIGHT, NORD_SNOW_STORM};
use anyhow::{bail, Result};
use image::{GenericImageView, Pixel};
use std::path::Path;
use structopt::StructOpt;

fn main() {
    let opts = Opt::from_args();
    if let Err(e) = run(opts.path, opts.schemes) {
        eprintln!("glacier: {:?}", e);
    }
}

fn run(path: impl AsRef<Path>, schemes: Vec<Scheme>) -> Result<()> {
    let image = image::open(path)?;

    let mut valid_colors = vec![];

    for scheme in schemes {
        match scheme {
            Scheme::Aurora(c) | Scheme::Frost(c) | Scheme::PolarNight(c) | Scheme::SnowStorm(c) => {
                for color in c {
                    valid_colors.push(color);
                }
            }
        };
    }

    let pixels = image.pixels();
    let rgb_values = pixels
        .map(|(_x, _y, pixel)| pixel.to_rgb())
        .map(|rgb| Color {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        })
        .collect::<Vec<_>>();

    let mut colorized = vec![];

    for rgb in rgb_values {
        let mut min = 255;
        let mut color_idx = 0;

        for (idx, color) in valid_colors.iter().enumerate() {
            let r_diff = color.r.abs_diff(rgb.r);
            let g_diff = color.g.abs_diff(rgb.g);
            let b_diff = color.b.abs_diff(rgb.b);

            let diff: u16 = r_diff as u16 + g_diff as u16 + b_diff as u16;

            if diff < min {
                min = diff;
                color_idx = idx;
            }
        }

        colorized.push(valid_colors[color_idx]);
    }

    image::save_buffer_with_format(
        "out.png",
        &colorized
            .iter()
            .flat_map(|color| vec![color.r, color.g, color.b])
            .collect::<Vec<_>>(),
        image.width(),
        image.height(),
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )?;

    Ok(())
}

#[derive(Debug, Clone)]
enum Scheme {
    Frost(Vec<Color>),
    PolarNight(Vec<Color>),
    SnowStorm(Vec<Color>),
    Aurora(Vec<Color>),
}

impl std::str::FromStr for Scheme {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Scheme> {
        match s {
            "frost" => Ok(Scheme::Frost(NORD_FROST.to_vec())),
            "polar_night" => Ok(Scheme::PolarNight(NORD_POLAR_NIGHT.to_vec())),
            "snow_storm" => Ok(Scheme::SnowStorm(NORD_SNOW_STORM.to_vec())),
            "aurora" => Ok(Scheme::Aurora(NORD_AURORA.to_vec())),
            _ => bail!("unknown scheme"),
        }
    }
}

#[derive(StructOpt)]
struct Opt {
    path: String,

    #[structopt(short, long)]
    schemes: Vec<Scheme>,
}
