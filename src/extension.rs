use crate::IgnorePath;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ffi::OsString, path::Path};

/// A filter that matches paths based on their extension.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExtensionFilter {
    extension: OsString,
}

impl IgnorePath for ExtensionFilter {
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref()
            .extension()
            .map_or(false, |ext| ext == self.extension)
    }
}

impl ExtensionFilter {
    /// Creates a new extension filter for a string containing an extension.
    ///
    /// # Examples
    /// ```
    /// use pathfilter::ExtensionFilter;
    /// use pathfilter::IgnorePath;
    /// use std::path::Path;
    ///
    /// let filter = ExtensionFilter::new(".rs");
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(filter.ignore(Path::new("src/main.rs")));
    /// assert!(!filter.ignore(Path::new("src/main.txt")));
    ///
    /// ```
    pub fn new<S: AsRef<str>>(extension: S) -> Self {
        ExtensionFilter {
            extension: extension.as_ref().trim_start_matches('.').into(),
        }
    }
}

/// A filter that matches paths based on their extension. Supports multiple extensions.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExtensionsFilter {
    extensions: HashSet<OsString>,
}

impl IgnorePath for ExtensionsFilter {
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref()
            .extension()
            .map_or(false, |ext| self.extensions.contains(ext))
    }
}

impl ExtensionsFilter {
    /// Creates a new extensions filter for a list of extensions.
    ///
    /// # Examples
    /// ```
    /// use pathfilter::ExtensionsFilter;
    /// use pathfilter::IgnorePath;
    /// use std::path::Path;
    ///
    /// let filter = ExtensionsFilter::new(&vec![".rs", ".txt"]);
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(filter.ignore(Path::new("src/main.rs")));
    /// assert!(filter.ignore(Path::new("src/main.txt")));
    /// assert!(!filter.ignore(Path::new("src/main.png")));
    ///
    /// ```
    pub fn new<S, T>(extensions: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<[S]>,
    {
        ExtensionsFilter {
            extensions: extensions
                .as_ref()
                .iter()
                .map(|ext| ext.as_ref().trim_start_matches('.').to_string().into())
                .collect(),
        }
    }

    /// Adds an extension to the filter.
    pub fn with_extension(mut self, extension: &str) -> Self {
        self.extensions
            .insert(extension.trim_start_matches('.').into());
        self
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn extension_filter() {
        use crate::{extension::ExtensionFilter, IgnorePath};

        let filter = ExtensionFilter::new(".rs");
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }

    #[test]
    fn extensions_filter() {
        use crate::{extension::ExtensionsFilter, IgnorePath};

        let filter = ExtensionsFilter::new(&[".rs", ".txt"]);
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(filter.ignore(Path::new("src/main.txt")));
        assert!(!filter.ignore(Path::new("src/main.png")));
    }
}
