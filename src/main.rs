use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use hooray::{Point3, Ray, Sphere, Vec3, World};

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    // image dimensions
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 360;
    const ASPECT_RATIO: f32 = WIDTH as f32 / HEIGHT as f32;

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // prepare world
    let mut world = World::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // set up camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let viewport_center = Point3::new(0.0, 0.0, -FOCAL_LENGTH);
    let lower_left_corner = viewport_center - horizontal / 2.0 - vertical / 2.0;

    // TODO: remove magic number 3
    let mut data = Vec::with_capacity((3 * WIDTH * HEIGHT) as usize);

    // start from lower left corner, row index reversed
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let u = col as f32 / (WIDTH - 1) as f32;
            let v = row as f32 / (HEIGHT - 1) as f32;

            let p = lower_left_corner + u * horizontal + v * vertical;

            let ray = Ray::new(origin, p - origin);
            let color = ray.color(&world);

            data.extend(color.to_bytes());
        }
    }

    writer.write_image_data(&data).unwrap();
}
