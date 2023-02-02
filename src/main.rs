use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

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
                .flat_map(|col| [row as u8, col as u8, 128])
                .collect::<Vec<_>>()
        })
        .collect();

    writer.write_image_data(&data).unwrap();
}
