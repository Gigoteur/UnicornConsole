mod input;
mod utils;
mod buffer;
mod keyboard;
mod keymap;
mod view;
mod log;
mod modes;
mod overlay;
mod command;
mod textobject;
mod iterators;

use std::path::PathBuf;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;

#[cfg(feature = "syntect")]
use syntect::highlighting::ThemeSet;
#[cfg(feature = "syntect")]
use syntect::parsing::SyntaxSet;

use unicorn::UnicornConfig;
use unicorn::edit::edit::State;

use gfx::Screen;
use config::Players;

use editor::text_editor::input::Input;
use editor::text_editor::keyboard::Key;
use editor::text_editor::view::View;
use editor::text_editor::modes::{Mode, ModeType, InsertMode, NormalMode, StandardMode};
use editor::text_editor::overlay::{Overlay, OverlayEvent};
use editor::text_editor::buffer::Buffer;
use editor::text_editor::command::Command;
use editor::text_editor::command::{Action, BuilderEvent, Operation, Instruction};
use unicorn::Palettes;


pub struct TextEditor {
    editor: Editor,
}

impl TextEditor {
    pub fn new(_state: Arc<Mutex<State>>) -> TextEditor {
        let options = Options { syntax_enabled: true };

        TextEditor {
            editor: Editor::new(Input::Code("".to_string()),
                                Box::new(StandardMode::new()),
                                options),
        }
    }

    pub fn init(&mut self,
                _config: Arc<Mutex<UnicornConfig>>,
                _screen: &mut Screen,
                filename: String,
                code: String) {
        info!("[EDITOR][TXT] Init");

        self.editor.reset();
        self.editor.set_buffer(filename.clone(), code.clone());
    }

    pub fn update(&mut self, _players: Arc<Mutex<Players>>) -> bool {
        true
    }

    pub fn draw(&mut self,
                players: Arc<Mutex<Players>>,
                palettes: Arc<Mutex<Palettes>>,
                screen: &mut Screen) {
        self.editor.render(screen, palettes.clone(), players.clone());
    }

    pub fn get_buffer(&mut self) -> Vec<String> {
        self.editor.get_buffer()
    }
}


pub struct Options {
    pub syntax_enabled: bool,
}

impl Default for Options {
    fn default() -> Options {
        Options { syntax_enabled: false }
    }
}


/// The main Editor structure
///
/// This is the top-most structure in Iota.
pub struct Editor {
    buffers: Vec<Arc<Mutex<Buffer>>>,
    view: View,
    running: bool,
    mode: Box<dyn Mode>,
    options: Options,

    command_queue: Receiver<Command>,
    command_sender: Sender<Command>,
}

impl Editor {
    /// Create a new Editor instance from the given source
    #[cfg(not(feature = "syntect"))]
    pub fn new(source: Input, mode: Box<dyn Mode>, opts: Options) -> Editor {
        let (snd, recv) = channel();

        let mut buffers = Vec::new();

        let buffer = match source {
            Input::Code(data) => Buffer::new_raw(data),
            Input::Stdin(reader) => Buffer::from(reader),
        };

        buffers.push(Arc::new(Mutex::new(buffer)));

        let view = View::new(buffers[0].clone(), 133, 34);

        Editor {
            buffers: buffers,
            view: view,
            running: true,

            mode: mode,
            options: opts,

            command_queue: recv,
            command_sender: snd,
        }
    }

