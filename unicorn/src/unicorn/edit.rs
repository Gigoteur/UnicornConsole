#[cfg(feature = "editor")]
pub mod edit {
    use editor::gfx_editor;
    use editor::music_editor;
    use editor::text_editor;
    
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    use unicorn::utils::Widget;

    use gfx::Screen;
    use config::Players;
    use sound::sound::{SoundInternal, Sound};
    use unicorn::{UnicornCartridge, UnicornConfig, Palettes};

    #[derive(Clone, Copy)]
    pub struct State {
        pub mouse_x: i32,
        pub mouse_y: i32,
        pub mouse_state: u32,
        pub mouse_statep: u32,

        pub idx_sprites_batch_x: i32,
        pub idx_sprites_batch_y: i32,
        pub idx_sprite_info: [i32; 2],
        pub idx_flag: [i32; 2],
        pub current_sprite: u32,

        pub x_zoom_sprite: u32,
        pub y_zoom_sprite: u32,
        pub zoom_sprite: u32,
        pub idx_zoom_sprite: u32,
        pub sprite_available_zooms: [u32; 4],

        pub idx_x_zoom_sprite: u32,
        pub idx_y_zoom_sprite: u32,

        pub idx_map: u32,

        pub on_current_sprite_x: u32,
        pub on_current_sprite_y: u32,
        pub on_current_sprite: bool,

        pub fill_action: bool,
    }

    impl State {
        pub fn new() -> State {
            State {
                mouse_x: 0,
                mouse_y: 0,
                mouse_state: 0,
                mouse_statep: 0,

                idx_sprites_batch_x: 0,
                idx_sprites_batch_y: 170,
                idx_sprite_info: [129, 190],

                idx_flag: [40, 160],
                current_sprite: 0,

                x_zoom_sprite: 0,
                y_zoom_sprite: 0,
                zoom_sprite: 1,
                idx_zoom_sprite: 0,
                sprite_available_zooms: [1, 2, 4, 8],

                idx_x_zoom_sprite: 10,
                idx_y_zoom_sprite: 18,

                idx_map: 0,

                on_current_sprite_x: 0,
                on_current_sprite_y: 0,
                on_current_sprite: false,

                fill_action: false,
            }
        }

        pub fn update(&mut self, players: Arc<Mutex<Players>>) {
            self.mouse_state = players.lock().unwrap().mouse_state();
            self.mouse_statep = players.lock().unwrap().mouse_state_quick();
            self.mouse_x = players.lock().unwrap().mouse_coordinate(0);
            self.mouse_y = players.lock().unwrap().mouse_coordinate(1);
        }
    }

   
    #[derive(Debug)]
    pub enum STATE {
        GfxEditor,
        TextEditor,
        MusicEditor,
    }

    pub struct Editor {
        state: Arc<Mutex<State>>,
        state_editor: STATE,
        gfx: gfx_editor::GFXEditor,
        txt: text_editor::TextEditor,
        music: music_editor::MusicEditor,
        filename: String,
        widgets: Vec<Arc<Mutex<Widget>>>,
    }

    impl Editor {
        pub fn new(screen: Arc<Mutex<Screen>>) -> Editor {
            let state = Arc::new(Mutex::new(State::new()));
            let mut highlight = HashMap::new();
            highlight.insert(6, 10);

            let mut widgets = Vec::new();
            widgets.push(Arc::new(Mutex::new(Widget::new("GFX".to_string(),
                                                         200,
                                                           1,
                                                         16,
                                                         16,
                                                         vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11,  6,  6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11,  11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                         highlight.clone(),
                                                         true, true))));
            widgets.push(Arc::new(Mutex::new(Widget::new("TEXT".to_string(),
                                                         220,
                                                         1,
                                                         16,
                                                         16,
                                                         vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                               6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                               6, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11,  6,  6,  6,  6, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                         highlight.clone(),
                                                         false, true))));

            widgets.push(Arc::new(Mutex::new(Widget::new("MUSIC".to_string(),
                                                         240,
                                                         1,
                                                         16,
                                                         16,
                                                         vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11,  6,  6,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11,  6,  6,  6,  6,  6,  6,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11,  6,  6,  6,  6,  6, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6,  6, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11, 11, 11,  6, 11, 11, 11, 11, 11, 11, 11, 11,  6, 11,  6,
                                                               6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                               6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                               6, 11,  6,  6,  6, 11, 11, 11, 11, 11, 11,  6,  6,  6, 11,  6,
                                                               6, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,  6,
                                                               6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                         highlight.clone(),
                                                         false, true))));

            Editor {
                state: state.clone(),
                state_editor: STATE::GfxEditor,
                gfx: gfx_editor::GFXEditor::new(state.clone()),
                txt: text_editor::TextEditor::new(state.clone()),
                music: music_editor::MusicEditor::new(state.clone()),
                filename: "".to_string(),
                widgets: widgets,
            }
        }

