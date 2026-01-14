//! GNOME Shell theme GResource XML generation for WallGDM.
//!
//! Generates a `.gresource.xml` file listing all theme files under the
//! extracted theme directory. This XML is used for compiling the theme
//! into a GResource bundle.

use crate::error::ThemeError;
use std::path::PathBuf;
use std::{fs, io::Write, path::Path};
use walkdir::WalkDir;

/// Generate `gnome-shell-theme.gresource.xml` for the theme in the working directory.
///
/// # Parameters
/// - `workdir`: The working directory where the theme files are located (`SetDirs.theme_workdir`).
///
/// # Returns
/// The path to the generated XML file, or `ThemeError::GresourceXmlGenerationFailed` if anything fails.
///
/// # Notes
/// This XML file is required for `glib-compile-resources` to build a `.gresource` binary.
pub fn generate_gresource_xml(workdir: &Path) -> Result<PathBuf, ThemeError> {
    let theme_dir = workdir.join("theme");

    // Collect all files in the theme directory
    let mut files = Vec::new();
    for entry in WalkDir::new(&theme_dir) {
        log::debug!("Found theme file: {:?}", entry);
        let entry =
            entry.map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;

        if entry.file_type().is_file() {
            let relative_path = entry
                .path()
                .strip_prefix(&theme_dir)
                .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;
            files.push(relative_path.to_owned());
        }
    }

    // Path to output XML file
    let xml_path = workdir.join("theme/gnome-shell-theme.gresource.xml");
    let mut xml_file = fs::File::create(&xml_path)
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;

    // Write XML content
    writeln!(xml_file, r#"<?xml version="1.0" encoding="UTF-8"?>"#)
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;
    writeln!(xml_file, r#"<gresources>"#)
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;
    writeln!(xml_file, r#"  <gresource prefix="/org/gnome/shell/theme">"#)
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;

    for file in files {
        writeln!(xml_file, "    <file>{}</file>", file.display())
            .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;
    }

    writeln!(xml_file, "  </gresource>")
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;
    writeln!(xml_file, "</gresources>")
        .map_err(|_| ThemeError::GresourceXmlGenerationFailed)?;

    Ok(xml_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_gresource_xml() {
        let tmp_dir = tempdir().expect("failed to create temp dir");
        let theme_dir = tmp_dir.path().join("theme");
        fs::create_dir_all(&theme_dir)
            .expect("failed to create theme directory");

        // Create some dummy files
        let file_paths = vec![
            theme_dir.join("gnome-shell-light.css"),
            theme_dir.join("gnome-shell-dark.css"),
            theme_dir.join("sth.svg"),
        ];

        for path in &file_paths {
            fs::write(path, path.to_string_lossy().as_bytes())
                .expect("failed to create dummy theme file");
        }

        let xml_path = generate_gresource_xml(tmp_dir.path())
            .expect("failed to generate gresource XML");
        let xml_content = fs::read_to_string(&xml_path)
            .expect("failed to read generated gresource XML");

        assert!(xml_content.contains("<file>gnome-shell-light.css</file>"));
        assert!(xml_content.contains("<file>gnome-shell-dark.css</file>"));
        assert!(xml_content.contains("<file>sth.svg</file>"));
    }
}
