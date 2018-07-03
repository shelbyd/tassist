pub trait WindowsPaths: ToOwned {
    fn strip_windows_unc(&self) -> &Self;
    fn escape_directory_delimiters(&self) -> Self::Owned;
}

impl WindowsPaths for str {
    fn strip_windows_unc(&self) -> &Self {
        if self.starts_with("\\\\?\\") {
            &self[4..]
        } else {
            self
        }
    }

    fn escape_directory_delimiters(&self) -> String {
        self.replace("\\", "\\\\")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn strip_windows_unc() {
        assert_eq!("\\\\?\\".strip_windows_unc(), "");
        assert_eq!("\\\\?\\C:\\whatever.txt".strip_windows_unc(), "C:\\whatever.txt");
        assert_eq!("C:\\whatever.txt".strip_windows_unc(), "C:\\whatever.txt");
    }
    
    #[test]
    fn escape_directory_delimiters() {
        assert_eq!("".escape_directory_delimiters(), "");
        assert_eq!("C:\\whatever.txt".escape_directory_delimiters(), "C:\\\\whatever.txt");
    }
}