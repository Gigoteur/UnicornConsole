use eframe::epaint::Vec2;
use ui::Editor;

mod ui;

fn main() {
    let options = eframe::NativeOptions {
        vsync: true,
        initial_window_size: Some(Vec2::new(1366.0, 768.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Unicorn Editor",
        options,
        Box::new(|_cc| Box::new(Editor::default())),
    )
}