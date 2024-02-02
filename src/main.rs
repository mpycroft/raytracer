mod arguments;
mod scene;

use std::{
    fs::write,
    io::{stdout, Write},
    path::Path,
};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use rand::{random, SeedableRng};
use rand_xoshiro::Xoroshiro128PlusPlus;
use raytracer::Output;

use crate::arguments::Arguments;

fn main() -> Result<()> {
    let arguments = Arguments::parse();

    let mut output = if arguments.quiet {
        Output::new_sink()
    } else {
        Output::new(stdout())
    };

    let seed = arguments.seed.unwrap_or_else(random);

    writeln!(output, "Using RNG seed {seed}")?;

    let mut rng = Xoroshiro128PlusPlus::seed_from_u64(seed);

    let scene_text = format!("Generating scene '{}'...", arguments.scene);
    writeln!(output, "{scene_text}")?;

    let scene = arguments.scene.generate(&arguments, &mut rng);

    output.clear_last_line()?;
    writeln!(output, "{scene_text}done")?;

    let canvas = scene.render(
        arguments.depth,
        arguments.single_threaded,
        &mut output,
    )?;

    writeln!(output, "Writing to file {}", arguments.out)?;

    let filename = Path::new(&arguments.out);
    if filename.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("ppm"))
    {
        write(filename, canvas.to_ppm())?;
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let image = ImageBuffer::from_fn(
            scene.camera.horizontal_size(),
            scene.camera.vertical_size(),
            |x, y| Rgb(canvas.get_pixel(x as usize, y as usize).to_u8()),
        );

        image.save(filename)?;
    }

    Ok(())
}