    #[cfg(feature = "syntect")]
    pub fn new(source: Input, mode: Box<Mode>, opts: Options) -> Editor {
        let (snd, recv) = channel();

        let mut buffers = Vec::new();


        let mut ps = SyntaxSet::load_defaults_nonewlines();

        let buffer = match source {
            Input::Code(data) => Buffer::new_with_syntax_raw(data, &ps),
            Input::Stdin(reader) => Buffer::from(reader),
        };

        buffers.push(Arc::new(Mutex::new(buffer)));

        // NOTE: this will only work on linux
        // TODO: make this more cross-platform friendly
        let mut subl_config = env::home_dir().unwrap();
        subl_config.push(".config/sublime-text-3/Packages/Base16/");

        let (theme_name, ts) = if subl_config.exists() {
            (String::from("base16-default-dark"),
             Rc::new(ThemeSet::load_from_folder(subl_config).unwrap()))
        } else {
            (String::from("base16-eighties.dark"), Rc::new(ThemeSet::load_defaults()))
        };

        let view = View::new(buffers[0].clone(), ts.clone(), theme_name, 133, 34);

        Editor {
            buffers: buffers,
            view: view,
            running: true,

            mode: mode,
            options: opts,

            command_queue: recv,
            command_sender: snd,
        }
    }

    pub fn reset(&mut self) {
        self.view.reset();
    }

    pub fn get_buffer(&mut self) -> Vec<String> {
        self.view.get_buffer()
    }


    #[cfg(not(feature = "syntect"))]
    pub fn set_buffer(&mut self, _filename: String, code: String) {
        let buffer = Buffer::new_raw(code);
        self.view.set_buffer(Arc::new(Mutex::new(buffer)));
    }

    #[cfg(feature = "syntect")]
    pub fn set_buffer(&mut self, filename: String, code: String) {
        let mut ps = SyntaxSet::load_defaults_nonewlines();

        let buffer = Buffer::new_with_syntax_raw(code, &ps);

        self.view.set_buffer(Arc::new(Mutex::new(buffer)));
    }
    /// Handle key events
    ///
    /// Key events can be handled in an Overlay, OR in the current Mode.
    ///
    /// If there is an active Overlay, the key event is sent there, which gives
    /// back an OverlayEvent. We then parse this OverlayEvent and determine if
    /// the Overlay is finished and can be cleared. The response from the
    /// Overlay is then converted to a Command and sent off to be handled.
    ///
    /// If there is no active Overlay, the key event is sent to the current
    /// Mode, which returns a Command which we dispatch to handle_command.
    fn handle_key_event(&mut self, rb: &mut Screen, key: Option<Key>) {
        info!("HANDLE KEY EVENT {:?}", key);

        let key = match key {
            Some(k) => k,
            None => return,
        };

        let mut remove_overlay = false;
        let command = match self.view.overlay {
            Overlay::None => self.mode.handle_key_event(key),
            _ => {
                let event = self.view.overlay.handle_key_event(key);
                match event {
                    OverlayEvent::Finished(response) => {
                        remove_overlay = true;
                        self.handle_overlay_response(rb, response)
                    }

                    _ => BuilderEvent::Incomplete,
                }
            }
        };

        if remove_overlay {
            self.view.overlay = Overlay::None;
            self.view.clear(rb);
        }

        if let BuilderEvent::Complete(c) = command {
            let _ = self.command_sender.send(c);
        }
    }

    /// Translate the response from an Overlay to a Command wrapped in a BuilderEvent
    ///
    /// In most cases, we will just want to convert the response directly to
    /// a Command, however in some cases we will want to perform other actions
    /// first, such as in the case of Overlay::SavePrompt.
    fn handle_overlay_response(&mut self,
                               rb: &mut Screen,
                               response: Option<String>)
                               -> BuilderEvent {
        // FIXME: This entire method neext to be updated
        match response {
            Some(data) => {
                match self.view.overlay {

                    // FIXME: this is just a temporary fix
                    Overlay::Prompt { ref data, .. } => {
                        match &**data {
                            // FIXME: need to find a better system for these commands
                            //        They should be chainable
                            //          ie: wq - save & quit
                            //        They should also take arguments
                            //          ie w file.txt - write buffer to file.txt
                            "q" | "quit" => BuilderEvent::Complete(Command::exit_editor()),
                            "w" | "write" => BuilderEvent::Complete(Command::save_buffer()),

                            _ => BuilderEvent::Incomplete,
                        }
                    }

                    Overlay::SavePrompt { .. } => {
                        if data.is_empty() {
                            BuilderEvent::Invalid
                        } else {
                            let path = PathBuf::from(&*data);
                            self.view.buffer.lock().unwrap().file_path = Some(path);
                            BuilderEvent::Complete(Command::save_buffer())
                        }
                    }

                    Overlay::SelectFile { .. } => {
                        let path = PathBuf::from(data);
                        let buffer = Arc::new(Mutex::new(Buffer::from(path)));
                        self.buffers.push(buffer.clone());
                        self.view.set_buffer(buffer.clone());
                        self.view.clear(rb);
                        BuilderEvent::Complete(Command::noop())
                    }

                    _ => BuilderEvent::Incomplete,
                }
            }
            None => BuilderEvent::Incomplete,
        }
    }

