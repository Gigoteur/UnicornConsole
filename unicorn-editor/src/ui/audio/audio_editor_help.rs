use eframe::egui::{RichText, Ui, Window};

#[derive(Debug, Default)]
pub(crate) struct AudioEditorHelp {
    pub(crate) open: bool,
}

impl AudioEditorHelp {
    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();
        Window::new("Audio Editor Help")
        .open(&mut self.open)
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(RichText::new("How to use the Piano Roll: ").strong());
            ui.label("Keys [Z] through [M], and [Q] through [U] represent the white keys.");
            ui.label("Black keys range from [S] to [J], and [2] and [7].");
            ui.label("The keys can also be clicked.");

            ui.separator();

            ui.label(RichText::new("Envelope Widget: ").strong());
            ui.label("TL: Total Level - The the full volume of this sound source.");
            ui.label("A: Attack time - How long it takes to reach Total Level.");
            ui.label("D1: Decay 1 - How long it takes to travel from Total Level to Sustain Level");
            ui.label("S: Sustain Level - The sustained volume of this sound source, when a key is held.");
            ui.label("D2: Decay 2 - How long it takes for this sound to decay while holding the key.");
            ui.label("R: Release - How long it takes for this sound to decay while the key is released.");

            ui.separator();

            ui.label(RichText::new("Songs, chains, and phrases: ").strong());
            ui.label("A Phrase is a series of notes and instruments.");
            ui.label("A Chain is a series if phrases linked together.");
            ui.label("A Song is a series of Chains, one per output channel.");

            ui.label(RichText::new("How to nagivate the tracker: ").strong());
            ui.label("Navigate around entries with arrow keys or clicking entries");
            ui.label("[Space Bar] can be used to play the current track");

            ui.label(RichText::new("How to modify values: ").strong());
            ui.label("Editing values can be done by holding [Shift] and pressing the related key.");
            ui.label("Hold [Shift] and [Z] to create or delete entries.");
            ui.label("Hold [Shift] and press [Up] or [Down] arrows to increase or decrease a value.");
            ui.label("Hold [Shift] and press [Right] or [Left] arrows to increase or decrease a value by 16.");
        });
    }
}
