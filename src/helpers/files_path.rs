use std::fmt::Display;

use crate::constants::ROOT_FOLDER;

pub fn files_path<S: AsRef<str> + Display>(filename: S) -> String {
    format!("{}/{}", *ROOT_FOLDER, filename)
}

#[cfg(test)]
mod tests {
    use crate::constants::ROOT_FOLDER;

    use super::files_path;

    #[test]
    fn filepath_is_correct() {
        let result = format!("{}/{}", *ROOT_FOLDER, "filename.rs");
        assert_eq!(files_path("filename.rs"), result);
        assert_eq!(files_path("filename.rs".to_string()), result);
    }
}
