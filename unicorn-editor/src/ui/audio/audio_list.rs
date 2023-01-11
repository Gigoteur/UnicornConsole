use eframe::egui::{ScrollArea, SidePanel, TopBottomPanel, Ui};

use crate::editor::editor_sounds_data::{EditorSoundData, EditorAudioDataEntry};

use crate::ui::AudioSyncHelper;

pub(crate) trait AudioList<T> {
    const MAX_ENTRY_COUNT: usize;
    const NAME: &'static str;
    fn target_data_mut(data: &mut EditorSoundData) -> &mut Vec<EditorAudioDataEntry<T>>;
    fn selected_index(&mut self) -> &mut usize;

    fn on_add() -> T;
    fn on_clear(&mut self, data: &mut Vec<EditorAudioDataEntry<T>>);

    fn draw_buttons(
        &mut self,
        ui: &mut Ui,
        data: &mut EditorSoundData,
        sync: &mut AudioSyncHelper,
    ) {
        let data = Self::target_data_mut(data);
        ui.horizontal(|ui| {
            if ui.button(format!("Add {}", Self::NAME)).clicked() {
                let curr_len = data.len();
                if curr_len < Self::MAX_ENTRY_COUNT {
                    let name = format!("{} {}", Self::NAME, curr_len);
                    data.push(EditorAudioDataEntry {
                        name,
                        data: Self::on_add(),
                    });
                    sync.notify_rom_changed();
                }
            }

            if ui.button(format!("Clear {}", Self::NAME)).clicked() {
                self.on_clear(data);
                sync.notify_rom_changed();
            }
        });
    }

    fn draw(&mut self, ui: &mut Ui, data: &mut EditorSoundData, sync: &mut AudioSyncHelper) {
        SidePanel::left("Audio List")
            .resizable(false)
            .show_inside(ui, |ui| {
                TopBottomPanel::bottom("Audio Bottom Panel")
                    .show_inside(ui, |ui| self.draw_buttons(ui, data, sync));

                ui.vertical(|ui| {
                    ui.label(format!("{} List", Self::NAME));

                    // Draws the list of instruments
                    ui.group(|ui| {
                        ScrollArea::vertical().show(ui, |ui| {
                            Self::target_data_mut(data).iter().enumerate().for_each(
                                |(index, thing)| {
                                    ui.horizontal(|ui| {
                                        let is_checked = *self.selected_index() == index;

                                        if ui
                                            .selectable_label(
                                                is_checked,
                                                format!("[{:02X}]: {}", index, &thing.name),
                                            )
                                            .clicked()
                                        {
                                            *self.selected_index() = index
                                        };
                                    });
                                },
                            );
                        })
                    });
                });
            });
    }
}
