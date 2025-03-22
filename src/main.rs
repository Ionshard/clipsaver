use anyhow::{Context, Result, anyhow};
use arboard::Clipboard;
use chrono::Local;
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::Config;
use image::{ImageBuffer, Rgba, RgbaImage};
use log::{debug, error, info, trace};
use xdg::BaseDirectories;
use std::{path::PathBuf, process};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    verbose: Verbosity,

    #[arg(short, long)]
    directory: Option<PathBuf>,
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

fn get_config(xdg_dirs: &BaseDirectories) -> Result<AppConfig> {
    let config_home = xdg_dirs.get_config_home();
    debug!("Looking for config in {:?}", config_home);

    let config_name = config_home.join("config");
    let config_name = config_name.to_str().ok_or(anyhow!("Failed to parse {:?}", config_name))?;

    let config = Config::builder()
    .add_source(config::File::with_name(config_name).required(false))
    .add_source(config::Environment::with_prefix("CLIPSAVER"))
    .build()?
    .try_deserialize::<AppConfig>()?;

    trace!("Config: {:?}", config);

    Ok(config)
}

fn get_save_directory(args: &Args, config: &AppConfig) -> PathBuf {
    if let Some(directory) = &args.directory {
        return directory.clone();
    }

    if let Some(directory) = &config.directory {
        return directory.clone();
    }

    return PathBuf::from(".")
}

fn get_image_from_clipboard() -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut clipboard = Clipboard::new().context("Attempted to get clipboard")?;

    let image = clipboard
        .get_image()
        .context("Getting image from clipboard")?;
    
    RgbaImage::from_raw(
        image.width as u32,
        image.height as u32,
        image.bytes.to_vec(),
    )
    .context("Could not parse image")
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

    image.save(file).context("Trying to save image")
}

fn clipsaver() -> Result<()> {
    let args = Args::parse();
    setup_logging(args.verbose)?;

    trace!("Command Line Args: {:?}", args);    

    let xdg_dirs = xdg::BaseDirectories::with_prefix("clipsaver")?;
    let config = get_config(&xdg_dirs)?;

    let directory = get_save_directory(&args, &config);

    info!("Saving to directory: {:?}", directory);

    let image = get_image_from_clipboard()?;
    save_image_to_directory(image, &directory)
}

fn main() {
    clipsaver().unwrap_or_else(|error| {
        error!("Failed to save clipboard: {:?}", error);
        process::exit(1);
    })
}
