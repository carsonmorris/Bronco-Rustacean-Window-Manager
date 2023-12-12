use brwm::config::Config;
use brwm::wm::WindowManager;

fn main(){
    let config = Config::new();

    // Create a window manager and start it
    let mut wm = WindowManager::new(&config);
    wm.start();
}