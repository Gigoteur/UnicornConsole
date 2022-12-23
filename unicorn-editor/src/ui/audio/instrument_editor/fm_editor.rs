use eframe::{
    egui::{ComboBox, Image, Slider, TextureFilter, Ui, Window},
    epaint::{ColorImage, TextureHandle, Vec2},
};
use gamercade_audio::{
    Algorithm, Detune, FMWaveform, FeedbackLevel, FrequencyMultiplier, OperatorDefinition,
    PatchDefinition, OPERATOR_COUNT,
};

use crate::ui::AudioSyncHelper;

use super::{envelope_widget::EnvelopeWidget, interpolator_widget::InterpolatorWidget};

#[derive(Clone)]
pub struct FMEditor {
    operator_widgets: [OperatorWidget; OPERATOR_COUNT],
    fm_help: FMHelp,
}

impl Default for FMEditor {
    fn default() -> Self {
        Self {
            operator_widgets: std::array::from_fn(OperatorWidget::new),
            fm_help: FMHelp::default(),
        }
    }
}

#[derive(Clone, Default)]
struct FMHelp {
    open: bool,
    diagram_texture: Option<TextureHandle>,
    diagram_size: Option<Vec2>,
}

impl FMHelp {
    fn draw(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();

        Window::new("FM Synth Help")
            .open(&mut self.open)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                let texture_id = self.diagram_texture.get_or_insert_with(|| {
                    let image = image::load_from_memory(include_bytes!("fm algorithm diagram.png"))
                        .expect("Failed to load FM Algorithm image.");
                    let size = [image.width() as usize, image.height() as usize];
                    self.diagram_size = Some(Vec2::new(size[0] as f32, size[1] as f32));
                    let image = image.to_rgba8();
                    ui.ctx().load_texture(
                        "fm algorithm diagram",
                        ColorImage::from_rgba_unmultiplied(size, &image),
                        TextureFilter::Nearest,
                    )
                });

                ui.label("Algorithm Chart:");
                ui.add(Image::new(texture_id, self.diagram_size.unwrap()));
            });
    }
}

impl FMEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        patch: &mut PatchDefinition,
        sync: &mut AudioSyncHelper,
    ) {
        self.fm_help.draw(ui);

        ui.label("FM Editor");

        if ui.button("FM Synth Help").clicked() {
            self.fm_help.open = !self.fm_help.open;
        }

        ui.horizontal(|ui| {
            ui.label("Feedback");
            if ui
                .add(Slider::new(&mut patch.feedback.0, 0..=FeedbackLevel::max()))
                .changed()
            {
                sync.notify_rom_changed();
            }

            ui.label("Algorithm");
            if ui
                .add(Slider::new(&mut patch.algorithm.0, 0..=Algorithm::max()))
                .changed()
            {
                sync.notify_rom_changed();
            }
        });

        ui.vertical(|ui| {
            self.operator_widgets.chunks_exact(2).for_each(|widget| {
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        widget[0].draw(ui, &mut patch.operators.operators[widget[0].index], sync);
                    });
                    ui.group(|ui| {
                        widget[1].draw(ui, &mut patch.operators.operators[widget[1].index], sync);
                    });
                });
            });
        });
    }
}

#[derive(Clone, Debug)]
struct OperatorWidget {
    index: usize,
}

impl OperatorWidget {
    fn new(index: usize) -> Self {
        Self { index }
    }

    fn draw(&self, ui: &mut Ui, operator: &mut OperatorDefinition, sync: &mut AudioSyncHelper) {
        let mut should_notify = false;

        ui.vertical(|ui| {
            let ptr = &(&operator.interpolator as *const _ as usize);
            InterpolatorWidget::draw(ui, &mut operator.interpolator, sync, ptr);

            ComboBox::from_label(format!("Op. {} Waveform", self.index + 1))
                .selected_text(format!("{:?}", &operator.waveform))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut operator.waveform, FMWaveform::Sine, "Sine")
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::InverseSine,
                            "InverseSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(&mut operator.waveform, FMWaveform::HalfSine, "HalfSine")
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::InverseHalfSine,
                            "InverseHalfSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::AlternatingSine,
                            "AlternatingSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::InverseAlternatingSine,
                            "InverseAlternatingSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::CamelSine,
                            "CamelSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                    if ui
                        .selectable_value(
                            &mut operator.waveform,
                            FMWaveform::InveseCamelSine,
                            "InveseCamelSine",
                        )
                        .clicked()
                    {
                        should_notify = true;
                    };
                });

            ui.group(|ui| {
                ui.label("Frequency Adjustment");
                ui.label(format!(
                    "Frequency Multiplier: {:.3}",
                    operator.frequency_multiplier.top as f32
                        / operator.frequency_multiplier.bottom as f32
                ));
                ui.label("Top");
                if ui
                    .add(Slider::new(
                        &mut operator.frequency_multiplier.top,
                        FrequencyMultiplier::min_value()..=FrequencyMultiplier::max_value(),
                    ))
                    .changed()
                {
                    should_notify = true;
                }

                ui.label("Bottom");
                if ui
                    .add(Slider::new(
                        &mut operator.frequency_multiplier.bottom,
                        FrequencyMultiplier::min_value()..=FrequencyMultiplier::max_value(),
                    ))
                    .changed()
                {
                    should_notify = true;
                }

                ui.label("Detune");
                if ui
                    .add(Slider::new(
                        &mut operator.detune.0,
                        Detune::min()..=Detune::max(),
                    ))
                    .changed()
                {
                    should_notify = true;
                };
            })
        });

        EnvelopeWidget::draw(ui, &mut operator.envlope_definition, sync);

        if should_notify {
            sync.notify_rom_changed()
        }
    }
}
