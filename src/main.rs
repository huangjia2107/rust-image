use std::fs;
use tiff_wrap::*;
mod tiff_wrap;

fn main() {
    let input_path = r"d:\\A1_B1_merged_1-260_107_1023.band";
    let image_data_with_u8 = fs::read(input_path).unwrap();
    let image_data: Vec<u16> = image_data_with_u8
        .chunks(2)
        .map(|b| u16::from_le_bytes([b[1], b[0]]))
        .collect();

    let min = *image_data.iter().min().unwrap();
    let max = *image_data.iter().max().unwrap();

    println!("min: {}, max: {}", min, max);

    encode_from_file_16(input_path, "d:\\ddd.tiff", 1980, min, max);
}
