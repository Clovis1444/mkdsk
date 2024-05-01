# About
Mkdsk - shorthand for **Make desktop**. This is a simple cli tool for linux that simplifies the creation of .desktop files.

# Usage
Use `mkdsk --help` for help.

# Installation
1. Download [mkdsk](https://github.com/Clovis1444/mkdsk/releases/latest).
2. cd to **mkdsk** executable directory: `cd path/to/mkdsk`.
3. Execute `sudo mv ./mkdsk /usr/local/bin` to make **mkdsk** globally accessible.

## Add context menu action
If you are using [Nemo](https://github.com/linuxmint/nemo) you can add the "Create desktop" context menu option by following these steps:

1. Download [create_desktop.nemo_action](https://github.com/Clovis1444/mkdsk/releases/latest).
2. cd to **create_desktop.nemo_action** file: `cd path/to/create_desktop.nemo_action`.
3. Execute `mv ./create_desktop.nemo_action ~/.local/share/nemo/actions`.

Note: **nemo_action** will not work if **mkdsk** is not [globally accessible](#installation).