use crate::PathFilter;
use std::path::Path;

pub struct RegexFilter {
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
    pub fn new_str(pattern: &str) -> Result<Self, regex::Error> {
        let regex = regex::Regex::new(pattern)?;
        Ok(RegexFilter { regex })
    }

    pub fn new(regex: regex::Regex) -> Self {
        RegexFilter { regex }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[cfg(feature = "regex")]
    #[test]
    fn regex_filter() {
        use crate::{regex::RegexFilter, PathFilter};

        let filter = RegexFilter::new_str("^(.*)\\.rs$").unwrap();
        assert!(filter.ignore(Path::new("src/lib.rs")));
        assert!(filter.ignore(Path::new("src/main.rs")));
        assert!(!filter.ignore(Path::new("src/Program.cs")));
    }
}
