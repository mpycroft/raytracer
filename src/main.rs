mod arguments;
mod scene;

use std::fs::write;

use anyhow::Result;
use clap::Parser;

use crate::arguments::Arguments;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    if !arguments.quiet {
        print!("Generating scene '{}'...", arguments.scene);
    }

    let scene = arguments.scene.generate();

    if !arguments.quiet {
        println!("done");
    }

    let canvas = scene.render(arguments.depth, arguments.quiet);

    if !arguments.quiet {
        println!("Writing to file {}", arguments.out);
    }

    write(arguments.out, canvas.to_ppm())?;

    Ok(())
}
