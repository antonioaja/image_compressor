use anyhow::*;
use clap::Parser;
use std::fs::File;
use std::io::Write;

pub mod formats;

#[derive(Parser, Debug)]
#[clap(author, version, about = "A CLI program to batch convert image files", long_about = None)]
struct Args {
    /// The file to delete meta from
    #[clap(short, long, value_parser)]
    file: String,

    /// Determine the quality of lossy re-enocding (maximum of 100)
    #[clap(short, long, value_parser, default_value_t = 80)]
    quality: u8,
}

fn main() -> Result<()> {
    // Collect arguments
    let args: Args = Args::parse();
    let input_file = &args.file;
    let quality: u8 = args.quality.clamp(1, 100);

    if formats::ext_is_valid(input_file) {
        let data = match formats::extension(input_file) {
            ".png" => formats::png::reencode_png(input_file)?,
            ".jpg" => formats::jpg::reencode_jpg(input_file, quality)?,
            ".jpeg" => formats::jpg::reencode_jpg(input_file, quality)?,
            _ => [].to_vec(),
        };

        if data.is_empty() {
            bail!("File is not supported");
        }

        let mut out = File::create(input_file).context("Unable to create file")?;

        out.write_all(&data).context("Could not write to file")?;
    } else {
        println!("Invalid image format!")
    }

    Ok(())
}
