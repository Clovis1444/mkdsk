//! # shortcut
//! This module contains `Shortcut` structure that contains information about the `.desktop` file that will be created.

use std::{fs::File, io::Write, path::PathBuf, process::exit};

/// Contains information about the `.desktop` file that will be created.
pub struct Shortcut {
    out: Option<PathBuf>,
    entry_type: EntryType,
    _version: String,
    name: String,
    generic_name: String,
    no_display: bool,
    comment: String,
    icon: String,
    hidden: bool,
    only_shown_in: String,
    not_shown_in: String,
    d_bus_activatable: bool,
    try_exec: String,
    exec: Option<PathBuf>,
    args: String,
    path: String,
    terminal: bool,
    actions: String,
    mime_type: String,
    categories: String,
    implements: String,
    keywords: String,
    startup_notify: bool,
    startup_wm_class: String,
    url: String,
    prefers_non_default_gpu: bool,
    single_main_window: bool,
}

impl Shortcut {
    pub fn new() -> Shortcut {
        Shortcut {
            out: None,
            entry_type: EntryType::Application,
            _version: String::new(),
            name: String::new(),
            generic_name: String::new(),
            no_display: false,
            comment: String::new(),
            icon: String::new(),
            hidden: false,
            only_shown_in: String::new(),
            not_shown_in: String::new(),
            d_bus_activatable: false,
            try_exec: String::new(),
            exec: None,
            args: String::new(),
            path: String::new(),
            terminal: false,
            actions: String::new(),
            mime_type: String::new(),
            categories: String::new(),
            implements: String::new(),
            keywords: String::new(),
            startup_notify: false,
            startup_wm_class: String::new(),
            url: String::new(),
            prefers_non_default_gpu: false,
            single_main_window: false,
        }
    }

