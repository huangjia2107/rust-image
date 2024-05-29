use image;
use std::fs;

pub fn encode_from_file_16(input_path: &str, output_path: &str, width: u32, min: u16, max: u16) {
    let file_bytes = fs::read(input_path).unwrap();
    let file_bytes_with_single_channel: Vec<u8> = file_bytes
        .chunks(2)
        .map(|b| {
            let pixel: u16 = u16::from_le_bytes([b[1], b[0]]);
            if pixel >= max {
                255
            } else {
                (((pixel - min) as f32) * 255.0 / ((max - min) as f32)) as u8
            }
        })
        .collect();

    encode_from_bytes(&file_bytes_with_single_channel, output_path, width);
}

pub fn encode_from_file_8(input_path: &str, output_path: &str, width: u32) {
    let file_bytes = fs::read(input_path).unwrap();
    encode_from_bytes(&file_bytes, output_path, width);
}

fn encode_from_bytes(bytes: &Vec<u8>, output_path: &str, width: u32) {
    image::save_buffer(
        output_path,
        bytes,
        width,
        bytes.len() as u32 / width,
        image::ExtendedColorType::L8,
    )
    .unwrap();
}
