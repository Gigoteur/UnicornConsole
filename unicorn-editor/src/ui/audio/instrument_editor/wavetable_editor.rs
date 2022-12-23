use eframe::{
    egui::{
        plot::{HLine, Line, Plot, PlotPoint, PlotPoints, VLine},
        ComboBox, Slider, Ui, Window,
    },
    epaint::{Color32, Vec2},
};
use gamercade_audio::{
    WavetableBitDepth, WavetableDefinition, WavetableGenerator, WavetableWaveform,
    WAVETABLE_MAX_LENGTH,
};

use crate::ui::AudioSyncHelper;

use super::{envelope_widget::EnvelopeWidget, interpolator_widget::InterpolatorWidget};

#[derive(Clone, Debug, Default)]
pub struct WavetableEditor {
    generator: WavetableGeneratorWidget,
}

impl WavetableEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        instrument: &mut WavetableDefinition,
        sync: &mut AudioSyncHelper,
    ) {
        self.generator.draw(ui, instrument, sync);

        let ptr = &(&instrument.interpolator as *const _ as usize);
        InterpolatorWidget::draw(ui, &mut instrument.interpolator, sync, ptr);

        let len = instrument.data.len();

        if len == 0 {
            return;
        }

        let last_index = len - 1;

        // Draw the waveform:
        let points: PlotPoints = instrument
            .data
            .iter()
            .enumerate()
            .map(|(index, val)| [index as f64, *val as f64])
            .collect();
        let line = Line::new(points).color(Color32::GREEN);

        let lerp_segment = vec![
            [last_index as f64, instrument.data[last_index] as f64],
            [(last_index + 1) as f64, instrument.data[0] as f64],
        ];
        let line_segment = Line::new(lerp_segment).color(Color32::DARK_GREEN);

        let primary_pointer_down = ui.input().pointer.primary_down();

        ui.label(&format!("Wavetable Length: {}", instrument.data.len()));
        Plot::new("Wavetable Plot")
            .width(1000.0)
            .height(200.0)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .set_margin_fraction(Vec2::ZERO)
            // .include_x(-1.0)
            // .include_x(last_index as f64 + 2.0)
            .include_y(WavetableBitDepth::MAX as f64 * 1.1)
            .include_y(WavetableBitDepth::MIN as f64 * 1.1)
            .label_formatter(move |_, point| {
                let (x, y) = plot_point_to_x_y(point, last_index);
                format!("Idx:{}\nVal:{}", x, y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.line(line_segment);

                plot_ui.hline(HLine::new(WavetableBitDepth::MAX as f64).color(Color32::RED));
                plot_ui.hline(HLine::new(WavetableBitDepth::MIN as f64).color(Color32::RED));
                plot_ui.vline(VLine::new(0.0).color(Color32::RED));
                plot_ui.vline(VLine::new(last_index as f64).color(Color32::RED));

                if plot_ui.plot_hovered() && primary_pointer_down {
                    let point = plot_ui.pointer_coordinate().unwrap();
                    let (x, y) = plot_point_to_x_y(&point, last_index);

                    // Only update if we changed the value!
                    if instrument.data[x] != y {
                        sync.notify_rom_changed();
                        instrument.data[x] = y;
                    }
                }
            });

        // TODO: Add something to add length
        if ui.button("Add Table Entry").clicked() {
            let mut new_data = instrument.data.clone().into_vec();
            new_data.push(WavetableBitDepth::default());
            instrument.data = new_data.into_boxed_slice();
            sync.notify_rom_changed();
        }

        // TODO: Add something to remove length
        if ui.button("Remove Table Entry").clicked() {
            let new_data = instrument.data[0..last_index].to_vec();
            instrument.data = new_data.into_boxed_slice();
            sync.notify_rom_changed();
        }

        // TODO: Add wavetable generator helper UI
        if ui.button("Waveform Generator").clicked() {
            self.generator.open = !self.generator.open;
        }

        EnvelopeWidget::draw(ui, &mut instrument.envelope, sync)
    }
}

fn plot_point_to_x_y(point: &PlotPoint, last_index: usize) -> (usize, WavetableBitDepth) {
    let x = (point.x.round() as usize).min(last_index);
    let y = point
        .y
        .round()
        .clamp(WavetableBitDepth::MIN as f64, WavetableBitDepth::MAX as f64)
        as WavetableBitDepth;

    (x, y)
}

#[derive(Clone, Debug, Default)]
struct WavetableGeneratorWidget {
    open: bool,
    generator: WavetableGenerator,
    duty_cycle: f32,
}

impl WavetableGeneratorWidget {
    fn draw(
        &mut self,
        ui: &mut Ui,
        instrument: &mut WavetableDefinition,
        sync: &mut AudioSyncHelper,
    ) {
        Window::new("Wavetable Generator")
            .open(&mut self.open)
            .collapsible(false)
            .show(ui.ctx(), |ui| {
                ComboBox::from_label("Waveform")
                    .selected_text(format!("{:?}", &self.generator.waveform))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Sine,
                            "Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Square,
                            "Square",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Pulse(self.duty_cycle),
                            "Pulse",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Saw,
                            "Saw",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Triangle,
                            "Triangle",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::HalfSine,
                            "Half Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::AbsoluteSine,
                            "Absolute Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::QuarterSine,
                            "Quarter Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::AlternatingSine,
                            "Alternating Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::CamelSine,
                            "Camel Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::LogarithmicSaw,
                            "Logarithmic Saw",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::InvertedSine,
                            "Inverted Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::InvertedHalfSine,
                            "Inverted Half Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::InvertedAlternatingSine,
                            "Inverted Alternating Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::InvertedCamelSine,
                            "Inverted Camel Sine",
                        );
                        ui.selectable_value(
                            &mut self.generator.waveform,
                            WavetableWaveform::Noise,
                            "Noise",
                        );
                    });

                if let WavetableWaveform::Pulse(duty_cycle) = &mut self.generator.waveform {
                    ui.add(Slider::new(duty_cycle, 0.0..=1.0).text("Pulse Duty Cycle"));
                    self.duty_cycle = *duty_cycle;
                };

                ui.add(Slider::new(
                    &mut self.generator.size,
                    1..=WAVETABLE_MAX_LENGTH,
                ));

                if ui.button("Generate").clicked() {
                    instrument.data = self.generator.generate();
                    sync.notify_rom_changed()
                }
            });
    }
}
