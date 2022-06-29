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
//! use pathfilter::extension::ExtensionFilter;
//! use pathfilter::PathFilter;
//!
//! let filter = ExtensionFilter::new(".rs");
//! assert!(filter.ignore(Path::new("src/lib.rs")));
//!
//! ```

use std::path::Path;

/// A filter that matches files based on their extension.
pub mod extension;
#[cfg(feature = "regex")]
/// A filter that matches files based on a regular expression.
pub mod regex;

/// Provides an interface filtering and ignoring paths.
pub trait PathFilter {
    /// Returns `true` if the path should be ignored.
    fn ignore(&self, path: &Path) -> bool;
}
