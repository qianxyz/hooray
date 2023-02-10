use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use hooray::*;

use indicatif::{ProgressBar, ProgressStyle};

fn random_scene() -> World {
    let mut world = World::new();

    // add ground
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    // add three big balls
    let glass = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass));
    let matte = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, matte));
    let metal = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, metal));

    // add a bunch of small balls
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * random::float(),
                0.2,
                b as f64 + 0.9 * random::float(),
            );

            // do not collide with the big boys
            if (center - Point3::new(0.0, 1.0, 0.0)).length() <= 1.2
                || (center - Point3::new(-4.0, 1.0, 0.0)).length() <= 1.2
                || (center - Point3::new(4.0, 1.0, 0.0)).length() <= 1.2
            {
                continue;
            }

            // add ball with random material
            let choose_material = random::float();
            if choose_material < 0.8 {
                // matte
                let albedo = random::color() * random::color();
                let material = Lambertian::new(albedo);
                world.add(Sphere::new(center, 0.2, material));
            } else if choose_material < 0.95 {
                // metal
                let albedo = random::color_between(0.5, 1.0);
                let fuzz = random::float_between(0.0, 0.5);
                let material = Metal::new(albedo, fuzz);
                world.add(Sphere::new(center, 0.2, material));
            } else {
                // glass
                let material = Dielectric::new(1.5);
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

    // prepare world
    let world = random_scene();

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

    // alloc image data buffer
    let mut data = Vec::with_capacity((3 * WIDTH * HEIGHT) as usize);

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
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (col as f64 + random::float()) / (WIDTH - 1) as f64;
                let v = (row as f64 + random::float()) / (HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world, MAX_DEPTH);
            }
            data.extend(pixel_color.to_bytes(SAMPLES_PER_PIXEL));

            bar.inc(1);
        }
    }

    // finish progress bar
    bar.finish();

    // write to png file
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}
