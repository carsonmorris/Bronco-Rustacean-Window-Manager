
use x11rb::{protocol::{xinerama::ScreenInfo, xproto::{ConnectionExt, ConfigureWindowAux, ChangeWindowAttributesAux, InputFocus, ClientMessageEvent, EventMask, GrabMode, ButtonIndex, ModMask, StackMode}}};

use crate::config::Config;



pub struct Workspace<'a> {
    pub config: &'a Config, // Reference to config
    pub x: i16, // X coordinate of workspace
    pub y: i16, // Y coordinate of workspace
    pub width: u16, // Width of workspace
    pub height: u16, // Height of workspace
    pub master_width: u16,// Width of the master window
    pub windows: Vec<u32>, // List of windows
    pub focused: usize, // Focused Window
    pub active: bool, // Is the workspace currently focused
    pub is_full: bool, // Is the workspace full screen
}

impl<'a> Workspace<'a> {
    pub fn new(screen: &ScreenInfo, config: &'a Config) -> Self {
        Self {
            config,
            x: screen.x_org,
            y: screen.y_org,
            width: screen.width,
            height: screen.height,
            master_width: screen.width/2,
            windows: Vec::new(),
            focused: 0,
            active: false,
            is_full: false
        }
    }

    pub fn tile(&mut self) {
        if self.is_full && !self.windows.is_empty() {
            for win in self.windows.iter(){
                self.config.connection.configure_window(*win, 
                    &ConfigureWindowAux::new().x(self.x as i32).y(self.y as i32)
                    .width(self.width as u32).height(self.height as u32))
                    .expect("Unable to tile full screen window");
            }
            return;
        }

        if self.windows.len() == 1 {
            self.config.connection.configure_window(self.windows[0],
                &ConfigureWindowAux::new().x(self.config.gap as i32 + self.x as i32).y(self.config.gap as i32 + self.y as i32)
                .width((self.width-(self.config.gap*2))as u32).height((self.height-(self.config.gap*2)) as u32))
                .expect("Unable to tile window");
        }
        else if self.windows.len() > 1 {
            self.config.connection.configure_window(self.windows[0],
                &ConfigureWindowAux::new().x(self.config.gap as i32 + self.x as i32).y(self.config.gap as i32 + self.y as i32)
                .width(((self.master_width)-(self.config.gap*2)) as u32).height((self.height-(self.config.gap*2)) as u32))
                .expect("Unable to tile window");

            let stack_height = self.height / (self.windows.len() - 1) as u16;
            for i in 1..self.windows.len(){
                self.config.connection.configure_window(self.windows[i],
                    &ConfigureWindowAux::new().x((self.master_width) as i32 + self.x as i32).y((stack_height*(i-1) as u16+self.config.gap) as i32 + self.y as i32)
                    .width((self.width-self.master_width-self.config.gap) as u32).height((stack_height-(self.config.gap * if i == self.windows.len()-1 {2} else {1})) as u32))
                    .expect("Unable to tile window");
            }
        }
    }

    pub fn hide(&mut self) {

    }

    pub fn show(&mut self) {

    }

    pub fn add_window(&mut self, window: u32) {
        self.config.connection.configure_window(window, &ConfigureWindowAux::new().border_width(if self.is_full {0} else {3})).expect("Unable to set border width");
        self.windows.insert(0, window);
        self.set_focus(0);
        self.tile();
    }

    pub fn remove_focused(&mut self) -> Option<u32> {
        if self.windows.is_empty() {
            return None;
        }
        return self.remove_window(self.windows[self.focused]);
    }

    pub fn remove_window(&mut self, to_remove: u32) -> Option<u32> {
        let mut removed = None;
        self.windows.retain(|window|{
            if *window != to_remove{
                return true;
            }
            removed = Some(*window);
            //self.config.connection.change_save_set(SetMode::DELETE, *window).expect("Unable to change save state");
            //self.config.connection.reparent_window(*window, self.config.connection.setup().roots[self.config.screen_num].root, 0,0).expect("Unable to reparent window");
            return false;
        });
        if let Some(_) = removed {
            self.set_focus(self.focused);
            self.tile();
        }
        return removed;
    }

