use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use hooray::*;

use indicatif::ProgressBar;

fn main() {
    // image dimensions and render configs
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 360;
    const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // prepare world
    let mut world = World::new();

    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let matte = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let metal = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);
    let glass = Dielectric::new(1.5);

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, matte));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, glass));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, metal));

    // set up camera
    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = (look_at - look_from).length();
    let vfov = 20.0;
    let aperture = 2.0;

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

    // clear progress bar
    bar.finish_and_clear();

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
