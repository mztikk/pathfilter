use crate::PathFilter;
use std::path::Path;

/// A filter that matches files based on their extension.
pub struct ExtensionFilter {
    extension: String,
}

impl PathFilter for ExtensionFilter {
    fn ignore(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| ext.to_string_lossy())
            .map_or(false, |ext| ext == self.extension)
    }
}

impl ExtensionFilter {
    /// Creates a new extension filter for a string containing an extension.
    ///
    /// # Examples
    /// ```
    /// use pathfilter::extension::ExtensionFilter;
    /// use pathfilter::PathFilter;
    /// use std::path::Path;
    ///
    /// let filter = ExtensionFilter::new(".rs");
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(filter.ignore(Path::new("src/main.rs")));
    /// assert!(!filter.ignore(Path::new("src/main.txt")));
    ///
    /// ```
    pub fn new(extension: &str) -> Self {
        ExtensionFilter {
            extension: extension.trim_start_matches('.').to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn extension_filter() {
        use crate::{extension::ExtensionFilter, PathFilter};

        let filter = ExtensionFilter::new(".rs");
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }
}
