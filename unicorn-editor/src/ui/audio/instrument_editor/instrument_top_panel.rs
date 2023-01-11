use eframe::{
    egui::{Label, RichText, SelectableLabel, Ui},
    epaint::Color32,
};

use unicorn::audio::instruments::instrument_data_definition::InstrumentDataDefinition;
use unicorn::audio::instruments::fm::patch_definition::PatchDefinition;
use unicorn::audio::instruments::sampler::sample_definition::SampleDefinition;
use unicorn::audio::instruments::wavetable::wavetable_definition::WavetableDefinition;


use crate::editor::editor_sounds_data::EditorAudioDataEntry;

use crate::ui::AudioSyncHelper;

use super::KeyboardMode;

#[derive(Clone)]
pub(crate) struct InstrumentTopPanel {
    editable: bool,

    wavetable_default: InstrumentDataDefinition,
    fm_default: InstrumentDataDefinition,
    sampler_default: InstrumentDataDefinition,
}

impl Default for InstrumentTopPanel {
    fn default() -> Self {
        Self {
            editable: false,
            wavetable_default: InstrumentDataDefinition::Wavetable(WavetableDefinition::default()),
            fm_default: InstrumentDataDefinition::FMSynth(PatchDefinition::default()),
            sampler_default: InstrumentDataDefinition::Sampler(SampleDefinition::default()),
        }
    }
}

impl InstrumentTopPanel {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        instrument: &mut EditorAudioDataEntry<Option<InstrumentDataDefinition>>,
        sync: &mut AudioSyncHelper,
        keyboard_mode: &mut KeyboardMode,
    ) {
        ui.group(|ui| {
            ui.label("Instrument Top Panel");
            ui.horizontal(|ui| {
                ui.label("Instrument Name: ");
                if ui.text_edit_singleline(&mut instrument.name).has_focus() {
                    *keyboard_mode = KeyboardMode::Normal
                } else {
                    *keyboard_mode = KeyboardMode::PianoRoll
                };
            });

            ui.horizontal(|ui| {
                if ui.button("Select Instrument Type").clicked() {
                    self.editable = !self.editable
                };

                ui.separator();

                add_instrument_type_button(
                    &mut self.editable,
                    ui,
                    &mut instrument.data,
                    sync,
                    "Wavetable",
                    &self.wavetable_default,
                );
                add_instrument_type_button(
                    &mut self.editable,
                    ui,
                    &mut instrument.data,
                    sync,
                    "FM Synth",
                    &self.fm_default,
                );
                add_instrument_type_button(
                    &mut self.editable,
                    ui,
                    &mut instrument.data,
                    sync,
                    "Sample",
                    &self.sampler_default,
                );

                if self.editable {
                    let text = Label::new(
                        RichText::new("Warning: Changing instrument type results in loss of data!")
                            .color(Color32::DARK_RED),
                    );
                    ui.add(text);
                }
            })
        });
    }
}

fn add_instrument_type_button(
    editable: &mut bool,
    ui: &mut Ui,
    instrument: &mut Option<InstrumentDataDefinition>,
    sync: &mut AudioSyncHelper,
    text: &str,
    default_instrument: &InstrumentDataDefinition,
) {
    if let Some(instrument) = instrument {
        let same_kind = instrument.get_kind() == default_instrument.get_kind();

        if ui
            .add_enabled(*editable, SelectableLabel::new(same_kind, text))
            .clicked()
            && !same_kind
        {
            *instrument = default_instrument.clone();
            *editable = false;
            sync.notify_rom_changed();
        };
    } else if ui
        .add_enabled(*editable, SelectableLabel::new(false, text))
        .clicked()
    {
        *instrument = Some(default_instrument.clone());
        *editable = false;
        sync.notify_rom_changed();
    };
}
