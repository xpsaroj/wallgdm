use std::{fs, path::Path};

use crate::error::ThemeError;

const BACKGROUND_RULE: &str = r#"
#lockDialogGroup {
    background: url("background.png");
    background-size: cover;
    background-repeat: no-repeat;
}
"#;

pub fn update_theme_css(workdir: &Path) -> Result<(), ThemeError> {
    let css_paths =
        ["theme/gnome-shell-dark.css", "theme/gnome-shell-light.css"];

    let css_paths = css_paths.map(|p| workdir.join(p));
    let mut modified_any = false;

    for css_path in css_paths {
        if css_path.exists() {
            update_single_file(&css_path)?;
            modified_any = true;
        }
    }

    if !modified_any {
        return Err(ThemeError::CssModificationFailed);
    }

    Ok(())
}

fn update_single_file(file_path: &Path) -> Result<(), ThemeError> {
    let css = fs::read_to_string(&file_path)
        .map_err(|_| ThemeError::CssModificationFailed)?;

    let updated_css =
        replace_or_append_block(&css, "#lockDialogGroup", BACKGROUND_RULE);

    fs::write(&file_path, updated_css)
        .map_err(|_| ThemeError::CssModificationFailed)?;

    Ok(())
}

fn replace_or_append_block(
    css: &str,
    selector: &str,
    new_block: &str,
) -> String {
    let bytes = css.as_bytes();
    let sel = selector.as_bytes();
    let mut i = 0;

    while i + sel.len() < bytes.len() {
        // match selector exactly
        if &bytes[i..i + sel.len()] == sel {
            let mut j = i + sel.len();

            // skip whitespace
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }

            // must be `{` to be a real rule
            if j < bytes.len() && bytes[j] == b'{' {
                // found correct block start
                let mut depth = 0;
                let open = j;

                for k in open..bytes.len() {
                    match bytes[k] {
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                let end = k + 1;
                                let mut result = String::new();
                                result.push_str(&css[..i]);
                                result.push_str(new_block);
                                result.push_str(&css[end..]);
                                return result;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1;
    }

    // selector not found → append
    format!("{css}\n\n{new_block}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_existing_block() {
        let css = r#"
#lockDialogGroup {
    background-color: #000000;
}
"#;

        let updated =
            replace_or_append_block(css, "#lockDialogGroup", "new_block");

        assert!(updated.contains("new_block"));
        assert!(!updated.contains("background-color"));
    }

    #[test]
    fn appends_if_missing() {
        let css = "body { color: white; }";
        let updated =
            replace_or_append_block(css, "#lockDialogGroup", "new_block");

        assert!(updated.ends_with("new_block"));
    }

    #[test]
    fn ignores_before_selector_and_replaces_real_block() {
        let css = r#"
#lockDialogGroup:before {
  content: "";
}

#lockDialogGroup{
  background: red;
}
"#;

        let updated =
            replace_or_append_block(css, "#lockDialogGroup", "NEW_BLOCK");

        assert!(updated.contains("NEW_BLOCK"));
        assert!(updated.contains("#lockDialogGroup:before"));
        assert!(!updated.contains("background: red"));
    }
}
