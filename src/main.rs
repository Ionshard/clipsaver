use anyhow::{Context, Result};
use arboard::Clipboard;
use chrono::Local;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use image::{ImageBuffer, Rgba, RgbaImage};
use log::{error, info};
use std::{path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: Verbosity,

    #[arg(short, long, default_value = ".")]
    directory: PathBuf,
}

fn setup_logging(verbose: Verbosity) -> Result<()> {
    if let Some(level) = verbose.log_level() {
        simple_logger::init_with_level(level)?;
    }

    Ok(())
}

fn get_image_from_clipboard() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut clipboard = Clipboard::new().context("Attempted to get clipboard")?;

    let image = clipboard
        .get_image()
        .context("Getting image from clipboard")?;
    let image = RgbaImage::from_raw(
        image.width as u32,
        image.height as u32,
        image.bytes.to_vec(),
    )
    .context("Could not parse image")?;

    Ok(image)
}

fn filename(directory: &PathBuf) -> PathBuf {
    let now = Local::now();
    let filename = format!("Clipboard {}.png", now.format("%Y-%m-%d_%H.%M.%S"));
    directory.join(filename)
}

fn save_image_to_directory(
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    directory: &PathBuf,
) -> Result<()> {
    let file = filename(&directory);

    info!("Saving image to {}", file.display());

    image.save(file).context("Trying to save image")?;

    Ok(())
}

fn clipsaver() -> Result<()> {
    let args = Args::parse();
    
    setup_logging(args.verbose)?;

    let directory = args.directory;
    info!("Saving to directory: {:?}", directory);

    let image = get_image_from_clipboard()?;
    save_image_to_directory(image, &directory)?;

    Ok(())
}

fn main() {
    clipsaver().unwrap_or_else(|error| {
        error!("Failed to save clipboard: {:?}", error);
        process::exit(1);
    })
}
