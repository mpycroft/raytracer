use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

use crate::scene::Scene;

fn styles() -> Styles {
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
    /// Suppress program output
    #[arg(short, long)]
    pub quiet: bool,
    /// Which scene to generate.
    #[arg(short, long, default_value = "chapter10")]
    pub scene: Scene,
    /// The number of reflection rays to produce.
    #[arg(short, long, default_value = "5")]
    pub depth: u32,
}
