use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use hooray::*;

use indicatif::{ProgressBar, ProgressStyle};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use rayon::prelude::*;

fn random_scene(rng: &mut impl RngExt) -> World {
    let mut world = World::new();

    // add ground
    let ground_material = Material::lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // add three big balls
    let glass = Material::dielectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass));
    let matte = Material::lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, matte));
    let metal = Material::metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, metal));

    // add a bunch of small balls
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.float(),
                0.2,
                b as f64 + 0.9 * rng.float(),
            );

            // do not collide with the big boys
            if (center - Point3::new(0.0, 1.0, 0.0)).length() <= 1.2
                || (center - Point3::new(-4.0, 1.0, 0.0)).length() <= 1.2
                || (center - Point3::new(4.0, 1.0, 0.0)).length() <= 1.2
            {
                continue;
            }

            // add ball with random material
            let choose_material = rng.float();
            if choose_material < 0.8 {
                // matte
                let albedo = rng.color() * rng.color();
                let material = Material::lambertian(albedo);
                world.add(Sphere::new(center, 0.2, material));
            } else if choose_material < 0.95 {
                // metal
                let albedo = rng.color_between(0.5, 1.0);
                let fuzz = rng.float_between(0.0, 0.5);
                let material = Material::metal(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, material));
            } else {
                // glass
                let material = Material::dielectric(1.5);
                world.add(Sphere::new(center, 0.2, material));
            }
        }
    }

    world
}

fn main() {
    // image dimensions and render configs
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 400;
    const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;
    const SEED: u64 = 42;

    // prepare world
    let mut rng = ChaCha8Rng::seed_from_u64(SEED);
    let world = random_scene(&mut rng);

    // set up camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let vfov = 20.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    // init progress bar
    let bar = ProgressBar::new((WIDTH * HEIGHT) as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {human_pos:>7}/{human_len:7}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    // the actual rendering
    // start from lower left corner, row index reversed
    let data: Vec<_> = (0..HEIGHT)
        .rev()
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map_iter(|row| {
            let mut rng = ChaCha8Rng::seed_from_u64(SEED);
            rng.set_stream(row as u64);

            // Partial move hack: The closure below needs to move in `rng`,
            // but prefixing `move` makes it greedy for all owned values.
            // We shadow the variables here so it only moves in the borrows.
            let camera = &camera;
            let world = &world;
            let bar = &bar;

            (0..WIDTH).flat_map(move |col| {
                bar.inc(1);

                (0..SAMPLES_PER_PIXEL)
                    .map(|_| {
                        let u = (col as f64 + rng.float()) / (WIDTH - 1) as f64;
                        let v = (row as f64 + rng.float()) / (HEIGHT - 1) as f64;
                        let ray = camera.get_ray(u, v, &mut rng);
                        ray.color(world, MAX_DEPTH, &mut rng)
                    })
                    .fold(Color::default(), |x, y| x + y)
                    .to_bytes(SAMPLES_PER_PIXEL)
            })
        })
        .collect();

    // finish progress bar
    bar.finish();

    // write to png file
    let path = Path::new(r"images/in-one-weekend.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}
