use anyhow::Result;
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use exmex::eval_str;
use raytracer::math::Angle;

use crate::scene::Scene;

const fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, styles = styles())]
pub struct Arguments {
    /// Output FILE to write to
    #[arg(short, long, id = "FILE", default_value = "image.ppm")]
    pub out: String,

    /// Which scene to generate.
    #[arg(short, long, default_value = "chapter13")]
    pub scene: Scene,

    /// Camera width (in pixels)
    #[arg(long)]
    pub width: Option<usize>,

    /// Camera height (in pixels)
    #[arg(long)]
    pub height: Option<usize>,

    /// Field of view (in radians)
    #[arg(long, value_parser = parse_fov)]
    pub fov: Option<Angle>,

    /// The number of reflection rays to produce.
    #[arg(long, default_value = "5")]
    pub depth: u32,

    /// Suppress program output
    #[arg(short, long)]
    pub quiet: bool,
}

fn parse_fov(string: &str) -> Result<Angle> {
    let result = eval_str::<f64>(string)?;

    Ok(Angle(result))
}
