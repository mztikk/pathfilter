#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

//! An interface for and collection of filters for ignoring files and directories.
//!
//! This library provides a simple interface for creating and using filters.
//! Filters are used to ignore files and directories.
//! # Examples
//!
//! ```
//! use std::path::Path;
//! use pathfilter::ExtensionFilter;
//! use pathfilter::IgnorePath;
//!
//! let filter = ExtensionFilter::new(".rs");
//! assert!(filter.ignore(Path::new("src/lib.rs")));
//!
//! ```

mod extension;
#[cfg(feature = "regex")]
mod regex;

#[cfg(feature = "regex")]
pub use crate::regex::RegexFilter;
pub use extension::{ExtensionFilter, ExtensionsFilter};
use std::path::Path;

/// Provides an interface ignoring paths.
pub trait IgnorePath {
    /// Returns `true` if the path should be ignored.
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// An enum that represents different types of filters for ignoring paths.
pub enum PathFilter {
    /// Filter that matches based on their extension.
    Extension(ExtensionFilter),
    /// Filter that matches based on multiple extensions.
    Extensions(ExtensionsFilter),
    #[cfg(feature = "regex")]
    /// Filter that matches based on a regular expression.
    Regex(RegexFilter),
}

impl From<ExtensionFilter> for PathFilter {
    fn from(value: ExtensionFilter) -> Self {
        PathFilter::Extension(value)
    }
}

impl From<ExtensionsFilter> for PathFilter {
    fn from(value: ExtensionsFilter) -> Self {
        PathFilter::Extensions(value)
    }
}

#[cfg(feature = "regex")]
impl From<RegexFilter> for PathFilter {
    fn from(value: RegexFilter) -> Self {
        PathFilter::Regex(value)
    }
}

impl PathFilter {
    /// Creates a new `PathFilter` based on a single extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfilter::PathFilter;
    ///
    /// let filter = PathFilter::new_extension(".rs");
    /// ```
    pub fn new_extension<S: AsRef<str>>(extension: S) -> Self {
        ExtensionFilter::new(extension).into()
    }

    /// Creates a new `PathFilter` based on multiple extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfilter::PathFilter;
    ///
    /// let filter = PathFilter::new_extensions([".rs", ".txt"]);
    /// ```
    pub fn new_extensions<S, T>(extensions: T) -> Self
    where
        S: AsRef<str>,
        T: AsRef<[S]>,
    {
        ExtensionsFilter::new(extensions).into()
    }
}

#[cfg(feature = "regex")]
impl PathFilter {
    /// Creates a new `PathFilter` based on a regular expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use pathfilter::PathFilter;
    /// use regex::Regex;
    ///
    /// let regex = Regex::new("^src/lib.rs$").unwrap();
    /// let filter = PathFilter::new_regex(regex);
    /// ```
    pub fn new_regex(regex: ::regex::Regex) -> Self {
        RegexFilter::new(regex).into()
    }
}

impl IgnorePath for PathFilter {
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool {
        match self {
            PathFilter::Extension(x) => x.ignore(path),
            PathFilter::Extensions(x) => x.ignore(path),
            #[cfg(feature = "regex")]
            PathFilter::Regex(x) => x.ignore(path),
        }
    }
}

impl<T: AsRef<[PathFilter]>> IgnorePath for T {
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool {
        self.as_ref().iter().any(|filter| filter.ignore(&path))
    }
}

#[cfg(test)]
mod tests {
    use crate::PathFilter;
    use std::path::Path;

    #[cfg(feature = "regex")]
    #[test]
    fn regex_filter() {
        use crate::IgnorePath;
        use regex::Regex;

        let filter = PathFilter::new_regex(Regex::new("^src/lib.rs$").unwrap());
        assert!(matches!(filter, PathFilter::Regex(_)));
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }

    #[test]
    fn extension_filter() {
        use crate::IgnorePath;

        let filter = PathFilter::new_extension(".rs");
        assert!(matches!(filter, PathFilter::Extension(_)));
        assert!(filter.ignore(Path::new("test.rs")));
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }

    #[test]
    fn extensions_filter() {
        use crate::IgnorePath;

        let filter = PathFilter::new_extensions([".rs", ".txt"]);
        assert!(matches!(filter, PathFilter::Extensions(_)));
        assert!(filter.ignore(Path::new("test.rs")));
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(filter.ignore(Path::new("src/main.txt")));
        assert!(!filter.ignore(Path::new("src/main.png")));
    }

    #[cfg(feature = "regex")]
    #[test]
    fn regex_extension_combined_filter() {
        use crate::IgnorePath;
        use regex::Regex;

        let filters = vec![
            PathFilter::new_regex(Regex::new("^src/lib.rs$").unwrap()),
            PathFilter::new_extension(".cs"),
        ];
        assert!(filters.ignore(Path::new("src/lib.rs")));
        assert!(!filters.ignore(Path::new("src/main.cpp")));
        assert!(filters.ignore(Path::new("test.cs")));
        assert!(filters.ignore(Path::new("src/lib.rs")));
        assert!(!filters.ignore(Path::new("src/main.rs")));
        assert!(filters.ignore(Path::new("src/Program.cs")));
    }
}
