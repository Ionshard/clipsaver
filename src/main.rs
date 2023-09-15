use anyhow::{Context, Result};
use arboard::Clipboard;
use chrono::Local;
use clap::Parser;
use image::RgbaImage;
use log::info;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

fn filename(directory: &PathBuf) -> PathBuf {
    let now = Local::now();
    let filename = format!("Clipboard {}.png", now.format("%Y-%m-%d_%H.%M.%S"));
    directory.join(filename)
}

fn main() -> Result<()> {
    simple_logger::init()?;
    let args = Args::parse();
    let mut clipboard = Clipboard::new()?;
    let image = clipboard.get_image()?;
    let img = RgbaImage::from_raw(
        image.width as u32,
        image.height as u32,
        image.bytes.to_vec(),
    )
    .context("Could not parse image")?;
    let directory = args.directory.unwrap_or(PathBuf::from("."));
    let file = filename(&directory);
    info!("Saving image to {}", file.display());
    img.save(file)?;
    Ok(())
}
