use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn write_to_file(path: &Path, buffer: &Vec<u32>, width: usize, height: usize) {
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = buffer
        .iter()
        .flat_map(|x| {
            let mut m: Vec<u8> = Vec::new();
            m.push(((x & (2u32.pow(24) - 1)) >> 16) as u8);
            m.push(((x & (2u32.pow(16) - 1)) >> 8) as u8);
            m.push((x & (2u32.pow(8) - 1)) as u8);
            m
        })
        .collect::<Vec<u8>>();

    writer.write_image_data(&data).unwrap();
}
