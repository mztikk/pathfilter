use crate::IgnorePath;
use std::{path::Path, str::FromStr};

/// A filter that matches files based on a regex
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RegexFilter {
    #[cfg_attr(feature = "serde", serde(with = "serde_regex"))]
    regex: regex::Regex,
}

impl IgnorePath for RegexFilter {
    fn ignore<P: AsRef<Path>>(&self, path: P) -> bool {
        match path.as_ref().to_str() {
            Some(s) => self.regex.is_match(s),
            None => false,
        }
    }
}

impl FromStr for RegexFilter {
    type Err = regex::Error;

    /// Attempts to parse a string into a regular expression
    fn from_str(s: &str) -> Result<Self, regex::Error> {
        Ok(RegexFilter::new(regex::Regex::new(s)?))
    }
}

impl RegexFilter {
    /// Creates a new regex filter for a string containing a regex.
    /// The regex is compiled and used to match paths.
    /// # Examples
    /// ```
    /// use pathfilter::RegexFilter;
    /// use pathfilter::IgnorePath;
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
    /// use pathfilter::RegexFilter;
    /// use pathfilter::IgnorePath;
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
        use crate::{regex::RegexFilter, IgnorePath};

        let filter = RegexFilter::new_str("^(.*)\\.rs$").unwrap();
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }

    #[test]
    fn regex_filter() {
        use crate::{regex::RegexFilter, IgnorePath};

        let filter = RegexFilter::new(Regex::new("^src/lib.rs$").unwrap());
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }
}
