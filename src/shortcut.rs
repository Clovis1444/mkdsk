//! # shortcut
//! This module contains `Shortcut` structure that contains information about the `.desktop` file that will be created.

use std::{env::current_dir, fs::File, io::Write, path::PathBuf, process::exit};

/// Contains information about the `.desktop` file that will be created.
#[derive(Debug)]
pub struct Shortcut {
    exec: Option<PathBuf>,
    out: Option<PathBuf>,
    args: String,
    name: String,
    comment: String,
    icon: String,
}

impl Shortcut {
    pub fn new() -> Shortcut {
        Shortcut {
            exec: None,
            out: None,
            args: String::new(),
            name: String::new(),
            comment: String::new(),
            icon: String::new(),
        }
    }

    /// Creates .desktop file, pass all `Shortcut data` to it.
    /// Will assert if any of the following fields is empty: `exec`, `out`, `name`.
    /// This function will close the program if file creation fails.
    /// Edit this function if you are plannig to add new `Shortcut` fields.
    pub fn create(&self) {
        assert!(&self.exec.is_some(), "exec must be set");
        assert!(&self.out.is_some(), "out must be set");
        assert!(!&self.name.is_empty(), "name must be set");

        let mut file_name = self.name.clone();
        file_name.push_str(".desktop");
        let file_path = self.out.clone().unwrap().join(file_name);

        let mut file = match File::create(file_path.clone()) {
            Ok(val) => val,
            Err(e) => {
                println!("mkdsk: {e}");
                exit(5)
            }
        };

        let mut text = format!("[Desktop Entry]\nType=Application\nName={}\n", self.name);
        if !self.comment.is_empty() {
            text.push_str(&format!("Comment={}\n", self.comment));
        }
        if !self.icon.is_empty() {
            text.push_str(&format!("Icon={}\n", self.icon));
        }
        let mut exec = String::from(self.exec.clone().unwrap().to_str().unwrap());
        if !self.args.is_empty() {
            exec.push_str(&self.args);
        }
        text.push_str(&format!("Exec={}\n", exec));

        match file.write(text.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("mkdsk: {e}");
                exit(6)
            }
        };

        // TODO(clovis): set chmod +x
        // file.metadata().unwrap().permissions().set_mode(0o111);
    }

    /// Note: this functions sets not only the `exec` field, but also the `out` and `name` fields,
    /// depending on the name of the executable file.
    pub fn set_exec(&mut self, exec: PathBuf) {
        self.exec = Some(exec);

        self.out = Some(current_dir().unwrap());

        let name = self.exec.clone().unwrap();
        let name = name.file_name().unwrap();
        self.name = String::from(name.to_str().unwrap());
    }
    pub fn set_out(&mut self, out: PathBuf) {
        self.out = Some(out);
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_comment(&mut self, comment: String) {
        self.comment = comment;
    }
    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }
    pub fn set_args(&mut self, args: String) {
        self.args = args;
    }
}
