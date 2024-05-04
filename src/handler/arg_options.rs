//! # arg_options
//! This module contains all possible arguments.
//!
//! # Add new argument
//! To add new argument do the following:
//! 1. Create argument handler function.
//! 2. Create new `ArgOption` entry in `OPTION_LIST`. Pass **arg name** and **arg handler** to your `ArgOption` structure.
//! 3. Run `cargo test` to ensure that there are no options with the same argument name.

use std::{path::PathBuf, process::exit, str::FromStr};

use path_absolutize::Absolutize;

use crate::Shortcut;

/// Structure that contain **argument name** and **argument handler** function pointer.
#[derive(Debug)]
struct ArgOption<'a> {
    option: &'a str,
    handler: fn(&str, &mut Shortcut),
}

/// Cointans all arguments data.
const OPTION_LIST: [ArgOption; 28] = [
    ArgOption {
        option: "-o",
        handler: out_handler,
    },
    ArgOption {
        option: "--out",
        handler: out_handler,
    },
    ArgOption {
        option: "-n",
        handler: name_handler,
    },
    ArgOption {
        option: "--name",
        handler: name_handler,
    },
    ArgOption {
        option: "-c",
        handler: comment_handler,
    },
    ArgOption {
        option: "--comment",
        handler: comment_handler,
    },
    ArgOption {
        option: "-i",
        handler: icon_handler,
    },
    ArgOption {
        option: "--icon",
        handler: icon_handler,
    },
    ArgOption {
        option: "-a",
        handler: args_handler,
    },
    ArgOption {
        option: "--args",
        handler: args_handler,
    },
    ArgOption {
        option: "-g",
        handler: gname_handler,
    },
    ArgOption {
        option: "--gname",
        handler: gname_handler,
    },
    ArgOption {
        option: "--no-display",
        handler: no_display_handler,
    },
    ArgOption {
        option: "--hidden",
        handler: hidden_handler,
    },
    ArgOption {
        option: "--only-shown-in",
        handler: only_shown_handler,
    },
    ArgOption {
        option: "--not-shown-in",
        handler: not_shown_handler,
    },
    ArgOption {
        option: "--d-bus",
        handler: d_bus_handler,
    },
    ArgOption {
        option: "--try-exec",
        handler: try_exec_handler,
    },
    ArgOption {
        option: "-p",
        handler: path_handler,
    },
    ArgOption {
        option: "--path",
        handler: path_handler,
    },
    ArgOption {
        option: "-t",
        handler: terminal_handler,
    },
    ArgOption {
        option: "--terminal",
        handler: terminal_handler,
    },
    ArgOption {
        option: "--actions",
        handler: actions_handler,
    },
    ArgOption {
        option: "-u",
        handler: url_handler,
    },
    ArgOption {
        option: "--url",
        handler: url_handler,
    },
    ArgOption {
        option: "--non-default-gpu",
        handler: non_default_gpu_handler,
    },
    ArgOption {
        option: "-s",
        handler: single_main_window_handler,
    },
    ArgOption {
        option: "--single-window",
        handler: single_main_window_handler,
    },
];

/// Function that call argument handler for all arguments, except arguments at `PATH_TO_EXEC_INDEX` position.
pub fn validate_option(option: String, option_val: String, shortcut: &mut Shortcut) {
    for i in OPTION_LIST {
        if i.option == option {
            (i.handler)(&option_val, shortcut);
            return;
        }
    }

    println!("mkdsk: invalid option {option}");
    exit(2);
}

//
//
//
fn out_handler(value: &str, shortcut: &mut Shortcut) {
    let mut out = PathBuf::from(value);

    if out.is_dir() {
        // if the path is relative - transform it to absolute
        out = out.absolutize().unwrap().to_path_buf();

        shortcut.set_out(out);
        return;
    }

    println!("mkdsk: invalid output directory {}", out.to_str().unwrap());
    exit(3);
}
fn name_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_name(value.to_string());
}
fn comment_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_comment(value.to_string());
}
fn icon_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_icon(value.to_string());
}
fn args_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_args(value.to_string());
}
fn gname_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_generic_name(value.to_string());
}
fn no_display_handler(value: &str, shortcut: &mut Shortcut) {
    let no_display = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(7)
        }
    };

    shortcut.set_no_display(no_display);
}
fn hidden_handler(value: &str, shortcut: &mut Shortcut) {
    let hidden = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(8)
        }
    };

    shortcut.set_hidden(hidden);
}
fn only_shown_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_only_shown_in(value.to_string());
}
fn not_shown_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_not_shown_in(value.to_string());
}
fn d_bus_handler(value: &str, shortcut: &mut Shortcut) {
    let d_bus = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(9)
        }
    };

    shortcut.set_d_bus_activatable(d_bus);
}
fn try_exec_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_try_exec(value.to_string());
}
fn path_handler(value: &str, shortcut: &mut Shortcut) {
    let path = match PathBuf::from_str(value) {
        Ok(val) => val,
        Err(_) => {
            println!("mkdsk: invalid path");
            exit(10)
        }
    };

    if let false = path.is_dir() {
        println!("mkdsk: path is not exists on disk or is not a directory");
        exit(10)
    }

    shortcut.set_path(value.to_string());
}
fn terminal_handler(value: &str, shortcut: &mut Shortcut) {
    let use_terminal = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(11)
        }
    };

    shortcut.set_terminal(use_terminal);
}
fn actions_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_actions(value.to_string());
}
fn url_handler(value: &str, shortcut: &mut Shortcut) {
    shortcut.set_url(value.to_string());
}
fn non_default_gpu_handler(value: &str, shortcut: &mut Shortcut) {
    let non_default_gpu = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(13)
        }
    };

    shortcut.set_prefers_non_default_gpu(non_default_gpu);
}
fn single_main_window_handler(value: &str, shortcut: &mut Shortcut) {
    let single_main_window = match value.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => {
            println!("mkdsk: invalid option value");
            exit(14)
        }
    };

    shortcut.set_single_main_window(single_main_window);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arg_name_duplicate() {
        for i in OPTION_LIST {
            for j in OPTION_LIST {
                assert!(
                    !(i.handler != j.handler && i.option == j.option),
                    "{}",
                    format!(
                        "OPTION_LIST contains options with the same argument name: {:?}, {:?}",
                        i, j
                    )
                );
            }
        }
    }
}
