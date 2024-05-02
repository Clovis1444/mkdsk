//! # arg_options
//! This module contains all possible arguments.
//!
//! # Add new argument
//! To add new argument do the following:
//! 1. Create argument handler function.
//! 2. Create new `ArgOption` entry in `OPTION_LIST`. Pass **arg name** and **arg handler** to your `ArgOption` structure.

use std::{path::PathBuf, process::exit};

use path_absolutize::Absolutize;

use crate::Shortcut;

/// Structure that contain **argument name** and **argument handler** function pointer.
struct ArgOption<'a> {
    option: &'a str,
    handler: fn(&str, &mut Shortcut),
}

/// Cointans all arguments data.
const OPTION_LIST: [ArgOption; 10] = [
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
        option: "-args",
        handler: args_handler,
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
