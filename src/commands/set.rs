use clap::Args;

/// Arguments for the 'set' command
#[derive(Args, Debug)]
pub struct SetArgs {
    /// Image path to use as wallpaper
    #[arg(
        short,
        long,
        help = "Set the wallpaper image path"
    )]
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

pub fn run(args: SetArgs) -> Result<(), anyhow::Error> {
    println!(
        "Setting wallpaper to '{}' with blur {}",
        args.image, args.blur
    );
    Ok(())
}
