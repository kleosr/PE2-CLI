use crate::errors::CliError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

fn write_bytes_atomic(path: &Path, bytes: &[u8], restrict: bool) -> Result<(), CliError> {
    let temp = path.with_extension(format!(".tmp.{}", std::process::id()));
    {
        let mut file = std::fs::File::create(&temp)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }
    apply_mode(&temp, restrict)?;
    std::fs::rename(&temp, path)?;
    Ok(())
}

fn apply_mode(path: &Path, restrict: bool) -> Result<(), CliError> {
    #[cfg(unix)]
    if restrict {
        use crate::constants;
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            path,
            std::fs::Permissions::from_mode(constants::CONFIG_FILE_MODE),
        )?;
    }
    #[cfg(not(unix))]
    let _ = (path, restrict);
    Ok(())
}

pub fn write_text_atomic(path: &Path, contents: &str) -> Result<(), CliError> {
    write_bytes_atomic(path, contents.as_bytes(), false)
}

pub fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), CliError> {
    write_bytes_atomic(path, serde_json::to_string_pretty(value)?.as_bytes(), true)
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, CliError> {
    let text = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn read_json_or_default<T: Default + DeserializeOwned>(path: &Path) -> T {
    if !path.exists() {
        return T::default();
    }
    read_json(path).unwrap_or_else(|error| {
        tracing::warn!("failed to load {}: {error}; using defaults", path.display());
        T::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
    struct Sample {
        count: u32,
    }

    #[test]
    fn read_json_or_default_missing_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let sample: Sample = read_json_or_default(&dir.path().join("missing.json"));
        assert_eq!(sample.count, 0);
    }

    #[test]
    fn read_json_or_default_recovers_from_corrupt_file() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "not-json").unwrap();
        let sample: Sample = read_json_or_default(&path);
        assert_eq!(sample.count, 0);
    }

    #[test]
    fn read_json_round_trip() {
        let dir = tempfile::TempDir::new().unwrap();
        let path = dir.path().join("sample.json");
        write_json_atomic(&path, &Sample { count: 3 }).unwrap();
        let sample: Sample = read_json(&path).unwrap();
        assert_eq!(sample.count, 3);
    }
}
