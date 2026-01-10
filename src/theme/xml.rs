use crate::error::ThemeError;
use std::path::PathBuf;
use std::{fs, io::Write, path::Path};
use walkdir::WalkDir;

pub fn generate_gresource_xml(
    workdir: &Path,
) -> Result<PathBuf, ThemeError> {
    let theme_dir = workdir.join("theme");

    let mut files = Vec::new();
    for entry in WalkDir::new(&theme_dir) {
        let entry = entry.map_err(|_| {
            ThemeError::GresourceXmlGenerationFailed
        })?;

        if entry.file_type().is_file() {
            let relative_path = entry
                .path()
                .strip_prefix(&theme_dir)
                .map_err(|_| {
                    ThemeError::GresourceXmlGenerationFailed
                })?;
            files.push(relative_path.to_owned());
        }
    }

    let xml_path =
        workdir.join("gnome-shell-theme.gresource.xml");
    let mut xml_file = fs::File::create(&xml_path)
        .map_err(|_| {
            ThemeError::GresourceXmlGenerationFailed
        })?;

    writeln!(
        xml_file,
        r#"<?xml version="1.0" encoding="UTF-8"?>"#
    )
    .map_err(|_| {
        ThemeError::GresourceXmlGenerationFailed
    })?;
    writeln!(xml_file, r#"<gresources>"#).map_err(
        |_| ThemeError::GresourceXmlGenerationFailed,
    )?;
    writeln!(
        xml_file,
        r#"  <gresource prefix="/org/gnome/shell/theme">"#
    )
    .map_err(|_| {
        ThemeError::GresourceXmlGenerationFailed
    })?;

    for file in files {
        writeln!(
            xml_file,
            "    <file>{}</file>",
            file.display()
        )
        .map_err(|_| {
            ThemeError::GresourceXmlGenerationFailed
        })?;
    }

    writeln!(xml_file, "  </gresource>").map_err(|_| {
        ThemeError::GresourceXmlGenerationFailed
    })?;
    writeln!(xml_file, "</gresources>").map_err(|_| {
        ThemeError::GresourceXmlGenerationFailed
    })?;

    Ok(xml_path)
}
