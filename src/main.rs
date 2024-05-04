// main.rs

mod handler;
mod shortcut;

use std::process::exit;

use handler::*;
use shortcut::Shortcut;

// TODO(clovis): create "settings" mod to store global variables
/// Entry source must be provided at this position.
const ENTRY_SOURCE_INDEX: usize = 0;

fn main() {
    let mut args = std::env::args();
    args.next();

    if args.len() == 0 {
        print_help();
        return;
    }

    let mut shortcut = Shortcut::new();

    let mut iter = args.enumerate();
    while let Some((i, arg)) = iter.next() {
        if i == ENTRY_SOURCE_INDEX {
            validate_source(arg, &mut shortcut);
        } else {
            let (_, option_val) = match iter.next() {
                Some(val) => val,
                None => {
                    println!("mkdsk: missing option value");
                    exit(4)
                }
            };
            validate_option(arg.clone(), option_val, &mut shortcut);
        }
    }

    shortcut.create();
}
