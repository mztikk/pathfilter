use std::path::Path;

#[cfg(feature = "regex")]
pub mod regex;
pub mod extension;

pub trait PathFilter {
    fn ignore(&self, path: &Path) -> bool;
}
