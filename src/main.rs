extern crate iui;

use iui::UI;
use iui::controls::{Window, WindowType};

fn main() {
    let ui = UI::init().unwrap();
    let mut window = iui::controls::Window::new(
        &ui,
        "hello window!",
        800,
        600,
        WindowType::HasMenubar
    );
    window.show(&ui);
    ui.main();
}
