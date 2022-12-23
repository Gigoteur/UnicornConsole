use std::collections::VecDeque;

use eframe::{
    egui::{
        plot::{Line, Plot, PlotPoints},
        Ui, Window,
    },
    epaint::{Color32, Vec2},
};
use gamercade_audio::SFX_CHANNELS;
use gamercade_sound_engine::SoundOutputChannels;
use rtrb::Consumer;

// TODO: Make this configurable?
const OSCILLOSCOPE_FRAMES: usize = 1024;
const BUFFER_LENGTH: usize = OSCILLOSCOPE_FRAMES * 4;

#[derive(Default, PartialEq, Eq)]
pub(crate) enum OscilloscopeMode {
    #[default]
    Off,
    Channels,
    Master,
}

pub(crate) struct Oscilloscope {
    pub(crate) open: bool,
    pub(crate) mode: OscilloscopeMode,
    buffer: VecDeque<SoundOutputChannels>,
    pub(crate) channel_outputs: Consumer<SoundOutputChannels>,
    // TODO: We need to convert this to unique plots for each channel
    // in order to support "channels" view
    points: Vec<f64>,
    next_points: Vec<f64>,
}

// struct ScopePointBuffer<const N: usize> {
//     points: Vec<[f32; N]>,
//     next_points: Vec<[f32; N]>,
// }

impl Oscilloscope {
    pub(crate) fn new(channel_outputs: Consumer<SoundOutputChannels>) -> Self {
        Self {
            open: false,
            buffer: VecDeque::with_capacity(BUFFER_LENGTH),
            mode: OscilloscopeMode::default(),
            channel_outputs,
            points: Vec::with_capacity(OSCILLOSCOPE_FRAMES),
            next_points: Vec::with_capacity(OSCILLOSCOPE_FRAMES),
        }
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        if self.mode == OscilloscopeMode::Off {
            self.open = false
        }

        self.buffer.clear();

        while let Ok(frame) = self.channel_outputs.pop() {
            if self.buffer.len() < BUFFER_LENGTH {
                self.buffer.push_back(frame);
            } else {
                break;
            }
        }

        if self.next_points.len() == OSCILLOSCOPE_FRAMES {
            self.points = std::mem::take(&mut self.next_points);

            //Find the zero cross
            while let (Some(prev), Some(next)) = (self.buffer.get(0), self.buffer.get(1)) {
                let prev = prev.get_sfx_output() + prev.get_bgm_output();
                let next = next.get_sfx_output() + next.get_bgm_output();
                if prev < 0.0 && next > 0.0 {
                    self.next_points.push(next as f64);
                    break;
                } else {
                    self.buffer.pop_front();
                }
            }
        }

        (self.next_points.len()..OSCILLOSCOPE_FRAMES).for_each(|_| {
            if let Some(next_frame) = self.buffer.pop_front() {
                let value = next_frame.get_sfx_output() + next_frame.get_bgm_output();
                self.next_points.push(value as f64)
            }
        });

        let max_wave_height = SFX_CHANNELS as f32;

        let ctx = ui.ctx();
        ctx.request_repaint();

        Window::new("Oscilloscope")
            .open(&mut self.open)
            .collapsible(false)
            .show(ctx, |ui| {
                let plot = Plot::new("oscilloscope")
                    .allow_boxed_zoom(false)
                    .allow_drag(false)
                    .allow_scroll(false)
                    .allow_zoom(false)
                    .show_axes([false, false])
                    .include_x(OSCILLOSCOPE_FRAMES as f64)
                    .include_x(0)
                    .include_y(max_wave_height)
                    .include_y(-max_wave_height)
                    .set_margin_fraction(Vec2::ZERO);

                plot.show(ui, |plot_ui| {
                    let points: PlotPoints = self
                        .points
                        .iter()
                        .enumerate()
                        .map(|(index, val)| [index as f64, *val])
                        .collect();
                    let line = Line::new(points).color(Color32::WHITE);

                    plot_ui.line(line);
                })
            });
    }
}
