//! # handler
//! This module contains functions that handles all arguments provided by the user.

mod arg_options;

use crate::settings::*;
use crate::shortcut::{EntryType, Shortcut};
use std::{env::current_dir, os::unix::fs::PermissionsExt, path::PathBuf, process::exit};

pub use arg_options::validate_option;
use is_url::is_url;
use path_absolutize::Absolutize;

/// Prints help message. This function is called if `-h` or `--help` argument was provided, or if no arguments were provided.
/// Program should be closed after calling this function.
pub fn print_help() {
    let tab = "    ";

    println!("usage: mkdsk [source] [options]...\n");

    println!("sources: ");
    println!("{tab}{:<15}[path/to/exec]", "Application");
    println!("{tab}{:<15}[path/to/dir]", "Directory");
    println!("{tab}{:<15}[url]", "Link");
    print!("\n");
    println!("");

    println!("options:");
    println!("{tab}{:<30} display this help and exit", "-h, --help");
    println!("{tab}{:<30} display program version", "-v, --version");
    println!(
        "{tab}{:<20}{:<10} set output directory. Current directory is used by default",
        "-o, --out", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set application name. Executable/dir name is used by default; if entry is a Link name \"link\" is used by default",
        "-n, --name", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set application generic name",
        "-g, --gname", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set if application will not be displayed in menus",
        "--no-display", "[bool]"
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
        "{tab}{:<20}{:<10} this can be used to tell the application to make a specific action",
        "--actions", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set the MIME type(s) supported by the application",
        "-m, --mime-type", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set categories in which the entry should be shown in a menu",
        "--categories", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set list of interfaces that this application implements",
        "--implements", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set list of strings which may be used in addition to other metadata to describe this entry",
        "--keywords", "[str]"
    );
    println!(
        "{tab}{:<20}{:<10} set if it is KNOWN that the application will send a \"remove\" message when started on startup",
        "--startup-notify", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set if it is known that the application will map at least one window with the given string as its WM class or WM name hint",
        "--startup-wm-class", "[str]"
    );
    // println!(
    //     "{tab}{:<20}{:<10} set url to access. If entry type is Link this option must be set",
    //     "-u, --url", "[str]"
    // );
    println!(
        "{tab}{:<20}{:<10} set if the application prefers to be run on a more powerful discrete GPU if available",
        "--non-default-gpu", "[bool]"
    );
    println!(
        "{tab}{:<20}{:<10} set if the application does not support having an more than one window opened",
        "-s, --single-window", "[bool]"
    );

    println!("\nNote: if you want the application to be globally accessible use \"--out ~/.local/share/applications\"");

    println!("\nFor more info see: https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html")
}

fn print_version() {
    println!("mkdsk version {PROGRAM_VERSION}");

    println!("\nFor more info see: {PROGRAM_GITHUB}");
}

/// Validates argument at `ENTRY_SOURCE_INDEX` position.
/// Closes program if source is not a url/dir/executable.
/// Prints help message and closes program if argument value is `-h` or `--help`.
/// Prints program version and closes program if argument value is `-v` or `--version`.
///
/// This function also sets default output path and default name.
pub fn validate_source(arg: String, shortcut: &mut Shortcut) {
    // Handle help arg
    if arg == "--help" || arg == "-h" {
        print_help();
        exit(0);
    }
    match arg.as_str() {
        // Help arg
        "--help" | "-h" => {
            print_help();
            exit(0);
        }
        // Version arg
        "--version" | "-v" => {
            print_version();
            exit(0);
        }
        _ => (),
    }

    let exec = PathBuf::from(&arg);

    // Set default output dir
    shortcut.set_out(current_dir().unwrap());

    if is_url(&arg) {
        shortcut.set_entry_type(EntryType::Link);

        shortcut.set_url(arg.clone());
        shortcut.set_name(DEFAULT_LINK_NAME.to_string());

        return;
    } else if exec.is_dir() {
        shortcut.set_entry_type(EntryType::Directory);

        shortcut.set_exec(exec.absolutize().unwrap().to_path_buf());

        shortcut.set_name(String::from(exec.file_name().unwrap().to_str().unwrap()));

        return;
    } else if exec.is_file() {
        // Check if file is an executable
        if exec.metadata().unwrap().permissions().mode() & 0o111 != 0 {
            shortcut.set_entry_type(EntryType::Application);

            shortcut.set_exec(exec.absolutize().unwrap().to_path_buf());

            shortcut.set_name(String::from(exec.file_name().unwrap().to_str().unwrap()));
        }

        return;
    } else if let Ok(_) = which::which(&arg) {
        shortcut.set_entry_type(EntryType::Application);

        shortcut.set_exec(PathBuf::from(&arg));

        shortcut.set_name(arg);

        return;
    }

    println!("mkdsk: invalid source");
    exit(1);
}
