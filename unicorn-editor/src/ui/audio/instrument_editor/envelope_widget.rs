use eframe::egui::{Slider, Ui};
use unicorn::audio::envelope_definition::{EnvelopeDefinition, EnvelopeValueType};

use crate::ui::AudioSyncHelper;

pub(crate) struct EnvelopeWidget {}

impl EnvelopeWidget {
    pub(crate) fn draw(ui: &mut Ui, envelope: &mut EnvelopeDefinition, sync: &mut AudioSyncHelper) {
        ui.group(|ui| {
            ui.label("Envelope Widget");

            ui.horizontal(|ui| {
                let responses = [
                    ui.add(
                        Slider::new(&mut envelope.total_level.0, 0..=EnvelopeValueType::MAX)
                            .text("TL")
                            .vertical(),
                    )
                    .changed(),
                    ui.add(
                        Slider::new(&mut envelope.attack_time.0, 0..=EnvelopeValueType::MAX)
                            .text("A")
                            .vertical(),
                    )
                    .changed(),
                    ui.add(
                        Slider::new(
                            &mut envelope.decay_attack_time.0,
                            0..=EnvelopeValueType::MAX,
                        )
                        .text("D1")
                        .vertical(),
                    )
                    .changed(),
                    ui.add(
                        Slider::new(&mut envelope.sustain_level.0, 0..=EnvelopeValueType::MAX)
                            .text("S")
                            .vertical(),
                    )
                    .changed(),
                    ui.add(
                        Slider::new(
                            &mut envelope.decay_sustain_time.0,
                            0..=EnvelopeValueType::MAX,
                        )
                        .text("D2")
                        .vertical(),
                    )
                    .changed(),
                    ui.add(
                        Slider::new(&mut envelope.release_time.0, 0..=EnvelopeValueType::MAX)
                            .text("R")
                            .vertical(),
                    )
                    .changed(),
                ];

                if responses.contains(&true) {
                    sync.notify_rom_changed();
                }
            });
        });
    }
}
