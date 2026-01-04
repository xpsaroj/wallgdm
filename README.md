# wallgdm

A simple Rust CLI tool to set wallpapers on GDM login screen with proper multi-monitor support.

## What it does

Sets custom wallpapers on GNOME's login screen (GDM) by:
- Applying blur effects (CSS doesn't support backdrop-filter)
- Handling multi-monitor setups (prevents wallpaper splitting)
- Automatically managing theme extraction, modification, and compilation
- Creating backups for easy revert

## Commands

```bash
wallgdm set --image <IMAGE> [--blur <amount>]  # Set wallpaper (default blur: 8)
wallgdm revert                                 # Restore original theme
```

## How it works

1. Extracts GNOME Shell theme from binary gresource
2. Detects monitor setup with xrandr
3. Creates blurred composite image for total resolution
4. Updates CSS with new wallpaper and resolution
5. Compiles and installs modified theme

## Project Structure

```
wallgdm/
├── Cargo.toml
├── README.md
├── LICENSE
├── src/
│   ├── main.rs           # CLI entry point, arg parsing
│   ├── lib.rs            # Library interface
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── set.rs        # Set wallpaper command
│   │   ├── revert.rs     # Revert to original
│   │   ├── status.rs     # Show current config
│   │   └── list.rs       # List available themes
│   ├── monitor/
│   │   ├── mod.rs
│   │   └── detect.rs     # Monitor detection via xrandr
│   ├── image/
│   │   ├── mod.rs
│   │   ├── blur.rs       # Image blurring logic
│   │   └── resize.rs     # Image resizing/compositing
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── extract.rs    # Extract gresource
│   │   ├── compile.rs    # Compile gresource
│   │   ├── css.rs        # CSS manipulation
│   │   └── xml.rs        # XML manipulation
│   ├── system/
│   │   ├── mod.rs
│   │   ├── backup.rs     # Backup management
│   │   └── install.rs    # Install theme
│   ├── config.rs         # Config file handling
│   ├── error.rs          # Custom error types
│   └── utils.rs          # Helper functions
└── tests/
    ├── integration.rs
    └── unit/
```

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
anyhow = "1"
thiserror = "1"
image = "0.24"           # Image processing (blur, resize)
quick-xml = "0.31"       # XML manipulation
colored = "2"            # Terminal colors
indicatif = "0.18.3"
serde = { version = "1.0.228", features = ["derive"] }
walkdir = "2.5.0"
```

## Requirements

- Arch Linux (or similar with GDM + GNOME Shell)
- ImageMagick (for image processing)
- xrandr (for monitor detection)
- gresource tools (glib2)
