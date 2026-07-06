use std::path::{Component, Path, PathBuf};

pub fn local_prompts_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("pe2-prompts")
}

fn rejects_traversal(path: &Path) -> bool {
    path.components()
        .any(|c| matches!(c, Component::ParentDir))
}

fn invalid_path(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, message)
}

pub fn resolve_output_file(
    output_file: Option<&str>,
    session_id: &str,
) -> Result<PathBuf, std::io::Error> {
    if let Some(file) = output_file {
        let path = if PathBuf::from(file).is_absolute() {
            PathBuf::from(file)
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(file)
        };
        if rejects_traversal(&path) {
            return Err(invalid_path("output path must not contain '..'"));
        }
        return Ok(path);
    }
    let dir = local_prompts_dir();
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(format!("pe2-session-{}.md", session_id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_parent_dir_in_output_file() {
        let err = resolve_output_file(Some("../escape.md"), "abc").unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    }
}