        pub fn init(&mut self, config: Arc<Mutex<UnicornConfig>>, palettes: Arc<Mutex<Palettes>>, screen: &mut Screen, filename: String, code: String) {
            info!("[EDITOR] Init {:?}", filename);
            self.filename = filename.clone();
            config.lock().unwrap().toggle_mouse(true);

            palettes.lock().unwrap().switch_to_palette("pico-8");
            screen.font("pico-8");

            self.gfx.init(config.clone(), screen);
            self.txt.init(config.clone(), screen, filename.clone(), code);
            self.music.init(config.clone(), screen);
        }

        pub fn get_code(&mut self) -> Vec<String> {
            self.txt.get_buffer()
        }

        pub fn update(&mut self, cartridge: &mut UnicornCartridge, screen: &mut Screen, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
            self.state.lock().unwrap().update(players.clone());

            let mut is_clickable = false;
            for widget in &self.widgets {
                let mouse_state = self.state.lock().unwrap().mouse_state;
                let mouse_x = self.state.lock().unwrap().mouse_x as u32;
                let mouse_y = self.state.lock().unwrap().mouse_y as u32;
                
                is_clickable = widget.lock().unwrap().is_clickable(mouse_state,
                                                                   mouse_x,
                                                                   mouse_y);
                if is_clickable {
                    break;
                }
            }

            if is_clickable {
                for widget in &self.widgets {
                    let mouse_state = self.state.lock().unwrap().mouse_state;
                    let mouse_x = self.state.lock().unwrap().mouse_x as u32;
                    let mouse_y = self.state.lock().unwrap().mouse_y as u32;

                    widget.lock().unwrap().update(mouse_state,
                                                  mouse_x,
                                                  mouse_y);
                }
            }

            for widget in &self.widgets {
                let is_click = widget.lock().unwrap().is_click();
                if is_click {
                    if widget.lock().unwrap().name == "GFX" {
                        self.state_editor = STATE::GfxEditor;
                    } else if widget.lock().unwrap().name == "TEXT" {
                        self.state_editor = STATE::TextEditor;
                    } else if widget.lock().unwrap().name == "MUSIC" {
                        self.state_editor = STATE::MusicEditor;
                    }
                }
            }

            match self.state_editor {
                STATE::GfxEditor => {
                    self.gfx.update(screen, players.clone());
                }
                STATE::TextEditor => {
                    self.txt.update(players.clone());
                }
                STATE::MusicEditor => {
                    self.music.update(cartridge, screen, players.clone(), sound_internal.clone(), sound.clone());
                }
            }

            true
        }

        pub fn draw(&mut self, players: Arc<Mutex<Players>>, palettes: Arc<Mutex<Palettes>>, screen: &mut Screen) {
            screen.cls(-1);

            let width = screen.mode_width() as i32;
            let height = screen.mode_height() as i32;

            match self.state_editor {
                STATE::GfxEditor => {
                    self.gfx.draw(players, screen);
                }
                STATE::TextEditor => {
                    self.txt.draw(players, palettes, screen);
                }
                STATE::MusicEditor => {
                    self.music.draw(players, screen);
                }
            }

            screen.rectfill(0, 0, width, 16, 11);
            screen.rectfill(0, height - 8, width, height, 11);

            // Print current filename
            screen.print(self.filename.clone(), 0, 2, 7);

            for widget in &self.widgets {
                widget.lock().unwrap().draw(screen);
            }
        }
    }
}

#[cfg(not(feature = "editor"))]
pub mod edit {
    use std::sync::{Arc, Mutex};
    use gfx::Screen;

    pub struct Editor {
    }

    impl Editor {
        pub fn new(screen: Arc<Mutex<Screen>>) -> Editor {
            Editor {}
        }
    }
}