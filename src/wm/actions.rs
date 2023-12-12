

use std::process::*;

use super::*;

pub enum ActionType{
    Run {prog: String, args: Vec<String>},
    Close,
    CycleFocus {direction: bool},
    ChangeSize {amount: i32},
    NextScreen,
    PromoteWindow,
    MoveWindow,
    ToggleFull,
    Quit
}

impl<'a> WindowManager<'a> {

    pub fn execute(&mut self, action: &ActionType){
        match action {
            ActionType::Run{prog, args} => self.run(prog, args),
            ActionType::Close => self.close(),
            ActionType::CycleFocus{direction} => self.cycle_focus(direction),
            ActionType::ChangeSize { amount } => self.change_size(amount),
            ActionType::NextScreen => self.next_screen(),
            ActionType::PromoteWindow => self.promote_window(),
            ActionType::MoveWindow => self.move_window(),
            ActionType::ToggleFull => self.toggle_full(),
            ActionType::Quit => self.quit()
        }
    }

    // Runs a given command with arguments
    pub fn run(&mut self, prog: &String, args: &Vec<String>){
        self.procs.push(Command::new(prog).args(args).spawn().expect("Unable to run command"));
    }

    // Close the focused window
    pub fn close(&mut self){
        self.workspaces[self.focused].close_focused();
    }


    // Cycle focus through windows
    pub fn cycle_focus(&mut self, direction: &bool){
        let ws = &mut self.workspaces[self.focused];
        if ws.windows.is_empty(){
            return;
        }
        ws.set_focus(match direction {
            true => ws.focused+1,
            false => ws.focused+ws.windows.len()-1
        });
    }

    pub fn next_screen(&mut self){
        self.workspaces[self.focused].set_active(false);
        self.focused = (self.focused+1) % self.workspaces.len();
        self.workspaces[self.focused].set_active(true);
    }

    pub fn change_size(&mut self, amount: &i32){
        let ws = &mut self.workspaces[self.focused];
        ws.set_master_width((ws.master_width as i32 + amount) as u16);
    }

    pub fn promote_window(&mut self){
        self.workspaces[self.focused].promote_focused();
    }

    pub fn move_window(&mut self){
        if self.workspaces.len() <= 1 || self.workspaces[self.focused].windows.is_empty(){
            return;
        }
        let win_opt = self.workspaces[self.focused].remove_focused();
        if let Some(win) = win_opt {
            self.next_screen();
            self.workspaces[self.focused].add_window(win);
        }
    }

    pub fn toggle_full(&mut self){
        self.workspaces[self.focused].toggle_full();
    }

    // Exit the window manager
    pub fn quit(&mut self){
        exit(0);
    }

}