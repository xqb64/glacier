use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub static NORD_FROST: [Color; 4] = [
    Color {
        r: 143,
        g: 188,
        b: 187,
    },
    Color {
        r: 136,
        g: 192,
        b: 208,
    },
    Color {
        r: 129,
        g: 161,
        b: 193,
    },
    Color {
        r: 94,
        g: 129,
        b: 172,
    },
];

pub static NORD_POLAR_NIGHT: [Color; 4] = [
    Color {
        r: 46,
        g: 52,
        b: 64,
    },
    Color {
        r: 59,
        g: 66,
        b: 82,
    },
    Color {
        r: 67,
        g: 76,
        b: 94,
    },
    Color {
        r: 76,
        g: 86,
        b: 106,
    },
];

pub static NORD_SNOW_STORM: [Color; 3] = [
    Color {
        r: 216,
        g: 222,
        b: 233,
    },
    Color {
        r: 229,
        g: 233,
        b: 240,
    },
    Color {
        r: 236,
        g: 239,
        b: 244,
    },
];

pub static NORD_AURORA: [Color; 5] = [
    Color {
        r: 191,
        g: 97,
        b: 106,
    },
    Color {
        r: 208,
        g: 135,
        b: 112,
    },
    Color {
        r: 235,
        g: 203,
        b: 139,
    },
    Color {
        r: 163,
        g: 190,
        b: 140,
    },
    Color {
        r: 180,
        g: 142,
        b: 173,
    },
];

#[derive(Debug, Clone)]
pub enum Scheme {
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
