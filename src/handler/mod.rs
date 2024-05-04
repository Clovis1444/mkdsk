//! # handler
//! This module contains functions that handles all arguments provided by the user.

mod arg_options;

use crate::shortcut::Shortcut;
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::exit};

pub use arg_options::validate_option;
use path_absolutize::Absolutize;

/// Prints help message. This function is called if `-h` or `--help` argument was provided, or if no arguments were provided.
/// Program should be closed after calling this function.
pub fn print_help() {
    let tab = "    ";

    println!("usage: mkdsk [path/to/exec] [options]...\n");

    println!("options:");
    println!("{tab}{:<30} display this help and exit", "-h, --help");
    println!(
        "{tab}{:<20}{:<10} set output directory. Current directory is used by default",
        "-o, --out", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set application name. Executable name is used by default",
        "-n, --name", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set application generic name",
        "-g, --gname", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set additional info about the application",
        "-c, --comment", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set application icon",
        "-i, --icon", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set list of arguments to the program",
        "-a, --args", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set the working directory to run the program in",
        "-p, --path", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set whether the program runs in a terminal window",
        "-t, --terminal", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set url to access. If entry is Link type this option must be set",
        "-u, --url", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set if the application does not support having an more than one window opened",
        "-s, --single-window", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set if application will not be displayed in menus",
        "--no-display", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} if this option is true - the desktop entry will be completely ignored",
        "--hidden", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set a list of desktop environments that should display the desktop entry",
        "--only-shown-in", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set a list of desktop environments that should not display the desktop entry",
        "--not-shown-in", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set if D-Bus activation is supported for the application",
        "--d-bus", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set path to an executable file on disk used to determine if the program is actually installed",
        "--try-exec", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} this can be used to tell the application to make a specific action",
        "--actions", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set if the application prefers to be run on a more powerful discrete GPU if available",
        "--non-default-gpu", "[bool]"
    );

    println!("\nNote: if you want the application to be globally accessible use \"--out ~/.local/share/applications\"");

    println!("\nFor more info see: https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html")
}

/// Handles argument at `PATH_TO_EXEC_INDEX` position.
/// Closes program if executable cannot be found.
/// Prints help message and closes program if argument value is `-h` or `--help`.
pub fn handle_path(arg: String, shortcut: &mut Shortcut) {
    if arg == "--help" || arg == "-h" {
        print_help();
        exit(0);
    }

    let mut exec = PathBuf::from(&arg);

    let is_exec: bool;
    // TODO(clovis): add directory and link types support
    // Check if file exists
    match exec.is_file() {
        // Check if file is an executable
        true => is_exec = exec.metadata().unwrap().permissions().mode() & 0o111 != 0,
        false => is_exec = false,
    }

    if is_exec {
        // if the path is relative - transform it to absolute
        exec = exec.absolutize().unwrap().to_path_buf();

        shortcut.set_exec(exec);

        return;
    }

    println!("mkdsk: failed to find executable");
    exit(1);
}
