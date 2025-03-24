#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::style,
    clippy::cargo
)]
#![allow(clippy::multiple_crate_versions)]

use anyhow::{Context, Result, anyhow};
use arboard::Clipboard;
use chrono::Local;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::Config;
use image::{ImageBuffer, Rgba, RgbaImage};
use log::{debug, error, info, trace};
use serde::Deserialize;
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: Verbosity,

    #[arg(short, long)]
    directory: Option<PathBuf>,

    #[arg(long)]
    save_directory: Option<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct AppConfig {
    directory: Option<PathBuf>,
}

fn setup_logging(verbose: Verbosity) -> Result<()> {
    if let Some(level) = verbose.log_level() {
        simple_logger::init_with_level(level)?;
    }

    Ok(())
}

fn get_config_home() -> Result<PathBuf> {
    if let Some(dir) = dirs::config_dir() {
        return Ok(dir);
    }
    Ok(current_dir()?)
}

fn get_config_dir() -> Result<PathBuf> {
    Ok(get_config_home()?.join("clipsaver"))
}

fn get_config() -> Result<AppConfig> {
    let config_dir = get_config_dir()?;
    debug!("Looking for config in {:?}", config_dir);

    let config_name = config_dir.join("config");
    let config_name = config_name
        .to_str()
        .ok_or_else(|| anyhow!("Failed to parse {:?}", config_name))?;

    let config = Config::builder()
        .add_source(config::File::with_name(config_name).required(false))
        .add_source(config::Environment::with_prefix("CLIPSAVER"))
        .build()?
        .try_deserialize::<AppConfig>()?;

    trace!("Config: {:?}", config);

    Ok(config)
}

fn get_save_directory(args: &Args, config: &AppConfig) -> Result<PathBuf> {
    let save_directory = args
        .directory
        .clone()
        .or_else(|| config.directory.clone())
        .unwrap_or_else(|| PathBuf::from("."));

    let x = shellexpand::path::full(&save_directory)?;

    Ok(PathBuf::from(x))
}

fn get_image_from_clipboard() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut clipboard = Clipboard::new().context("Attempted to get clipboard")?;

    let image = clipboard
        .get_image()
        .context("Getting image from clipboard")?;

    RgbaImage::from_raw(
        u32::try_from(image.width)?,
        u32::try_from(image.height)?,
        image.bytes.to_vec(),
    )
    .context("Could not parse image")
}

fn get_save_filename(directory: &Path) -> PathBuf {
    let now = Local::now();
    let filename = format!("Clipboard {}.png", now.format("%Y-%m-%d_%H.%M.%S"));
    directory.join(filename)
}

fn save_image_to_file(image: &ImageBuffer<Rgba<u8>, Vec<u8>>, filename: &PathBuf) -> Result<()> {
    info!("Saving image to {:?}", filename);
    image.save(filename).context("Trying to save image")
}

fn save_directory(directory: &str) -> Result<()> {
    let config_dir = get_config_dir()?;
    let filename = config_dir.join("config.ini");

    debug!("Setting configuration in {:?}", filename);

    fs::create_dir_all(config_dir)?;
    fs::write(filename, format!("directory = {directory}"))?;

    println!("Set save directory to {directory}");

    process::exit(0);
}

fn clipsaver() -> Result<PathBuf> {
    let args = Args::parse();
    setup_logging(args.verbose)?;

    trace!("Command Line: {:?}", args);

    if let Some(dir) = &args.save_directory {
        save_directory(dir)?;
    }

    let config = get_config()?;

    let directory = get_save_directory(&args, &config)?;

    info!("Saving to directory: {:?}", directory);

    let filename = get_save_filename(&directory);

    let image = get_image_from_clipboard()?;
    save_image_to_file(&image, &filename)?;

    Ok(filename)
}

fn main() {
    let file = clipsaver().unwrap_or_else(|error| {
        error!("Failed to save clipboard: {:?}", error);
        process::exit(1);
    });

    println!("Saved image from clipboard to {file:?}");
}
