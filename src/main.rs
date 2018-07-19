extern crate iui;

use iui::UI;
use iui::controls::{Window, WindowType, Button};

fn main() {
    let ui = UI::init().unwrap();
    let mut window = Window::new(
        &ui,
        "hello window!",
        800,
        600,
        WindowType::HasMenubar
    );

    let button = Button::new(
        &ui,
        "click me"
    );

    window.set_child(&ui, button);
    window.show(&ui);
    ui.main();
}
