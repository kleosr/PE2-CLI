#[cfg(unix)]
use crate::constants;
use crate::errors::CliError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

fn write_bytes_atomic(
    path: &Path,
    bytes: &[u8],
    #[cfg_attr(not(unix), allow(unused_variables))] restrict_permissions: bool,
) -> Result<(), CliError> {
    let tmp_path = path.with_extension(format!(".tmp.{}", std::process::id()));
    {
        let mut file = std::fs::File::create(&tmp_path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
    }

    #[cfg(unix)]
    if restrict_permissions {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            &tmp_path,
            std::fs::Permissions::from_mode(constants::CONFIG_FILE_MODE),
        )?;
    }

    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

pub fn write_text_atomic(path: &Path, content: &str) -> Result<(), CliError> {
    write_bytes_atomic(path, content.as_bytes(), false)
}

pub fn write_json_atomic<T: Serialize>(path: &Path, data: &T) -> Result<(), CliError> {
    let content = serde_json::to_string_pretty(data)?;
    write_bytes_atomic(path, content.as_bytes(), true)
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, CliError> {
    let content = std::fs::read_to_string(path).map_err(CliError::Io)?;
    serde_json::from_str(&content).map_err(CliError::Json)
}

pub fn read_json_or_default<T: Default + DeserializeOwned>(path: &Path) -> T {
    if !path.exists() {
        return T::default();
    }
    match read_json(path) {
        Ok(value) => value,
        Err(e) => {
            tracing::warn!("failed to load {}: {e}; using defaults", path.display());
            T::default()
        }
    }
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
        let path = dir.path().join("missing.json");
        let sample: Sample = read_json_or_default(&path);
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
        let loaded: Sample = read_json(&path).unwrap();
        assert_eq!(loaded.count, 3);
    }
}
