use clap::Args;

use crate::error::{Result, SetError};
use crate::image::compose_and_save_wallpaper;
use crate::monitor::get_monitor_layout;
use crate::theme::extract_and_modify_theme;

/// Arguments for the 'set' command
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Image path to use as wallpaper
    #[arg(short, long, help = "Set the wallpaper image path")]
    pub image: String,

    /// Blur amount for the wallpaper (default: 8)
    #[arg(
        short,
        long,
        default_value_t = 8,
        help = "Set the wallpaper blur amount"
    )]
    pub blur: u32,
}

pub fn run(args: SetArgs) -> Result<()> {
    println!(
        "\nSetting wallpaper to '{}' with blur {}\n",
        args.image, args.blur
    );

    let monitor_layout = get_monitor_layout().map_err(SetError::from)?;

    println!("Detected monitor layout: {:#?}\n", monitor_layout);

    compose_and_save_wallpaper(&args.image, &monitor_layout, args.blur as f32)
        .map_err(SetError::from)?;

    extract_and_modify_theme(&monitor_layout).map_err(SetError::from)?;

    Ok(())
}
