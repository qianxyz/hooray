use std::io;

fn main() {
    hello_ppm(&mut io::stdout()).unwrap();
}

fn hello_ppm(out: &mut impl io::Write) -> io::Result<()> {
    writeln!(out, "P6")?; // magic number
    writeln!(out, "256 256")?; // width and height
    writeln!(out, "255")?; // color range

    for h in 0..=255 {
        for w in 0..=255 {
            out.write_all(&[h, w, 128])?;
        }
    }

    Ok(())
}
