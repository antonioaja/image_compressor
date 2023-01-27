use anyhow::*;
use image::ImageEncoder;

pub fn reencode_jpg(filename: &str, quality: u8) -> Result<Vec<u8>> {
    let image = image::open(filename).context(format!("Could not open {}", filename))?;

    let mut data: Vec<u8> = [].to_vec();

    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut data, quality);

    encoder
        .write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color(),
        )
        .context("Could not encode image")?;

    Ok(data)
}
