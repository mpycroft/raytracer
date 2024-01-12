mod arguments;
mod scene;

use std::{fs::write, path::Path};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};

use crate::arguments::Arguments;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    if !arguments.quiet {
        print!("Generating scene '{}'...", arguments.scene);
    }

    let scene = arguments.scene.generate(&arguments);

    if !arguments.quiet {
        println!("done");
    }

    let canvas = scene.render(arguments.depth, arguments.quiet)?;

    if !arguments.quiet {
        println!("Writing to file {}", arguments.out);
    }

    let filename = Path::new(&arguments.out);
    if filename.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("ppm"))
    {
        write(filename, canvas.to_ppm())?;
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let image = ImageBuffer::from_fn(
            scene.camera.horizontal_size as u32,
            scene.camera.vertical_size as u32,
            |x, y| Rgb(canvas.get_pixel(x as usize, y as usize).to_u8()),
        );

        image.save(filename)?;
    }

    Ok(())
}
