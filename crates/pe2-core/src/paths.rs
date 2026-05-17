use std::path::PathBuf;

pub fn local_prompts_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("pe2-prompts")
}

pub fn resolve_output_file(output_file: Option<&str>, session_id: &str) -> PathBuf {
    if let Some(file) = output_file {
        if PathBuf::from(file).is_absolute() {
            PathBuf::from(file)
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(file)
        }
    } else {
        let dir = local_prompts_dir();
        std::fs::create_dir_all(&dir).ok();
        dir.join(format!("pe2-session-{}.md", session_id))
    }
}
