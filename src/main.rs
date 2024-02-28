mod arguments;

use std::{
    fs::write,
    io::{stdout, Write},
    path::Path,
};

use anyhow::Result;
use clap::Parser;
use image::{ImageBuffer, Rgb};
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;
use raytracer::{Output, Scene};

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

    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);

    let scene_text = if arguments.sphere_scene {
        String::from("Generating scene 'random-spheres'...")
    } else {
        format!("Generating scene '{}'...", arguments.scene)
    };
    writeln!(output, "{scene_text}")?;

    let scene = if arguments.sphere_scene {
        Scene::generate_random_spheres(arguments.scale, &mut rng)
    } else {
        Scene::from_file(arguments.scene, arguments.scale, &mut rng)?
    };

    output.clear_last_line()?;

    writeln!(output, "{scene_text}done")?;

    let canvas = scene.render(
        arguments.depth,
        arguments.single_threaded,
        &mut output,
        &mut rng,
    )?;

    writeln!(output, "Writing to file {}", arguments.out)?;

    let filename = Path::new(&arguments.out);
    if filename.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("ppm"))
    {
        write(filename, canvas.to_ppm())?;
    } else {
        #[allow(clippy::cast_possible_truncation)]
        let image = ImageBuffer::from_fn(
            scene.horizontal_size(),
            scene.vertical_size(),
            |x, y| Rgb(canvas.get_pixel(x as usize, y as usize).to_u8()),
        );

        image.save(filename)?;
    }

    Ok(())
}
