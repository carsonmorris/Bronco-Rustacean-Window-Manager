
use crate::wm::actions::ActionType;

pub struct Keybind {
    pub keycode: u8,
    pub modkeys: u16,
    pub action: ActionType
}