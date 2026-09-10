#[cfg(unix)]
use crate::constants;
use crate::errors::CliError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

fn write_bytes_atomic(
    p: &Path,
    b: &[u8],
    #[cfg_attr(not(unix), allow(unused_variables))] r: bool,
) -> Result<(), CliError> {
    let t = p.with_extension(format!(".tmp.{}", std::process::id()));
    {
        let mut f = std::fs::File::create(&t)?;
        f.write_all(b)?;
        f.sync_all()?;
    }
    #[cfg(unix)]
    if r {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(
            &t,
            std::fs::Permissions::from_mode(constants::CONFIG_FILE_MODE),
        )?;
    }
    std::fs::rename(&t, p)?;
    Ok(())
}
pub fn write_text_atomic(p: &Path, c: &str) -> Result<(), CliError> {
    write_bytes_atomic(p, c.as_bytes(), false)
}
pub fn write_json_atomic<T: Serialize>(p: &Path, d: &T) -> Result<(), CliError> {
    write_bytes_atomic(p, serde_json::to_string_pretty(d)?.as_bytes(), true)
}
pub fn read_json<T: DeserializeOwned>(p: &Path) -> Result<T, CliError> {
    serde_json::from_str(&std::fs::read_to_string(p).map_err(CliError::Io)?).map_err(CliError::Json)
}
pub fn read_json_or_default<T: Default + DeserializeOwned>(p: &Path) -> T {
    if !p.exists() {
        return T::default();
    }
    read_json(p).unwrap_or_else(|e| {
        tracing::warn!("failed to load {}: {e}; using defaults", p.display());
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
        let d = tempfile::TempDir::new().unwrap();
        let s: Sample = read_json_or_default(&d.path().join("missing.json"));
        assert_eq!(s.count, 0);
    }
    #[test]
    fn read_json_or_default_recovers_from_corrupt_file() {
        let d = tempfile::TempDir::new().unwrap();
        let q = d.path().join("bad.json");
        std::fs::write(&q, "not-json").unwrap();
        let s: Sample = read_json_or_default(&q);
        assert_eq!(s.count, 0);
    }
    #[test]
    fn read_json_round_trip() {
        let d = tempfile::TempDir::new().unwrap();
        let q = d.path().join("sample.json");
        write_json_atomic(&q, &Sample { count: 3 }).unwrap();
        let s: Sample = read_json(&q).unwrap();
        assert_eq!(s.count, 3);
    }
}
