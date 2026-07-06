use crate::constants;

pub fn mask_api_key(key: Option<&str>) -> String {
    match key {
        Some(k) if k.len() > constants::SHORT_API_KEY_THRESHOLD => {
            let suffix_start = k.len().saturating_sub(constants::SHORT_API_KEY_SUFFIX);
            format!(
                "{}...{}",
                &k[..constants::SHORT_API_KEY_PREFIX],
                &k[suffix_start..]
            )
        }
        Some(k) if !k.is_empty() => "**** (short key)".to_string(),
        _ => "not set".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_api_key_none() {
        assert_eq!(mask_api_key(None), "not set");
    }

    #[test]
    fn mask_api_key_empty() {
        assert_eq!(mask_api_key(Some("")), "not set");
    }

    #[test]
    fn mask_api_key_short() {
        assert_eq!(mask_api_key(Some("abc")), "**** (short key)");
    }

    #[test]
    fn mask_api_key_long_shows_prefix_and_suffix() {
        let masked = mask_api_key(Some("sk-abcdefghijklmnop"));
        assert!(masked.starts_with("sk-a"));
        assert!(masked.contains("..."));
        assert!(masked.ends_with("mnop"));
    }

    #[test]
    fn mask_api_key_threshold_boundary() {
        let twelve = "a".repeat(12);
        assert_eq!(mask_api_key(Some(&twelve)), "**** (short key)");
        let thirteen = "a".repeat(13);
        let masked = mask_api_key(Some(&thirteen));
        assert!(masked.contains("..."));
        assert!(!masked.contains("short key"));
    }
}
