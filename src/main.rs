mod ray;
mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    // image dimensions
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 360;
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // set up camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let viewport_center = -Point3::new(0.0, 0.0, focal_length);
    let lower_left_corner = viewport_center - horizontal / 2.0 - vertical / 2.0;

    let data: Vec<_> = (0..HEIGHT)
        .rev()
        .flat_map(|row| {
            (0..WIDTH)
                .flat_map(|col| {
                    let u = col as f32 / (WIDTH - 1) as f32;
                    let v = row as f32 / (HEIGHT - 1) as f32;

                    let p = lower_left_corner + u * horizontal + v * vertical;

                    let ray = Ray::new(origin, p - origin);

                    ray.color()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    writer.write_image_data(&data).unwrap();
}
