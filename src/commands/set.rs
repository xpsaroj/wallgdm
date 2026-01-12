use clap::{Args, ValueEnum};

use crate::error::{self, SetError};
use crate::image::compose_and_save_wallpaper;
use crate::monitor::get_monitor_layout;
use crate::theme::extract_and_modify_theme;

/// Scale factor for high-DPI displays
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

pub fn run(args: SetArgs) -> error::Result<()> {
    println!(
        "\nSetting wallpaper to '{}' with blur {}\n",
        args.image, args.blur
    );

    let monitor_layout = get_monitor_layout(args.scale.as_f32() ).map_err(SetError::from)?;

    compose_and_save_wallpaper(
        &args.image,
        &monitor_layout,
        args.blur as f32,
    )
    .map_err(SetError::from)?;

    extract_and_modify_theme().map_err(SetError::from)?;

    Ok(())
}
