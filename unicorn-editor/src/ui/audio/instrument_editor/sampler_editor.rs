use eframe::{
    egui::{
        plot::{HLine, Line, Plot, PlotPoint, PlotPoints, VLine},
        Slider, Ui,
    },
    epaint::{Color32, Vec2},
};
use hound::WavReader;
use rfd::FileDialog;

use unicorn::audio::instruments::sampler::loop_mode::LoopMode;
use unicorn::audio::instruments::sampler::SampleBitDepth;
use unicorn::audio::instruments::sampler::sample_definition::SampleDefinition;


use crate::ui::AudioSyncHelper;

use super::envelope_widget::EnvelopeWidget;

#[derive(Default)]
pub struct SamplerEditor {
    points: Option<Vec<PlotPoint>>,
}

impl SamplerEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        instrument: &mut SampleDefinition,
        sync: &mut AudioSyncHelper,
    ) {
        ui.label("Sampler Editor:");

        // TODO: Maybe show information about the sample here?
        // TODO: Add "frequency Prediction" when loading a sample

        match &mut instrument.sample_frequency {
            Some(frequency) => {
                ui.label("Sample Frequency:");
                if ui.add(Slider::new(frequency, 0.0..=40_000.0)).changed() {
                    sync.notify_rom_changed();
                }

                if ui.button("Disable Sample Frequency").clicked() {
                    instrument.sample_frequency = None;
                    sync.notify_rom_changed();
                }
            }
            None => {
                if ui.button("Enable Sample Frequency").clicked() {
                    instrument.sample_frequency = Some(0.0);
                }
            }
        }

        if ui.button("Load Sample").clicked() {
            match try_load_sample(instrument) {
                Ok(_) => {
                    self.generate_sample_plot(&instrument.data);
                    sync.notify_rom_changed();
                }
                Err(e) => println!("{}", e),
            };
        }

        if self.points.is_none() {
            self.generate_sample_plot(&instrument.data);
        }

        Plot::new("Sample Plot")
            .width(1000.0)
            .height(200.0)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .set_margin_fraction(Vec2::ZERO)
            .include_y(SampleBitDepth::MAX as f64 * 1.1)
            .include_y(SampleBitDepth::MIN as f64 * 1.1)
            .show(ui, |plot_ui| {
                let len = instrument.data.len();

                let last_index = len - 1;

                let line = Line::new(PlotPoints::Owned(self.points.as_ref().unwrap().clone()))
                    .color(Color32::WHITE);

                plot_ui.line(line);

                plot_ui.hline(HLine::new(SampleBitDepth::MAX as f64).color(Color32::RED));
                plot_ui.hline(HLine::new(SampleBitDepth::MIN as f64).color(Color32::RED));
                plot_ui.vline(VLine::new(0.0).color(Color32::RED));
                plot_ui.vline(VLine::new(last_index as f64).color(Color32::RED));

                if let LoopMode::LoopRange(range) = &instrument.loop_mode {
                    plot_ui.vline(VLine::new(range.start as f64).color(Color32::BLUE));
                    plot_ui.vline(VLine::new(range.end as f64).color(Color32::BLUE));
                }
            });

        ui.horizontal(|ui| {
            let last_index = instrument.data.len().saturating_sub(1);

            ui.label("Loop Mode:");
            if ui
                .selectable_value(&mut instrument.loop_mode, LoopMode::Loop, "Loop")
                .clicked()
            {
                sync.notify_rom_changed();
            };
            if ui
                .selectable_value(&mut instrument.loop_mode, LoopMode::Oneshot, "Oneshot")
                .clicked()
            {
                sync.notify_rom_changed();
            };
            if ui
                .selectable_value(
                    &mut instrument.loop_mode,
                    LoopMode::LoopRange(0..last_index),
                    "Loop Range",
                )
                .clicked()
            {
                sync.notify_rom_changed();
            };

            if let LoopMode::LoopRange(range) = &mut instrument.loop_mode {
                ui.label("Start:");
                if ui
                    .add(Slider::new(
                        &mut range.start,
                        0..=range.end.saturating_sub(0),
                    ))
                    .changed()
                {
                    sync.notify_rom_changed()
                };

                ui.label("End:");
                if ui
                    .add(Slider::new(
                        &mut range.end,
                        (range.start + 1).min(last_index)..=last_index,
                    ))
                    .changed()
                {
                    sync.notify_rom_changed();
                }

                ui.separator();

                let start = instrument.data[range.start];
                let end = instrument.data[range.end];

                ui.label(format!("Start Value: {}", start));
                ui.label(format!("End Value: {}", end));
                ui.label(format!(
                    "Difference: {}",
                    SampleBitDepth::abs_diff(start, end)
                ));
            }
        });

        EnvelopeWidget::draw(ui, &mut instrument.envelope_definition, sync);
    }

    fn generate_sample_plot(&mut self, data: &[SampleBitDepth]) {
        self.points = Some(
            data.iter()
                .enumerate()
                .map(|(index, val)| PlotPoint::new(index as f64, *val as f64))
                .collect::<Vec<_>>(),
        );
    }
}

fn try_load_sample(instrument: &mut SampleDefinition) -> Result<(), &'static str> {
    if let Some(path) = FileDialog::new()
        .add_filter("wave (.wav)", &["wav"])
        .set_title("Load Sample")
        .pick_file()
    {
        let reader = WavReader::open(path).map_err(|_| "Failed to open file")?;

        // TODO: Handle Spec if things are the wrong setup
        let spec = reader.spec();
        let source_sample_rate = spec.sample_rate as usize;
        let data = reader
            .into_samples::<SampleBitDepth>()
            .flatten()
            .collect::<Vec<_>>()
            .into_boxed_slice();

        if data.is_empty() {
            return Err("Unable to convert sample into required i16 format.");
        }

        instrument.data = data;
        instrument.source_sample_rate = source_sample_rate;

        Ok(())
    } else {
        Err("User didn't pick a file")
    }
}
