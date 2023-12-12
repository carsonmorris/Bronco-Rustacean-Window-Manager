
use std::process::Child;

use x11rb::protocol::xinerama::query_screens;
use x11rb::rust_connection::ReplyError;
use x11rb::connection::Connection;
use x11rb::protocol::{xproto::*, ErrorKind};
use crate::config::Config;

use self::workspace::Workspace;


pub mod event;
pub mod actions;
pub mod workspace;

pub struct WindowManager<'a>{
    config: &'a Config, // Reference to the config struct
    workspaces: Vec<Workspace<'a>>, // Vector of windows to manage
    focused: usize, // Index of the focused workspace
    procs: Vec<Child>, // Current running child processes spawned by the run action
}

impl<'a> WindowManager<'a>{
    pub fn new(config: &'a Config) -> Self{
        // Get the screen
        let screen = &config.connection.setup().roots[config.screen_num];

        // Ensure we are the window manager
        let change = ChangeWindowAttributesAux::default()
            .event_mask(EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY
            | EventMask::BUTTON_PRESS | EventMask::BUTTON_RELEASE | EventMask::KEY_PRESS | EventMask::KEY_RELEASE);
        let res = config.connection.change_window_attributes(screen.root, &change)
            .expect("Unable to set attributes").check();
        if let Err(ReplyError::X11Error(ref error)) = res {
            if error.error_kind == ErrorKind::Access {
                panic!("Cannot become window manager");
            }
        }

        // Return the new window manager
        Self{
            config,
            workspaces: Vec::new(),
            focused: 0,
            procs: Vec::new()
        }
    }


    // Start the window manager
    pub fn start(&mut self){

        // Query screens and build workspaces
        let screen_list = query_screens(&self.config.connection)
            .expect("Unable to query screens")
            .reply().expect("Unable to query screens")
            .screen_info;

        for s in screen_list.iter() {
            self.workspaces.push(Workspace::new(s, self.config));
        }
        if self.workspaces.is_empty() {
            panic!("No screens available");
        }
        self.workspaces[self.focused].set_active(true);

        // Grab all keybindings
        for kb in &self.config.keybindings {
            self.config.connection.grab_key(
                false,
                self.config.connection.setup().roots[self.config.screen_num].root,
                ModMask::from(kb.modkeys),
                kb.keycode,
                GrabMode::ASYNC,
                GrabMode::ASYNC
            ).expect("Unable to grab key");
        }
        // Start event loop
        loop{
            // Wait for a new event
            self.config.connection.flush().unwrap();
            let event = self.config.connection.wait_for_event().unwrap();
            let mut event_option = Some(event);

            // Loop while there are still events and handle them
            while let Some(event) = event_option {
                self.handle_event(event);
                event_option = self.config.connection.poll_for_event().unwrap();
            }

            // Clean up any finished child processes
            self.procs.retain_mut(|proc| {
                match proc.try_wait() {
                    Ok(Some(_)) => false,
                    Ok(None) => true,
                    Err(_) => true
                }
            })
        }
    }

    pub fn tile_windows(&mut self){
        for ws in self.workspaces.iter_mut() {
            ws.tile();
        }

    }

    pub fn set_focus(&mut self, index: usize){
        self.workspaces[self.focused].set_focus(index);
    }

    pub fn set_master_width(&mut self, width: u16){
        self.workspaces[self.focused].set_master_width(width);
    }

    pub fn focus_window_id(&mut self, win: u32){
        for i in 0..self.workspaces.len() {
            if self.workspaces[i].focus_window_id(win) {
                self.focused = i;
            }
        }
    }
}