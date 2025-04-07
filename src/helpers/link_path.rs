use std::fmt::Display;

use crate::constants::FEATURES;

pub fn link_path<S: AsRef<str> + Display>(path: S) -> String {
    if FEATURES.contains(&"custom_client") {
        if path.to_string() == *"/" {
            return "/legacy".to_string();
        }
        return format!("/legacy{}", path);
    }
    path.to_string()
}
