use anyhow::*;
use clap::Parser;
use image::ImageEncoder;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A CLI program to convert image files", long_about = None)]
struct Args {
    /// The file to compress
    #[clap(short, long, value_parser)]
    file: String,

    /// Output file (include extension)
    #[clap(short, long, value_parser)]
    output: String,

    /// The speed of compression (1 => slowest ; 10 => fastest)
    #[clap(short, long, value_parser, default_value_t = 3)]
    speed: u8,

    /// Determine the quality of lossy re-encoding (maximum of 100)
    #[clap(short, long, value_parser, default_value_t = 80)]
    quality: u8,
}

fn main() -> Result<()> {
    // Collect arguments
    let args: Args = Args::parse();
    let input_file = &args.file;
    let output_file = &args.output;
    let _quality: u8 = args.quality.clamp(1, 100);
    let _speed = args.speed.clamp(1, 10);

    if ext_is_valid(input_file) {
        let image = image::open(input_file).context(format!("Could not open {}", input_file))?;

        let mut out = File::create(output_file).context("Unable to create file")?;

        let mut data: Vec<u8> = [].to_vec();

        let encoder = image::codecs::png::PngEncoder::new_with_quality(
            &mut data,
            image::codecs::png::CompressionType::Best,
            image::codecs::png::FilterType::Adaptive,
        );

        encoder
            .write_image(
                image.as_bytes(),
                image.width(),
                image.height(),
                image.color(),
            )
            .context("Could not encode image")?;

        out.write_all(&data).context("Could not write to file")?;
    } else {
        println!("Invalid image format!")
    }

    Ok(())
}

pub fn ext_is_valid(filename: &str) -> bool {
    matches!(
        extension(filename),
        ".jpg" | ".jpeg" | ".png" | ".gif" | ".bmp"
    )
}
pub fn extension(filename: &str) -> &str {
    filename
        .rfind('.')
        .map(|idx| &filename[idx..])
        .filter(|ext| ext.chars().skip(1).all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("")
}
