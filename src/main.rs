mod vec3;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use vec3::Vec3;

fn main() {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    let mut encoder = png::Encoder::new(w, WIDTH, HEIGHT);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let data: Vec<_> = (0..HEIGHT)
        .flat_map(|row| {
            (0..WIDTH)
                .flat_map(|col| {
                    let v = Vec3::new(
                        col as f32 / (WIDTH - 1) as f32,
                        row as f32 / (HEIGHT - 1) as f32,
                        0.25,
                    );

                    v.color()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    writer.write_image_data(&data).unwrap();
}
