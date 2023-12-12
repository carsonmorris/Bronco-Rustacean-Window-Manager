
use std::io::Read;
use std::fs::File;

use dirs_next::config_dir;
use serde_derive::Deserialize;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{Atom, ConnectionExt};
use x11rb::rust_connection::RustConnection;

pub mod keybind;
use keybind::Keybind;

use crate::wm::actions::ActionType;

pub struct Config {
    pub connection: RustConnection,
    pub screen_num: usize,
    pub gap: u16,
    pub border: u16,
    pub keybindings: Vec<Keybind>,
    pub protocol_atom: Atom,
    pub delete_atom: Atom,
}

impl Config {
    pub fn new() -> Self {
        let (connection, screen_num) = x11rb::connect(None)
            .expect("Unable to establish connection");

        let protocol_atom = connection.intern_atom(false, b"WM_PROTOCOLS")
            .expect("Unable to get protocol atom")
            .reply()
            .expect("Unable to get protocol atom").atom;
        let delete_atom = connection.intern_atom(false, b"WM_DELETE_WINDOW")
            .expect("Unable to get delete atom")
            .reply()
            .expect("Unable to get delete atom").atom;

        // Get the screen
        let screen = &connection.setup().roots[screen_num];

        // Open config file
        let mut config_path = config_dir().expect("Unable to get config directory");
        config_path.push("brwm/config.json");
        let config_file = File::open(config_path);
        let mut contents = String::new();
        config_file.expect("Config file not found.").read_to_string(&mut contents).expect("Config file not found.");

        #[derive(Deserialize)]
        struct JSONRead {
            pub keycode: u8,
            pub modkey: u16,
            pub action: String,
            pub args: String
        }

        let reads: Vec<JSONRead> = serde_json::from_str(&contents).expect("Misformatted config file.");

        let mut keybindings: Vec<Keybind> = Vec::new();

        for binding in reads {
            match binding.action.as_str() {
                "Run" => {
                    let words = shlex::split(binding.args.as_str()).expect("Unable to get command");
                    let prog = words[0].to_owned();
                    let args = if words.len() > 1 {Vec::from_iter(words[1..].to_owned())} else {Vec::new()};
                    keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::Run {prog, args}})
                },
                "Quit" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::Quit}),
                "CycleFocus" => {
                    match binding.args.as_str() {
                        "+" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::CycleFocus {direction: true}}),
                        "-" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::CycleFocus {direction: false}}),
                        _ => panic!("Misformatted CycleFocus args.")
                    }
                },
                "ChangeSize" => {
                    match binding.args.as_str() {
                        "+" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::ChangeSize {amount: -(screen.width_in_pixels as i32/20)}}),
                        "-" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::ChangeSize {amount: (screen.width_in_pixels as i32/20)}}),
                        _ => panic!("Misformatted ChangeSize args.")
                    }
                },
                "PromoteWindow" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::PromoteWindow}),
                "Close" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::Close}),
                "NextScreen" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::NextScreen}),
                "MoveWindow" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::MoveWindow}),
                "ToggleFull" => keybindings.push(Keybind {keycode: binding.keycode, modkeys: binding.modkey, action: ActionType::ToggleFull}),
                _ => panic!("Misformatted config file.")
            };
        }

        Self {
            connection,
            screen_num,
            gap: 16,
            border: 2,
            keybindings,
            protocol_atom,
            delete_atom
        }
    }
}
