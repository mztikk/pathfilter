#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::PathFilter;
use std::path::Path;

/// A filter that matches files based on a regex
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegexFilter {
    #[cfg_attr(feature = "serde", serde(with = "serde_regex"))]
    regex: regex::Regex,
}

impl PathFilter for RegexFilter {
    fn ignore(&self, path: &Path) -> bool {
        match path.to_str() {
            Some(s) => self.regex.is_match(s),
            None => false,
        }
    }
}

impl RegexFilter {
    /// Creates a new regex filter for a string containing a regex.
    /// The regex is compiled and used to match paths.
    /// # Examples
    /// ```
    /// use pathfilter::regex::RegexFilter;
    /// use pathfilter::PathFilter;
    /// use std::path::Path;
    ///
    /// let filter = RegexFilter::new_str("^src/lib.rs$").unwrap();
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(!filter.ignore(Path::new("src/main.rs")));
    ///
    /// ```
    /// # Errors
    /// If the regex is invalid, an error is returned.
    pub fn new_str(pattern: &str) -> Result<Self, regex::Error> {
        let regex = regex::Regex::new(pattern)?;
        Ok(RegexFilter { regex })
    }

    /// Creates a new regex filter for a regex.
    ///
    /// # Examples
    /// ```
    /// use pathfilter::regex::RegexFilter;
    /// use pathfilter::PathFilter;
    /// use std::path::Path;
    /// use regex::Regex;
    ///
    /// let filter = RegexFilter::new(Regex::new("^src/lib.rs$").unwrap());
    /// assert!(filter.ignore(Path::new("src/lib.rs")));
    /// assert!(!filter.ignore(Path::new("src/Program.cs")));
    ///
    /// ```
    pub fn new(regex: regex::Regex) -> Self {
        RegexFilter { regex }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::path::Path;

    #[test]
    fn regex_filter_str() {
        use crate::{regex::RegexFilter, PathFilter};

        let filter = RegexFilter::new_str("^(.*)\\.rs$").unwrap();
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }

    #[test]
    fn regex_filter() {
        use crate::{regex::RegexFilter, PathFilter};

        let filter = RegexFilter::new(Regex::new("^src/lib.rs$").unwrap());
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }
}
