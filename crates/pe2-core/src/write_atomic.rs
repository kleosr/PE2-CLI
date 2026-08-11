use crate::constants;
use crate::errors::CliError;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

fn write_bytes_atomic(
    path: &Path,
    bytes: &[u8],
    restrict_permissions: bool,
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
