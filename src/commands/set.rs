//! Implements the `set` command for WallGDM.
//!
//! This module provides functionality to set a GDM login screen wallpaper,
//! including image composition, blur application, DPI scaling, and theme modification.

use clap::{Args, ValueEnum};

use crate::config::{AppState, SetDirs};
use crate::error::{self, SetError};
use crate::image::compose_and_save_wallpaper;
use crate::monitor::get_monitor_layout;
use crate::theme::extract_and_modify_theme;

/// Scale factor for high-DPI displays.
///
/// Used to adjust the wallpaper layout for monitors with different pixel densities.
#[derive(ValueEnum, Clone, Debug)]
pub enum ScaleFactor {
    #[clap(name = "1")]
    One,
    #[clap(name = "1.25")]
    OnePointTwoFive,
    #[clap(name = "1.5")]
    OnePointFive,
    #[clap(name = "1.75")]
    OnePointSevenFive,
    #[clap(name = "2")]
    Two,
}

impl ScaleFactor {
    /// Convert the enum variant to its corresponding `f32` value.
    pub fn as_f32(&self) -> f32 {
        match self {
            ScaleFactor::One => 1.0,
            ScaleFactor::OnePointTwoFive => 1.25,
            ScaleFactor::OnePointFive => 1.5,
            ScaleFactor::OnePointSevenFive => 1.75,
            ScaleFactor::Two => 2.0,
        }
    }
}

/// Arguments for the `set` command.
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Image path to use as wallpaper
    #[arg(short, long, help = "Set the wallpaper image path")]
    pub image: String,

    /// Blur amount to apply for the wallpaper (default: 8, max: 50)
    #[arg(
        short,
        long,
        default_value_t = 8,
        value_parser = clap::value_parser!(u32).range(0..=50),
        help = "Set the wallpaper blur amount"
    )]
    pub blur: u32,

    /// Scale factor for high-DPI displays (default: 1)
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = ScaleFactor::One,
        help = "Set the scale factor for high-DPI displays"
    )]
    pub scale: ScaleFactor,
}

/// Execute the `set` command: compose wallpaper, apply blur, and modify the GDM theme.
///
/// Returns a `Result` with a top-level [`WallGdmError`] if any step fails.
///
/// # Steps
/// 1. Prepare working directories.
/// 2. Detect monitor layout (with DPI scaling).
/// 3. Compose and save the wallpaper image.
/// 4. Extract and modify the GNOME Shell theme for the login screen.
///
/// Errors are propagated and should be handled at the top-level.
pub fn run(args: SetArgs) -> error::Result<()> {
    log::info!(
        "Setting gdm login screen wallpaper to '{}' with blur: {} and scale: {:?}",
        args.image,
        args.blur,
        args.scale
    );

    // Prepare working directories
    let working_dirs = SetDirs::new().map_err(SetError::from)?;

    // Detect monitor layout, considering scale factor
    let monitor_layout =
        get_monitor_layout(args.scale.as_f32()).map_err(SetError::from)?;

    // Compose and save the wallpaper image
    let wallpaper_image_path = compose_and_save_wallpaper(
        &working_dirs,
        &args.image,
        &monitor_layout,
        args.blur as f32,
    )
    .map_err(SetError::from)?;

    // Extract theme and update background
    let compiled_gresource =
        extract_and_modify_theme(&working_dirs, &wallpaper_image_path)
            .map_err(SetError::from)?;

    // Install the compiled theme resource using sudo
    crate::commands::install::install(&compiled_gresource)
        .map_err(SetError::from)?;

    // Save the application state for potential future reverts
    let app_state = AppState::new(
        wallpaper_image_path,
        args.blur,
        args.scale.as_f32(),
        monitor_layout,
    );

    let app_state_file_path = working_dirs.data_dir.join("current_state.json");
    app_state
        .save(&app_state_file_path)
        .map_err(SetError::from)?;

    Ok(())
}