    /// Handle resize events
    ///
    /// width and height represent the new height of the window.
   // fn handle_resize_event(&mut self, width: usize, height: usize) {
   //     self.view.resize(width, height);
   // }

    /// Draw the current view to the frontend
    fn draw(&mut self, rb: &mut Screen, palettes: Arc<Mutex<Palettes>>) {
        self.view.draw(rb, palettes.clone(), self.options.syntax_enabled);
    }

    /// Handle the given command, performing the associated action
    fn handle_command(&mut self, rb: &mut Screen, command: Command) {
        let repeat = if command.number > 0 {
            command.number
        } else {
            1
        };
        for _ in 0..repeat {
            match command.action {
                Action::Instruction(i) => self.handle_instruction(rb, i, command),
                Action::Operation(o) => self.handle_operation(o, command),
            }
        }
    }


    fn handle_instruction(&mut self, rb: &mut Screen, instruction: Instruction, command: Command) {
        match instruction {
//            Instruction::SaveBuffer => self.view.try_save_buffer(),
            Instruction::ExitEditor => {
                if self.view.buffer_is_dirty() {
                    let _ = self.command_sender.send(Command::show_message("Unsaved changes"));
                } else {
                    self.running = false;
                }
            }
            Instruction::SetMark(mark) => {
                if let Some(object) = command.object {
                    self.view.move_mark(mark, object)
                }
            }
            Instruction::SetOverlay(overlay_type) => self.view.set_overlay(overlay_type),
            Instruction::SetMode(mode) => {
                match mode {
                    ModeType::Insert => self.mode = Box::new(InsertMode::new()),
                    ModeType::Normal => self.mode = Box::new(NormalMode::new()),
                }
            }
            Instruction::SwitchToLastBuffer => {
                self.view.switch_last_buffer();
                self.view.clear(rb);
            }
            Instruction::ShowMessage(msg) => self.view.show_message(msg),

            _ => {}
        }
    }

    fn handle_operation(&mut self, operation: Operation, command: Command) {
        match operation {
            Operation::Insert(c) => {
                for _ in 0..command.number {
                    self.view.insert_char(c)
                }
            }
            Operation::DeleteObject => {
                if let Some(obj) = command.object {
                    self.view.delete_object(obj);
                }
            }
            Operation::DeleteFromMark(m) => {
                if command.object.is_some() {
                    self.view.delete_from_mark_to_object(m, command.object.unwrap())
                }
            }
            Operation::Undo => self.view.undo(),
            Operation::Redo => self.view.redo(),
        }
    }

    pub fn render(&mut self,
                  screen: &mut Screen,
                  palettes: Arc<Mutex<Palettes>>,
                  players: Arc<Mutex<Players>>) {
        let mut players = players.lock().unwrap();

        self.draw(screen, palettes.clone());
        self.view.maybe_clear_message();
        // if players

        for (key, value) in &players.akeys_quick {
            if *value {
                let k = Key::from_special_code(*key);
                self.handle_key_event(screen, k);
            }
        }

        let text = players.get_text();
        if text.len() == 1 {
            self.handle_key_event(screen, Some(Key::Char(text.chars().next().unwrap())));
        }

        while let Ok(message) = self.command_queue.try_recv() {
            self.handle_command(screen, message)
        }

    }
}