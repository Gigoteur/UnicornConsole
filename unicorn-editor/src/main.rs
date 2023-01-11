use eframe::epaint::Vec2;
use ui::Editor;
use std::env;
use env_logger;

mod ui;
mod editor;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let options = eframe::NativeOptions {
        vsync: true,
        initial_window_size: Some(Vec2::new(1366.0, 768.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Unicorn Console Editor",
        options,
        Box::new(|_cc| Box::new(Editor::default())),
    )
}