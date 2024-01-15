mod arguments;
mod scene;

use std::{
    fs::write,
    io::{sink, stdout, Write},
    path::Path,
};

use anyhow::Result;
use clap::Parser;
use either::Either::{Left, Right};
use image::{ImageBuffer, Rgb};

use crate::arguments::Arguments;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    let mut buffer =
        if arguments.quiet { Left(sink()) } else { Right(stdout()) };

    let buffer: &mut dyn Write = &mut buffer;

    write!(buffer, "Generating scene '{}'...", arguments.scene)?;

    let scene = arguments.scene.generate(&arguments);

    writeln!(buffer, "done")?;

    let canvas = scene.render(arguments.depth, arguments.quiet, buffer)?;

    writeln!(buffer, "Writing to file {}", arguments.out)?;

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
