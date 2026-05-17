use serde::Serialize;
use std::io::Write;
use std::path::Path;
use crate::constants;
use crate::errors::CliError;

pub fn write_json_atomic<T: Serialize>(path: &Path, data: &T) -> Result<(), CliError> {
    let tmp_path = path.with_extension(format!(
        ".tmp.{}",
        std::process::id()
    ));

    let content = serde_json::to_string_pretty(data)?;

    {
        let mut file = std::fs::File::create(&tmp_path)?;
        file.write_all(content.as_bytes())?;
        file.sync_all()?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(constants::CONFIG_FILE_MODE))?;
    }

    std::fs::rename(&tmp_path, path)?;
    Ok(())
}
