// handler.rs

mod arg_options;

use crate::shortcut::Shortcut;
use std::{os::unix::fs::PermissionsExt, path::PathBuf, process::exit};

pub use arg_options::validate_option;
use path_absolutize::Absolutize;

pub fn print_help() {
    let tab = "    ";

    println!("usage: mkdsk [path/to/exec] [options]...\n");

    println!("options:");
    println!("{tab}{:<20} display this help and exit", "-h, --help");
    println!(
        "{tab}{:<20} set output directory. Current directory is used by default",
        "-o, --out"
    );
    println!(
        "{tab}{:<20} set application name. Executable name is used by default",
        "-n, --name"
    );
    println!(
        "{tab}{:<20} set additional info about the application",
        "-c, --comment"
    );
    println!("{tab}{:<20} set application icon", "-i, --icon");
    println!("{tab}{:<20} set executable arguments", "-a, --args");

    println!("\nNote: if you want the application to be globally accessible use \"--out ~/.local/share/applications\"")
}

pub fn handle_path(arg: String, shortcut: &mut Shortcut) {
    if arg == "--help" || arg == "-h" {
        print_help();
        exit(0);
    }

    let mut exec = PathBuf::from(&arg);

    let is_exec: bool;
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
