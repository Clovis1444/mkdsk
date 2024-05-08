//! # settings
//! Contains all project wide global constants.

/// Entry source must be provided at this position.
pub const ENTRY_SOURCE_INDEX: usize = 0;

/// Default file name when entry type is Link.
pub const DEFAULT_LINK_NAME: &str = "link";

/// Current version of the program.
pub const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Link to the program's github page.
pub const PROGRAM_GITHUB: &str = "https://github.com/Clovis1444/mkdsk";
