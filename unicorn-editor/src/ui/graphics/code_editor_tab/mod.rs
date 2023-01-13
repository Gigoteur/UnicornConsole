
mod syntax_highlighting;

use eframe::egui::{TextEdit, TextureId, Ui, Vec2, Color32, ScrollArea};
use unicorn::core::Code;

#[derive(Clone, Debug)]
pub struct CodeEditor {
    language: String,
    code: String,
}

impl Default for CodeEditor {
    fn default() -> Self {
        Self {
            language: "py".into(),
            code: "// A very simple example\n\
fn main() {\n\
\tprintln!(\"Hello world!\");\n\
}\n\
"
            .into(),
        }
    }
}

impl CodeEditor {
    pub fn set_code(&mut self, language: String, code: String) {
        self.language = language;
        self.code = code;
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui, rom: &mut unicorn::core::Unicorn, scale: f32, texture_id: TextureId) {

        let mut theme = syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        ui.collapsing("Theme", |ui| {
            ui.group(|ui| {
                theme.ui(ui);
                theme.clone().store_in_memory(ui.ctx());
            });
        });

        let mut layouter = |ui: &eframe::egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                syntax_highlighting::highlight(ui.ctx(), &theme, string, &self.language);
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };

        ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                eframe::egui::TextEdit::multiline(&mut self.code)
                    .font(eframe::egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layouter),
            );
        });
    }
}