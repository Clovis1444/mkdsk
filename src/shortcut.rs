//! # shortcut
//! This module contains `Shortcut` structure that contains information about the `.desktop` file that will be created.

use std::{env::current_dir, fs::File, io::Write, path::PathBuf, process::exit};

/// Contains information about the `.desktop` file that will be created.
pub struct Shortcut {
    entry_type: EntryType,
    _version: String,
    exec: Option<PathBuf>,
    out: Option<PathBuf>,
    args: String,
    name: String,
    generic_name: String,
    comment: String,
    icon: String,
    no_display: bool,
    hidden: bool,
    only_shown_in: String,
    not_shown_in: String,
    d_bus_activatable: bool,
    try_exec: String,
    path: String,
    terminal: bool,
    actions: String,
    url: String,
    prefers_non_default_gpu: bool,
    single_main_window: bool,
}

impl Shortcut {
    pub fn new() -> Shortcut {
        Shortcut {
            entry_type: EntryType::Application,
            _version: String::new(),
            exec: None,
            out: None,
            args: String::new(),
            name: String::new(),
            generic_name: String::new(),
            comment: String::new(),
            icon: String::new(),
            no_display: false,
            hidden: false,
            only_shown_in: String::new(),
            not_shown_in: String::new(),
            d_bus_activatable: false,
            try_exec: String::new(),
            path: String::new(),
            terminal: false,
            actions: String::new(),
            url: String::new(),
            prefers_non_default_gpu: false,
            single_main_window: false,
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

        let mut text = format!(
            "[Desktop Entry]\nType={}\nName={}\n",
            self.entry_type.get_str(),
            self.name
        );
        if !self.generic_name.is_empty() {
            text.push_str(&format!("GenericName={}\n", self.generic_name));
        }
        if !self.comment.is_empty() {
            text.push_str(&format!("Comment={}\n", self.comment));
        }
        if !self.icon.is_empty() {
            text.push_str(&format!("Icon={}\n", self.icon));
        }

        let mut exec = String::from(self.exec.clone().unwrap().to_str().unwrap());
        if !self.args.is_empty() {
            exec.push(' ');
            exec.push_str(&self.args);
        }
        text.push_str(&format!("Exec={}\n", exec));
        if !self.try_exec.is_empty() {
            text.push_str(&format!("TryExec={}\n", self.try_exec));
        }

        text.push_str(&format!("NoDisplay={}\n", self.no_display.to_string()));
        text.push_str(&format!("Hidden={}\n", self.hidden.to_string()));

        if !self.only_shown_in.is_empty() {
            text.push_str(&format!("OnlyShownIn={}\n", self.only_shown_in));
        }
        if !self.not_shown_in.is_empty() {
            text.push_str(&format!("NotShownIn={}\n", self.not_shown_in));
        }

        text.push_str(&format!(
            "DBusActivatable={}\n",
            self.d_bus_activatable.to_string()
        ));
        if !self.path.is_empty() {
            text.push_str(&format!("Path={}\n", self.path));
        }
        text.push_str(&format!("Terminal={}\n", self.terminal.to_string()));
        if !self.actions.is_empty() {
            text.push_str(&format!("Actions={}\n", self.actions));
        }

        if let EntryType::Link = self.entry_type {
            match self.url.is_empty() {
                false => text.push_str(&format!("URL={}\n", self.url)),
                true => {
                    println!("mkdsk: you must specify URL if entry type is Link");
                    exit(12);
                }
            }
        }

        text.push_str(&format!(
            "PrefersNonDefaultGPU={}\n",
            self.prefers_non_default_gpu.to_string()
        ));
        text.push_str(&format!(
            "SingleMainWindow={}\n",
            self.single_main_window.to_string()
        ));

        // Write to file
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
    pub fn set_generic_name(&mut self, generic_name: String) {
        self.generic_name = generic_name;
    }
    pub fn set_no_display(&mut self, no_display: bool) {
        self.no_display = no_display;
    }
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }
    pub fn set_only_shown_in(&mut self, envs: String) {
        self.only_shown_in = envs;
    }
    pub fn set_not_shown_in(&mut self, envs: String) {
        self.not_shown_in = envs;
    }
    pub fn set_d_bus_activatable(&mut self, d_bus_activatable: bool) {
        self.d_bus_activatable = d_bus_activatable;
    }
    pub fn set_try_exec(&mut self, try_exec: String) {
        self.try_exec = try_exec;
    }
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
    pub fn set_terminal(&mut self, use_terminal: bool) {
        self.terminal = use_terminal;
    }
    pub fn set_actions(&mut self, actions: String) {
        self.actions = actions;
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url
    }
    pub fn set_prefers_non_default_gpu(&mut self, prefers_non_default_gpu: bool) {
        self.prefers_non_default_gpu = prefers_non_default_gpu
    }
    pub fn set_single_main_window(&mut self, single_main_window: bool) {
        self.single_main_window = single_main_window
    }
}

enum EntryType {
    Application,
    Link,
    Directory,
}

impl EntryType {
    fn get_str(&self) -> &str {
        match self {
            EntryType::Application => "Application",
            EntryType::Link => "Link",
            EntryType::Directory => "Directory",
        }
    }
}
