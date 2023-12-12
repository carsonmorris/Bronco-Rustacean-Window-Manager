```
 ____                                  ____            _                             
| __ ) _ __ ___  _ __   ___ ___       |  _ \ _   _ ___| |_ __ _  ___ ___  __ _ _ __  
|  _ \| '__/ _ \| '_ \ / __/ _ \ _____| |_) | | | / __| __/ _` |/ __/ _ \/ _` | '_ \ 
| |_) | | | (_) | | | | (_| (_) |_____|  _ <| |_| \__ \ || (_| | (_|  __/ (_| | | | |
|____/|_|  \___/|_| |_|\___\___/      |_| \_\\__,_|___/\__\__,_|\___\___|\__,_|_| |_|
                                                                                     
__        ___           _                 __  __                                   
\ \      / (_)_ __   __| | _____      __ |  \/  | __ _ _ __   __ _  __ _  ___ _ __ 
 \ \ /\ / /| | '_ \ / _` |/ _ \ \ /\ / / | |\/| |/ _` | '_ \ / _` |/ _` |/ _ \ '__|
  \ V  V / | | | | | (_| | (_) \ V  V /  | |  | | (_| | | | | (_| | (_| |  __/ |   
   \_/\_/  |_|_| |_|\__,_|\___/ \_/\_/   |_|  |_|\__,_|_| |_|\__,_|\__, |\___|_|   
                                                                   |___/           
```

## General Info
- Project: Bronco-Rustacean Window Manager
- Authors: Matthew Oberg, Spencer Mirly, and Carson Morris


## Overview
- The Bronco-Rustacean Window Manager (BRWM) is a minimal tiling window manager for X11 written in Rust for the CS354 Programming Languages final project
- It contains basic funcitonality for rendering and manipulating windows using keyboard shortcuts
- Features include:
    - Rendering Windows
    - Opening / Closing WIndows
    - Resizing windows
    - Re-organizing windows
    - Shortcuts for running applications
    - Customizable keybindings
    - Multi-monitor support
    - And more!


## Files
- README.md - Documentation
- config.json - Default config file
- Cargo.toml - Project configuration
- install.sh - Install script
- src - Source code directory

## Installation Instructions (Ubuntu)

1. Install rustup
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install build dependancies
```bash
$ sudo apt install build-essential libx11-dev libxinerama-dev
```
3. Optionally install Rofi (Default run prompt)
```bash
$ sudo apt install rofi
```
3. Build and install the application with the install script
```bash
$ ./install.sh
```
4. Switch from GDM to Xinit
```bash
$ sudo apt install xinit
$ sudo systemctl disable gdm
```
5. Restart Ubuntu and log in at the terminal
6. Start BRWM with this command
```
$ startx
```

## Using BRWM
- Running the install script copied the default config.json to ~/.config/brwm/config.json if it was not already there.
- All keybindings can be modified in that file, however default keybindings will be listed here.

| Keybinding       | Action                      |
|------------------|-----------------------------|
| ALT + R          | Launch Rofi                 |
| ALT + X          | Close Window                |
| ALT + J/K        | Cycle Focus                 |
| ALT + H/L        | Resize Window               |
| ALT + Enter      | Promote Window              |
| ALT + M          | Toggle Fullscreen           |
| ALT + Space      | Focus Next Monitor          |
| ALT + O          | Move Window To Next Monitor |
| CTRL + SHIFT + Q | Quit BRWM                   |

- The default window layout is master and stack
- Using ALT + M toggles to fullscreen layout


## Customizing BRWM

- Keycodes are used to customize the keyboard shortcuts
    - To view the keycodes, you can use xmodmap:
    ```bash
    $ xmodmap -pke
    ```
    - For modkeys, here is a simple cheat sheet for modifier combos:

    |Key          |Code  |
    |-------------|------|
    |Shift        |1     |
    |Control      |4     |
    |Alt          |8     |
    |Super/Windows|64    |
    - To get the desired value, add together the values of the keys you want
    - For example: Control + Shift = 1 + 4 = 5
- Actions determine what a keybinding does, along with its arguments

|Action       |Arguments  | Effects                                              |
|-------------|-----------|------------------------------------------------------|
|Run          |Command    |Runs the specified command as a new process           |
|CycleFocus   |"+" or "-" |Cycles focus through the windows forwards or backwards|
|ChangeSize   |"+" or "-" |Increases or decreases the size of the master window  |
|PromoteWindow|None       |Sets the focused window to the master window          |
|Close        |None       |Closes the focused window                             |
|NextScreen   |None       |Switches focus to the next monitor                    |
|MoveWindow   |None       |Moves the focused window to the next monitor          |
|ToggleFull   |None       |Toggles fullscreen mode on the focused monitor        |
|Quit         |None       |Closes BRWM and ends the current X session            |