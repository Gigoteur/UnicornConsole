use std::hash::Hash;

use eframe::egui::{ComboBox, Ui};

use gamercade_audio::IndexInterpolator;

use crate::ui::AudioSyncHelper;

pub(crate) struct InterpolatorWidget {}

impl InterpolatorWidget {
    pub(crate) fn draw(
        ui: &mut Ui,
        interpolator: &mut IndexInterpolator,
        sync: &mut AudioSyncHelper,
        id_source: &impl Hash,
    ) {
        let mut should_notify = false;
        ui.group(|ui| {
            ComboBox::new(id_source, "Interpolation")
                .selected_text(format!("{:?}", &interpolator))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(interpolator, IndexInterpolator::Linear, "Linear")
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            interpolator,
                            IndexInterpolator::NearestNeighbor,
                            "NearestNeighbor",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(interpolator, IndexInterpolator::Truncate, "Truncate")
                        .clicked()
                    {
                        should_notify = true;
                    };
                });
        });

        if should_notify {
            sync.notify_rom_changed()
        }
    }
}
