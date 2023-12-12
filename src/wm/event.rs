

use x11rb::{protocol::{Event, xproto::{MapRequestEvent, UnmapNotifyEvent, ButtonPressEvent, ButtonReleaseEvent, ConnectionExt, SetMode, KeyPressEvent, KeyReleaseEvent, ConfigureRequestEvent, ConfigureWindowAux}}};
use x11rb::connection::Connection;

use super::WindowManager;



impl<'a> WindowManager<'a>{

    // Match event type and call appropriate handler
    pub fn handle_event(&mut self, event: Event){
        match event {
            Event::MapRequest(event) => self.handle_map_request(event),
            Event::UnmapNotify(event) => self.handle_unmap_notify(event),
            Event::ButtonPress(event) => self.handle_button_press(event),
            Event::ButtonRelease(event) => self.handle_button_release(event),
            Event::KeyPress(event) => self.handle_key_press(event),
            Event::KeyRelease(event) => self.handle_key_release(event),
            Event::ConfigureRequest(event) => self.handle_configure_request(event),
            _ => {}
        }
    }

    // Handle adding a new window to the manager
    fn handle_map_request(&mut self, event: MapRequestEvent){
        println!("Adding Window: {:?}", event.window);
        self.config.connection.grab_server().expect("Unable To Grab Server");
        self.config.connection.change_save_set(SetMode::INSERT, event.window).expect("Unable To Change Save Set");
        self.config.connection.map_window(event.window).expect("Unable To Map Window");
        self.config.connection.ungrab_server().expect("Unable To Ungrab Server");
        self.workspaces[self.focused].add_window(event.window);
        self.tile_windows();
        self.set_focus(0);
    }

    // Handle removing a window from the manager
    fn handle_unmap_notify(&mut self, event: UnmapNotifyEvent){
        for ws in self.workspaces.iter_mut() {
            ws.remove_window(event.window);
        }
    }

    // Handle mouse button pressing
    fn handle_button_press(&mut self, event: ButtonPressEvent){
        println!("Button Pressed: {:?}", event.detail);
        if event.event != self.config.connection.setup().roots[self.config.screen_num].root {
            self.focus_window_id(event.event);
        }
    }

    // Handle mouse button releasing
    fn handle_button_release(&mut self, event: ButtonReleaseEvent){
        println!("Button Released: {:?}", event.detail);
    }

    // Handle key pressing
    fn handle_key_press(&mut self, event: KeyPressEvent){
        println!("Key Pressed: {:?}", event.detail);
        for kb in self.config.keybindings.iter() {
            if event.detail == kb.keycode && event.state == kb.modkeys.into() {
                self.execute(&kb.action);
            }
        }
    }

    // Handle key releasing
    fn handle_key_release(&mut self, event: KeyReleaseEvent){
        println!("Key Released: {:?}", event.detail);
    }

    // Handle window configuration requests
    fn handle_configure_request(&mut self, event: ConfigureRequestEvent){
        self.config.connection.configure_window(event.window, &ConfigureWindowAux::from_configure_request(&event)).expect("Unable to configure window");
        self.tile_windows();
    }
}