    pub fn set_active(&mut self, state: bool) {
        self.active = state;
        self.set_focus(self.focused);
    }

    pub fn set_focus(&mut self, index: usize) {
        if self.windows.is_empty(){
            self.focused = 0;
            return;
        }
        self.focused = index % self.windows.len();
        for i in 0..self.windows.len() {
            if i == self.focused && self.active {
                self.config.connection.ungrab_button(ButtonIndex::ANY, self.windows[i], ModMask::ANY).expect("Unable to ungrab button");
                self.config.connection.change_window_attributes(self.windows[i], &ChangeWindowAttributesAux::new().border_pixel(0x00bfff)).expect("Unable to set attributes");
                self.config.connection.configure_window(self.windows[i], &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE)).expect("Unable to raise window");
                self.config.connection.set_input_focus(InputFocus::PARENT, self.windows[i], x11rb::CURRENT_TIME).expect("Unable to focus window");
            }
            else {
                self.config.connection.grab_button(
                    true,
                    self.windows[i],
                    EventMask::BUTTON_PRESS,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                    x11rb::NONE,
                    x11rb::NONE,
                    ButtonIndex::ANY,
                    ModMask::ANY
                ).expect("Unable to grab button");
                self.config.connection.change_window_attributes(self.windows[i], &ChangeWindowAttributesAux::new().border_pixel(0x8b8378)).expect("Unable to set attributes");
            }
        }
    }

    pub fn set_master_width(&mut self, width: u16){
        if self.windows.is_empty() {
            return;
        }
        self.master_width = width;
        if self.master_width < self.width/10 {
            self.master_width = self.width/10
        }
        else if self.master_width > (9*self.width/10){
            self.master_width = 9*self.width/10;
        }
        self.tile();
    }

    pub fn close_focused(&mut self){
        if self.windows.is_empty() {
            return;
        }
        self.config.connection.send_event(
            false,
            self.windows[self.focused],
            EventMask::NO_EVENT,
            ClientMessageEvent::new(
                32,
                self.windows[self.focused],
                self.config.protocol_atom,
                [self.config.delete_atom, 0, 0, 0, 0]

            )
        ).expect("Unable to send close event");
        //self.config.connection.destroy_window(self.windows[self.focused]).expect("Unable to destroy window");
    }

    pub fn promote_focused(&mut self){
        if self.windows.is_empty(){
            return;
        }
        let win = self.windows.remove(self.focused);
        self.windows.insert(0, win);
        self.set_focus(0);
        self.tile();
    }

    pub fn toggle_full(&mut self){
        self.is_full = !self.is_full;
        for win in self.windows.iter() {
            self.config.connection.configure_window(*win, &ConfigureWindowAux::new().border_width(if self.is_full {0} else {3})).expect("Unable to set border width");
        }
        self.tile();
    }

    pub fn focus_window_id(&mut self, win: u32) -> bool{
        if self.windows.is_empty(){
            self.focused = 0;
            return false;
        }
        let mut ret = false;
        for i in 0..self.windows.len() {
            if self.windows[i] == win {
                self.focused = i;
                self.active = true;
                self.config.connection.ungrab_button(ButtonIndex::ANY, self.windows[i], ModMask::ANY).expect("Unable to ungrab button");
                self.config.connection.change_window_attributes(self.windows[i], &ChangeWindowAttributesAux::new().border_pixel(0x00bfff)).expect("Unable to set attributes");
                self.config.connection.configure_window(self.windows[i], &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE)).expect("Unable to raise window");
                self.config.connection.set_input_focus(InputFocus::PARENT, self.windows[i], x11rb::CURRENT_TIME).expect("Unable to focus window");
                ret = true;
            }
            else {
                self.config.connection.grab_button(
                    true,
                    self.windows[i],
                    EventMask::BUTTON_PRESS,
                    GrabMode::ASYNC,
                    GrabMode::ASYNC,
                    x11rb::NONE,
                    x11rb::NONE,
                    ButtonIndex::ANY,
                    ModMask::ANY
                ).expect("Unable to grab button");
                self.config.connection.change_window_attributes(self.windows[i], &ChangeWindowAttributesAux::new().border_pixel(0x8b8378)).expect("Unable to set attributes");
            }
        }
        return ret;
    }
}