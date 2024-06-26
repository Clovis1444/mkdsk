# About
Mkdsk - shorthand for **Make desktop**. This is a simple cli tool for linux that simplifies the creation of .desktop files.

# Features
- Ability to create **.desktop** files without using any text editor.
- You can create desktop entries with types: **Application**, **Link**, **Directory**.
- To create minimal **.desktop** you can pass only path to source.
- You can pass various `[Desktop Entry]` keys. All of them may be omitted, except *path to the executable*.
- You can create file manager [**context menu option**](#add-context-menu-action) using this tool.

# Usage
Use `mkdsk --help` for help.

# Installation
1. Download [mkdsk](https://github.com/Clovis1444/mkdsk/releases/latest).
2. Open **mkdsk** directory in terminal.
3. Execute `sudo mv ./mkdsk /usr/local/bin` to make **mkdsk** globally accessible.

## Add context menu action
If you are using [Nemo File Manager](https://github.com/linuxmint/nemo) you can add the "Create desktop" context menu option by following these steps:

1. Download [create_desktop.nemo_action](https://github.com/Clovis1444/mkdsk/releases/latest).
2. Open **create_desktop.nemo_action** directory in terminal.
3. Execute `mv ./create_desktop.nemo_action ~/.local/share/nemo/actions`.

Note: **nemo_action** will not work if **mkdsk** is not [globally accessible](#installation).