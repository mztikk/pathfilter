use crate::PathFilter;
use std::{collections::HashSet, path::Path};

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

/// A filter that matches files based on their extension. Supports multiple extensions.
pub struct ExtensionsFilter {
    extensions: HashSet<String>,
}

impl PathFilter for ExtensionsFilter {
    fn ignore(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| ext.to_string_lossy())
            .map_or(false, |ext| self.extensions.contains(ext.as_ref()))
    }
}

impl ExtensionsFilter {
    /// Creates a new extensions filter for a list of extensions.
    ///
    /// # Examples
    /// ```
    /// use pathfilter::extension::ExtensionsFilter;
    /// use pathfilter::PathFilter;
    /// use std::path::Path;
    ///
    /// let filter = ExtensionsFilter::new(&vec![".rs", ".txt"]);
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(filter.ignore(Path::new("src/main.rs")));
    /// assert!(filter.ignore(Path::new("src/main.txt")));
    /// assert!(!filter.ignore(Path::new("src/main.png")));
    ///
    /// ```
    pub fn new(extensions: &[&str]) -> Self {
        ExtensionsFilter {
            extensions: extensions
                .iter()
                .map(|ext| ext.trim_start_matches('.').to_string())
                .collect(),
        }
    }

    /// Adds an extension to the filter.
    pub fn with_extension(mut self, extension: &str) -> Self {
        self.extensions
            .insert(extension.trim_start_matches('.').to_string());
        self
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

    #[test]
    fn extensions_filter() {
        use crate::{extension::ExtensionsFilter, PathFilter};

        let filter = ExtensionsFilter::new(&[".rs", ".txt"]);
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(filter.ignore(Path::new("src/main.txt")));
        assert!(!filter.ignore(Path::new("src/main.png")));
    }
}