    /// Creates .desktop file, passes all `Shortcut data` to it.
    /// This function will close the program if file creation fails.
    /// Edit this function if you are plannig to add new `Shortcut` fields.
    pub fn create(&self) {
        assert!(&self.out.is_some(), "out must be set");
        assert!(!&self.name.is_empty(), "name must be set");
        match self.entry_type {
            EntryType::Application | EntryType::Directory => {
                assert!(&self.exec.is_some(), "exec must be set")
            }
            EntryType::Link => assert!(!&self.url.is_empty(), "url must be set"),
        }

        let mut file_name = self.name.clone();
        file_name.push_str(".desktop");
        let file_path = self.out.clone().unwrap().join(file_name);

        // Check if file already exists
        if file_path.exists() {
            // Exit if user does not want to overwrite existing file
            if !self.ask_user_overwrite() {
                exit(0);
            }
        }

        let mut file = match File::create(file_path.clone()) {
            Ok(val) => val,
            Err(e) => {
                println!("mkdsk: {e}");
                exit(5)
            }
        };

        // Keys that make sense in all contexts
        let mut text = format!(
            "[Desktop Entry]\nType={}\nName={}\n",
            self.entry_type.get_str(),
            self.name
        );
        if !self.generic_name.is_empty() {
            text.push_str(&format!("GenericName={}\n", self.generic_name));
        }
        text.push_str(&format!("NoDisplay={}\n", self.no_display.to_string()));
        if !self.comment.is_empty() {
            text.push_str(&format!("Comment={}\n", self.comment));
        }
        if !self.icon.is_empty() {
            text.push_str(&format!("Icon={}\n", self.icon));
        }
        text.push_str(&format!("Hidden={}\n", self.hidden.to_string()));
        if !self.only_shown_in.is_empty() {
            text.push_str(&format!("OnlyShownIn={}\n", self.only_shown_in));
        }
        if !self.not_shown_in.is_empty() {
            text.push_str(&format!("NotShownIn={}\n", self.not_shown_in));
        }

        // Apllication specific keys
        if self.entry_type == EntryType::Application {
            text.push_str(&format!(
                "DBusActivatable={}\n",
                self.d_bus_activatable.to_string()
            ));
            if !self.try_exec.is_empty() {
                text.push_str(&format!("TryExec={}\n", self.try_exec));
            }
            let mut exec = String::from(self.exec.clone().unwrap().to_str().unwrap());
            if !self.args.is_empty() {
                exec.push(' ');
                exec.push_str(&self.args);
            }
            text.push_str(&format!("Exec={}\n", exec));

            if !self.path.is_empty() {
                text.push_str(&format!("Path={}\n", self.path));
            }
            text.push_str(&format!("Terminal={}\n", self.terminal.to_string()));
            if !self.actions.is_empty() {
                text.push_str(&format!("Actions={}\n", self.actions));
            }
            if !self.mime_type.is_empty() {
                text.push_str(&format!("MimeType={}\n", self.mime_type));
            }
            if !self.categories.is_empty() {
                text.push_str(&format!("Categories={}\n", self.categories));
            }
            if !self.implements.is_empty() {
                text.push_str(&format!("Implements={}\n", self.implements));
            }
            if !self.keywords.is_empty() {
                text.push_str(&format!("Keywords={}\n", self.keywords));
            }
            text.push_str(&format!(
                "StartupNotify={}\n",
                self.startup_notify.to_string()
            ));
            if !self.startup_wm_class.is_empty() {
                text.push_str(&format!("StartupWMClass={}\n", self.startup_wm_class));
            }
            text.push_str(&format!(
                "PrefersNonDefaultGPU={}\n",
                self.prefers_non_default_gpu.to_string()
            ));
            text.push_str(&format!(
                "SingleMainWindow={}\n",
                self.single_main_window.to_string()
            ));
        }
        // Directory specific keys
        else if self.entry_type == EntryType::Directory {
            // TODO(clovis): fix Directory?
            text.push_str(&format!(
                "Exec={}\n",
                self.exec.clone().unwrap().to_str().unwrap()
            ));
        }
        // Link specific keys
        else if self.entry_type == EntryType::Link {
            text.push_str(&format!("URL={}\n", self.url))
        }

        // Write to file
        match file.write(text.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                println!("mkdsk: {e}");
                exit(6)
            }
        };
    }

    pub fn set_exec(&mut self, exec: PathBuf) {
        self.exec = Some(exec);
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
    pub fn set_entry_type(&mut self, entry_type: EntryType) {
        self.entry_type = entry_type
    }
    pub fn set_mime_type(&mut self, mime_type: String) {
        self.mime_type = mime_type
    }
    pub fn set_categories(&mut self, categories: String) {
        self.categories = categories
    }
    pub fn set_implements(&mut self, implements: String) {
        self.implements = implements
    }
    pub fn set_keywords(&mut self, keywords: String) {
        self.keywords = keywords
    }
    pub fn set_startup_notify(&mut self, startup_notify: bool) {
        self.startup_notify = startup_notify;
    }
    pub fn set_startup_wm_class(&mut self, startup_wm_class: String) {
        self.startup_wm_class = startup_wm_class
    }

    /// Asks user to overwrite file if the file is already exists.
    fn ask_user_overwrite(&self) -> bool {
        let mut file_name = self.name.clone();
        file_name.push_str(".desktop");

        println!(
            "\"{}\" already exists in {:#?}. Do you want to replace it?[Y/N]: ",
            file_name,
            self.out.clone().unwrap()
        );

        let mut user_input = String::new();

        loop {
            match std::io::stdin().read_line(&mut user_input) {
                Err(e) => {
                    println!("mkdsk: {e}");
                    exit(16)
                }
                Ok(_) => (),
            };

            match user_input.to_lowercase().as_str().trim() {
                "y" => return true,
                "n" => return false,
                _ => {
                    user_input.clear();
                    continue;
                }
            }
        }
    }
}

#[derive(PartialEq)]
pub enum EntryType {
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
