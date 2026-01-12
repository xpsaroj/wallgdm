use crate::{error::ThemeError, theme::compile};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn compile_theme(gresource_xml: &Path) -> Result<PathBuf, ThemeError> {
    let parent_dir = gresource_xml
        .parent()
        .ok_or(ThemeError::ThemeCompilationFailed)?;

    let status = Command::new("glib-compile-resources")
        .current_dir(parent_dir)
        .arg(gresource_xml)
        .status()
        .map_err(|_| ThemeError::ThemeCompilationFailed)?;

    if !status.success() {
        return Err(ThemeError::ThemeCompilationFailed);
    }

    let compiled_resource = parent_dir.join("gnome-shell-theme.gresource");
    if !compiled_resource.exists() {
        return Err(ThemeError::ThemeCompilationFailed);
    }

    Ok(compiled_resource)
}
