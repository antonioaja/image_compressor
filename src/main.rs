use anyhow::*;
use clap::Parser;
use image::ImageEncoder;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(
    author = "Antonio Aguilar",
    version,
    about = "A CLI program to compress image files\nSupports .jpg, .png, .gif, & .bmp"
)]
struct Args {
    /// The file to compress
    #[clap(short, long, value_parser)]
    input: String,

    /// Output file (include extension)
    #[clap(short, long, value_parser)]
    output: String,

    /// The speed of compression (1 => slowest; .gif => [1,30])
    #[clap(short, long, value_parser, default_value_t = 1)]
    speed: u8,

    /// Determine the quality of lossy re-encoding (maximum of 100)
    #[clap(short, long, value_parser, default_value_t = 80)]
    quality: u8,
}

fn main() -> Result<()> {
    // Collect arguments
    let args: Args = Args::parse();
    let input_file = &args.input;
    let output_file = &args.output;
    let quality: u8 = args.quality.clamp(1, 100);
    let speed = args.speed;

    // Validate files' extensions before compressing
    if ext_is_valid(input_file) && ext_is_valid(output_file) {
        let image = image::open(input_file).context(format!("Could not open {}", input_file))?;

        let mut out = File::create(output_file).context("Unable to create file")?;

        let mut data: Vec<u8> = vec![];

        match extension(&output_file.to_lowercase()) {
            ".png" => {
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
                    .context("Could not encode the input image to .png")?;
            }

            ".jpg" | ".jpeg" => {
                let encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut data, quality);
                encoder
                    .write_image(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color(),
                    )
                    .context("Could not encode the input image to .jpg/.jpeg")?;
            }

            ".bmp" => {
                let encoder = image::codecs::bmp::BmpEncoder::new(&mut data);
                encoder
                    .write_image(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color(),
                    )
                    .context("Could not encode the input image to .jpg/.jpeg")?;
            }

            ".gif" => {
                let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(
                    &mut data,
                    speed.clamp(1, 30).into(),
                );
                encoder
                    .encode(
                        image.as_bytes(),
                        image.width(),
                        image.height(),
                        image.color(),
                    )
                    .context("Could not encode the input image to .gif")?;
            }

            _ => bail!("Invalid output file format {}", extension(output_file)),
        }

        out.write_all(&data)
            .context(format!("Could not write to {}", output_file))?;
    } else {
        if !ext_is_valid(input_file) {
            bail!("Invalid input format {}", extension(input_file));
        } else if !ext_is_valid(output_file) {
            bail!("Invalid output format {}", extension(output_file));
        }
    }

    Ok(())
}

pub fn ext_is_valid(filename: &str) -> bool {
    matches!(
        extension(&filename.to_lowercase()),
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
