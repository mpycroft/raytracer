use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

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
    /// Output file to write to
    #[arg(short, long, default_value = "image.ppm")]
    pub out: String,

    /// Input Yaml file to read from
    #[arg(short, long, default_value = "scenes/checkered-sphere.yaml")]
    pub scene: String,

    /// Generate random spheres scene
    #[arg(long, default_value = "false")]
    pub sphere_scene: bool,

    /// Scale the width and height of the image by this value
    #[arg(long, default_value = "1.0")]
    pub scale: f64,

    /// The number of reflection rays to produce
    #[arg(long, default_value = "5")]
    pub depth: u32,

    /// The seed to use when using random numbers
    #[arg[long]]
    pub seed: Option<u64>,

    /// Run the rendering process with a single thread
    #[arg(long)]
    pub single_threaded: bool,

    /// Suppress program output
    #[arg(short, long)]
    pub quiet: bool,
}
