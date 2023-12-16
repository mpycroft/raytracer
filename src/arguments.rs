use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Cyan.on_default())
}

#[derive(Parser, Debug)]
#[command(author, version, about, styles = styles())]
pub struct Arguments {
    /// Output FILE to write to
    #[arg(short, long, id = "FILE", default_value = "image.ppm")]
    pub out: String,
    /// Suppress program output
    #[arg(short, long)]
    pub quiet: bool,
}
