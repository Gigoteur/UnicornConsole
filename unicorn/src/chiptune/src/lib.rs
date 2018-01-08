#[macro_use]
extern crate lazy_static;
extern crate libc;
  
#[cfg(feature = "libksnd")]
pub mod chiptune {

  use std::cmp;
  use std::ffi::{CString, CStr};
  use libc::{c_int, c_ushort, c_void};

  pub mod ffi;

  #[derive(Debug, Clone, Copy)]
  pub enum ChiptuneError {
      LoadingError,
      InstructionError,
      NoteError,
  }


  #[allow(non_snake_case)]
  pub struct Chiptune {
    P: ffi::chiptune_player,
  }

  #[allow(non_snake_case)]
  pub struct ChiptuneSong {
    S: ffi::chiptune_song,
  }

  #[derive(Clone, Copy)]
  #[allow(non_snake_case)]
  pub struct ChiptuneSound {
    pub S: ffi::chiptune_sound,
  }

  pub use self::ffi::{
    CYD_PAN_CENTER, CYD_PAN_LEFT, CYD_PAN_RIGHT, CYD_CUTOFF_MAX, CYD_MAX_FX_CHANNELS, CYD_WAVE_MAX_ENTRIES, MAX_VOLUME,
    FREQ_TAB_SIZE,
    MUS_FX_ARPEGGIO,
    MUS_FX_ARPEGGIO_ABS,
    MUS_FX_SET_EXT_ARP,
    MUS_FX_PORTA_UP,
    MUS_FX_PORTA_DN,
    MUS_FX_PORTA_UP_LOG,
    MUS_FX_PORTA_DN_LOG,
    MUS_FX_SLIDE,
    MUS_FX_VIBRATO,
    MUS_FX_FADE_VOLUME,
    MUS_FX_SET_VOLUME,
    MUS_FX_LOOP_PATTERN,
    MUS_FX_SKIP_PATTERN,
    MUS_FX_EXT,
    MUS_FX_EXT_PORTA_UP,
    MUS_FX_EXT_PORTA_DN,
    MUS_FX_EXT_RETRIGGER,
    MUS_FX_EXT_FADE_VOLUME_DN,
    MUS_FX_EXT_FADE_VOLUME_UP,
    MUS_FX_EXT_NOTE_CUT,
    MUS_FX_EXT_NOTE_DELAY,
    MUS_FX_SET_SPEED,
    MUS_FX_SET_RATE,
    MUS_FX_PORTA_UP_SEMI,
    MUS_FX_PORTA_DN_SEMI,
    MUS_FX_SET_PANNING,
    MUS_FX_PAN_LEFT,
    MUS_FX_PAN_RIGHT,
    MUS_FX_FADE_GLOBAL_VOLUME,
    MUS_FX_SET_GLOBAL_VOLUME,
    MUS_FX_SET_CHANNEL_VOLUME,
    MUS_FX_CUTOFF_UP,
    MUS_FX_CUTOFF_DN,
    MUS_FX_CUTOFF_SET,
    MUS_FX_RESONANCE_SET,
    MUS_FX_FILTER_TYPE,
    MUS_FX_CUTOFF_SET_COMBINED,
    MUS_FX_BUZZ_UP,
    MUS_FX_BUZZ_DN,
    MUS_FX_BUZZ_SHAPE,
    MUS_FX_BUZZ_SET,
    MUS_FX_BUZZ_SET_SEMI,
    MUS_FX_FM_SET_MODULATION,
    MUS_FX_FM_SET_FEEDBACK,
    MUS_FX_FM_SET_HARMONIC,
    MUS_FX_FM_SET_WAVEFORM,
    MUS_FX_PW_DN,
    MUS_FX_PW_UP,
    MUS_FX_PW_SET,
    MUS_FX_SET_WAVEFORM,
    MUS_FX_SET_FXBUS,
    MUS_FX_SET_SYNCSRC,
    MUS_FX_SET_RINGSRC,
    MUS_FX_SET_WAVETABLE_ITEM,
    MUS_FX_SET_DOWNSAMPLE,
    MUS_FX_WAVETABLE_OFFSET,
    MUS_FX_CUTOFF_FINE_SET,
    MUS_FX_END,
    MUS_FX_JUMP,
    MUS_FX_LABEL,
    MUS_FX_LOOP,
    MUS_FX_TRIGGER_RELEASE,
    MUS_FX_RESTART_PROGRAM,
    MUS_FX_NOP,
  };

  #[derive(Debug, Clone)]
  pub struct ChiptuneInstruction {
    opcode: c_int,
    mask: c_int,
    name: String,
    shortname: String,
    minv: c_int,
    maxv: c_int,
  }

  impl ChiptuneInstruction {
    pub fn new(opcode: c_int, mask: c_int, name: &str, shortname: &str, minv: c_int, maxv: c_int) -> ChiptuneInstruction {
      ChiptuneInstruction{
        opcode: opcode,
        mask: mask,
        name: String::from(name),
        shortname: String::from(shortname),
        minv: minv,
        maxv: maxv,
       }
    }
  }

  lazy_static! {
      static ref NOTENAME: [&'static str; 12] = {
        let m : [&'static str; 12] = [
          "C-",
          "C#",
          "D-",
          "D#",
          "E-",
          "F-",
          "F#",
          "G-",
          "G#",
          "A-",
          "A#",
          "B-"
        ];
        m
      };

      static ref INSTRUCTION_DESC: [ChiptuneInstruction; 63] = {
          let m : [ChiptuneInstruction; 63] = [
          ChiptuneInstruction::new(MUS_FX_END, 0xffff, "Program end", "PrgEnd", 0, 0),
          ChiptuneInstruction::new(MUS_FX_NOP, 0xffff, "No operation", "Nop", 0, 0),
          ChiptuneInstruction::new(MUS_FX_JUMP, 0xff00, "Goto", "Goto", -1, -1),
          ChiptuneInstruction::new(MUS_FX_LABEL, 0xff00, "Loop begin", "Begin", 0, 0),
          ChiptuneInstruction::new(MUS_FX_LOOP, 0xff00, "Loop end", "Loop", -1, -1),
          ChiptuneInstruction::new(MUS_FX_ARPEGGIO, 0x7f00, "Set arpeggio note", "Arp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_ARPEGGIO_ABS, 0x7f00, "Set absolute arpeggio note", "AbsArp", 0, FREQ_TAB_SIZE - 1),
          ChiptuneInstruction::new(MUS_FX_SET_EXT_ARP, 0x7f00, "Set external arpeggio notes", "ExtArp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_UP, 0x7f00, "Portamento up", "PortUp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_DN, 0x7f00, "Portamento down", "PortDn", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_UP_LOG, 0x7f00, "Portamento up (curve)", "PortUpC", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_DN_LOG, 0x7f00, "Portamento down (curve)", "PortDnC", -1, -1),
          ChiptuneInstruction::new(MUS_FX_EXT_NOTE_DELAY, 0x7ff0, "Note delay", "Delay", -1, -1),
          ChiptuneInstruction::new(MUS_FX_VIBRATO, 0x7f00, "Vibrato", "Vibrato", -1, -1),
          ChiptuneInstruction::new(MUS_FX_SLIDE, 0x7f00, "Slide", "Slide", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_UP_SEMI, 0x7f00, "Portamento up (semitones)", "PortUpST", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PORTA_DN_SEMI, 0x7f00, "Portamento down (semitones)", "PortDnST", -1, -1),
          ChiptuneInstruction::new(MUS_FX_CUTOFF_UP, 0x7f00, "Filter cutoff up", "CutoffUp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_CUTOFF_DN, 0x7f00, "Filter cutoff down", "CutoffDn", -1, -1),
          ChiptuneInstruction::new(MUS_FX_CUTOFF_SET, 0x7f00, "Set filter cutoff", "Cutoff", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_CUTOFF_SET_COMBINED, 0x7f00, "Set combined cutoff", "CutoffAHX", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_RESONANCE_SET, 0x7f00, "Set filter resonance", "Resonance", 0, 3),
          ChiptuneInstruction::new(MUS_FX_FILTER_TYPE, 0x7f00, "Set filter type", "FltType", 0, 2),
          ChiptuneInstruction::new(MUS_FX_PW_DN, 0x7f00, "PW down", "PWDn", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PW_UP, 0x7f00, "PW up", "PWUp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PW_SET, 0x7f00, "Set PW", "PW", -1, -1),
          ChiptuneInstruction::new(MUS_FX_SET_VOLUME, 0x7f00, "Set volume", "Volume", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_FADE_GLOBAL_VOLUME, 0x7f00, "Global volume fade", "GlobFade", -1, -1),
          ChiptuneInstruction::new(MUS_FX_SET_GLOBAL_VOLUME, 0x7f00, "Set global volume", "GlobVol", 0, MAX_VOLUME),
          ChiptuneInstruction::new(MUS_FX_SET_CHANNEL_VOLUME, 0x7f00, "Set channel volume", "ChnVol", 0, MAX_VOLUME),
          ChiptuneInstruction::new(MUS_FX_SET_WAVEFORM, 0x7f00, "Set waveform", "Waveform", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_SET_WAVETABLE_ITEM, 0x7f00, "Set wavetable item", "Wavetable", 0, CYD_WAVE_MAX_ENTRIES - 1),
          ChiptuneInstruction::new(MUS_FX_SET_FXBUS, 0x7f00, "Set FX bus", "SetFxBus", 0, CYD_MAX_FX_CHANNELS - 1),
          ChiptuneInstruction::new(MUS_FX_SET_RINGSRC, 0x7f00, "Set ring modulation source (FF=off)", "SetRingSrc", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_SET_SYNCSRC, 0x7f00, "Set sync source (FF=off)", "SetSyncSrc", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_SET_DOWNSAMPLE, 0x7f00, "Set downsample", "SetDnSmp", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_SET_SPEED, 0x7f00, "Set speed", "Speed", -1, -1),
          ChiptuneInstruction::new(MUS_FX_SET_RATE, 0x7f00, "Set rate", "Rate", -1, -1),
          ChiptuneInstruction::new(MUS_FX_LOOP_PATTERN, 0x7f00, "Loop pattern", "PatLoop", -1, -1),
          ChiptuneInstruction::new(MUS_FX_SKIP_PATTERN, 0x7f00, "Skip pattern", "PatSkip", -1, -1),
          ChiptuneInstruction::new(MUS_FX_TRIGGER_RELEASE, 0x7f00, "Trigger release", "Release", 0, 0xff),
          ChiptuneInstruction::new(MUS_FX_RESTART_PROGRAM, 0x7f00, "Restart program", "Restart", 0, 0),
          ChiptuneInstruction::new(MUS_FX_FADE_VOLUME, 0x7f00, "Fade volume", "VolFade", -1, -1),
          ChiptuneInstruction::new(MUS_FX_EXT_FADE_VOLUME_UP, 0x7ff0, "Fine fade volume in", "VolUpFine", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_EXT_FADE_VOLUME_DN, 0x7ff0, "Fine fade volume out", "VolDnFine", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_EXT_PORTA_UP, 0x7ff0, "Fine portamento up", "PortUpFine", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_EXT_PORTA_DN, 0x7ff0, "Fine portamento down", "PortDnFine", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_EXT_NOTE_CUT, 0x7ff0, "Note cut", "NoteCut", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_EXT_RETRIGGER, 0x7ff0, "Retrigger", "Retrig", 0, 0xf),
          ChiptuneInstruction::new(MUS_FX_WAVETABLE_OFFSET, 0x7000, "Wavetable offset", "WaveOffs", 0, 0xfff),
          ChiptuneInstruction::new(MUS_FX_SET_PANNING, 0x7f00, "Set panning", "SetPan", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PAN_LEFT, 0x7f00, "Pan left", "PanLeft", -1, -1),
          ChiptuneInstruction::new(MUS_FX_PAN_RIGHT, 0x7f00, "Pan right", "PanRight", -1, -1),
          ChiptuneInstruction::new(MUS_FX_BUZZ_UP, 0x7f00, "Tune buzz up", "BuzzUp", -1, -1),
          ChiptuneInstruction::new(MUS_FX_BUZZ_DN, 0x7f00, "Tune buzz down", "BuzzDn", -1, -1),
          ChiptuneInstruction::new(MUS_FX_BUZZ_SHAPE, 0x7f00, "Set buzz shape", "BuzzShape", 0, 3),
          ChiptuneInstruction::new(MUS_FX_BUZZ_SET, 0x7f00, "Set buzz finetune", "BuzzFine", -1, -1),
          ChiptuneInstruction::new(MUS_FX_CUTOFF_FINE_SET, 0x7000, "Set filter cutoff (fine)", "CutFine", 0, CYD_CUTOFF_MAX - 1),
          ChiptuneInstruction::new(MUS_FX_BUZZ_SET_SEMI, 0x7f00, "Set buzz semitone", "BuzzSemi", -1, -1),
          ChiptuneInstruction::new(MUS_FX_FM_SET_MODULATION, 0x7f00, "Set FM modulation", "FMMod", 0, 0x7f),
          ChiptuneInstruction::new(MUS_FX_FM_SET_FEEDBACK, 0x7ff0, "Set FM feedback", "FMFB", 0, 7),
          ChiptuneInstruction::new(MUS_FX_FM_SET_HARMONIC, 0x7f00, "Set FM multiplier", "Mult", 0, 255),
          ChiptuneInstruction::new(MUS_FX_FM_SET_WAVEFORM, 0x7f00, "Set FM waveform", "FMWave", 0, 255)
          ];

          m
      };
  }

  pub fn base_note_name(note: u8) -> Result<String, ChiptuneError> {
    let note = cmp::min(cmp::max(0, note), FREQ_TAB_SIZE as u8 - 1);
    Ok(format!("{}{}", NOTENAME[(note%12) as usize], note/12))
  }

  pub fn notename(opcode: c_int, base_note: u8) -> Result<String, ChiptuneError> {
    match get_instruction(opcode as i32) {
        Ok(v) => {
          if ((opcode & 0x7f00) == MUS_FX_ARPEGGIO) || ((opcode & 0x7f00) == MUS_FX_ARPEGGIO_ABS) {
            if (opcode & 0xff) != 0xf0 && (opcode & 0xff) != 0xf1 {
              let mut note = 0;
              if (opcode & 0x7f00) == MUS_FX_ARPEGGIO {
                note = base_note as i32 + (opcode & 0xff);
              }
              return Ok(format!("{}", base_note_name(note as u8).unwrap()));
            } else {
              return Ok(format!("EXT{:x}", opcode & 0x0f));
            }
          }

          return Ok(v.shortname.clone());
        },
        Err(e) => println!("Error {:?}", e),
    }

    Err(ChiptuneError::NoteError)
  }

  pub fn get_instruction(opcode: c_int) -> Result<ChiptuneInstruction, ChiptuneError> {
    for instruction in INSTRUCTION_DESC.iter() {
      if instruction.opcode == (opcode & instruction.mask) {
        return Ok(instruction.clone());
      }
    }
    Err(ChiptuneError::InstructionError)
  }

  impl Chiptune {
    pub fn new() -> Chiptune {
      unsafe {
        Chiptune { P: ffi::Chiptune_CreatePlayer(44100) }
      }
    }

    pub fn load_music(&mut self, path: String) -> Result<ChiptuneSong, ChiptuneError> {
      unsafe {
        let path = CString::new(path).unwrap();
        let s = ffi::Chiptune_LoadMusic(self.P, path.as_ptr());
        if s.is_null() {
          Err(ChiptuneError::LoadingError)
        } else {
          Ok(ChiptuneSong { S: s })
        }
      }
    }

    pub fn load_music_from_memory(&mut self, data: Vec<u8>) -> Result<ChiptuneSong, ChiptuneError> {
      unsafe {
        let data_size = data.len() as i32;
        let data = CString::from_vec_unchecked(data);
        
        let s = ffi::Chiptune_LoadMusicFromMemory(self.P, data.as_ptr() as *mut c_void, data_size);
        if s.is_null() {
          Err(ChiptuneError::LoadingError)
        } else {
          Ok(ChiptuneSong { S: s })
        }
      }
    }

    pub fn play_music(&mut self, song: &mut ChiptuneSong, start_position: c_int) {
      unsafe {
        ffi::Chiptune_PlayMusic(self.P, song.S, start_position);
      }
    }

    pub fn load_sound(&mut self, path: String) -> Result<ChiptuneSound, ChiptuneError> {
      unsafe {
        let path = CString::new(path).unwrap();
        let s = ffi::Chiptune_LoadSound(self.P, path.as_ptr());
        if s.is_null() {
          Err(ChiptuneError::LoadingError)
        } else {
          Ok(ChiptuneSound { S: s })
        }
      }
    }

    pub fn new_music(&mut self, name: String) -> Result<ChiptuneSong, ChiptuneError> {
      unsafe {
          let name = CString::new(name).unwrap();
          let s = ffi::Chiptune_NewMusic(self.P, name.as_ptr());
          if s.is_null() {
            Err(ChiptuneError::LoadingError)
          } else {
            Ok(ChiptuneSong { S: s })
          }
        }
    }

    pub fn new_sound(&mut self, name: String) -> Result<ChiptuneSound, ChiptuneError> {
      unsafe {
          let name = CString::new(name).unwrap();
          let s = ffi::Chiptune_NewSound(self.P, name.as_ptr());
          if s.is_null() {
            Err(ChiptuneError::LoadingError)
          } else {
            Ok(ChiptuneSound { S: s })
          }
        }
    }

    pub fn load_sound_from_memory(&mut self, data: Vec<u8>) -> Result<ChiptuneSound, ChiptuneError> {
      unsafe {
        let data_size = data.len() as i32;
        let data = CString::from_vec_unchecked(data);
        
        let s = ffi::Chiptune_LoadSoundFromMemory(self.P, data.as_ptr() as *mut c_void, data_size) as *mut ffi::MusInstrument;
        if s.is_null() {
          Err(ChiptuneError::LoadingError)
        } else {
          Ok(ChiptuneSound { S: s })
        }
      }
    }

    pub fn play_sound(&mut self, sound: &mut ChiptuneSound, chan: c_int, note: c_ushort, panning: c_int, rate: c_int) -> c_int {
      unsafe {
        return ffi::Chiptune_PlaySound(self.P, sound.S, chan, note, panning, rate);
      }
    }

    pub fn get_num_songs(&mut self, song: &mut ChiptuneSong) -> c_int {
      unsafe {
        (*song.S).num_instruments as i32
      }
    }

    pub fn get_song(&mut self, song: &mut ChiptuneSong, idx: c_int) -> Result<ChiptuneSound, ChiptuneError> {
      unsafe {
        let s = ffi::Chiptune_GetInstrument(song.S, idx);
        if s.is_null() {
          Err(ChiptuneError::LoadingError)
        } else {
          Ok(ChiptuneSound { S: s })
        }
      }
    }

    pub fn set_drum(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).flags ^= ffi::MUS_INST_DRUM as u32;
      }
    }

    pub fn get_drum(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).flags & ffi::MUS_INST_DRUM as u32) != 0;
      }
    }

    pub fn get_name(&mut self, sound: ChiptuneSound) -> String {
      unsafe {
          
          let c_str: &CStr = CStr::from_ptr((*sound.S).name.as_ptr());
          let str_slice: &str = c_str.to_str().unwrap();
          let str_buf: String = str_slice.to_owned();

          return str_buf;
      }
    }


    pub fn set_pulse(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).cydflags ^= ffi::CYD_CHN_ENABLE_PULSE as u32;
      }
    }

    pub fn get_pulse(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).cydflags & ffi::CYD_CHN_ENABLE_PULSE as u32) != 0;
      }
    }

    pub fn set_saw(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).cydflags ^= ffi::CYD_CHN_ENABLE_SAW as u32;
      }
    }

    pub fn get_saw(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).cydflags & ffi::CYD_CHN_ENABLE_SAW as u32) != 0;
      }
    }

    pub fn set_metal(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).cydflags ^= ffi::CYD_CHN_ENABLE_METAL as u32;
      }
    }

    pub fn get_metal(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).cydflags & ffi::CYD_CHN_ENABLE_METAL as u32) != 0;
      }
    }

    pub fn set_noise(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).cydflags ^= ffi::CYD_CHN_ENABLE_NOISE as u32;
      }
    }

    pub fn get_noise(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).cydflags & ffi::CYD_CHN_ENABLE_NOISE as u32) != 0;
      }
    }

    pub fn set_tri(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).cydflags ^= ffi::CYD_CHN_ENABLE_TRIANGLE as u32;
      }
    }

    pub fn get_tri(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).cydflags & ffi::CYD_CHN_ENABLE_TRIANGLE as u32) != 0;
      }
    }

    pub fn set_vib(&mut self, sound: ChiptuneSound) {
      unsafe {
        (*sound.S).flags ^= ffi::MUS_INST_INVERT_VIBRATO_BIT as u32;
      }
    }

    pub fn get_vib(&mut self, sound: ChiptuneSound) -> bool {
      unsafe {
        return ((*sound.S).flags & ffi::MUS_INST_INVERT_VIBRATO_BIT as u32) != 0;
      }
    }
    
    pub fn stop(&mut self) {
     unsafe {
        ffi::Chiptune_Stop(self.P);
      }
    }

    pub fn stop_chan(&mut self, chan: c_int) {
     unsafe {
        ffi::Chiptune_StopChan(self.P, chan);
      }
    }

    pub fn pause(&mut self, state: c_int) {
      unsafe {
        ffi::Chiptune_Pause(self.P, state);
      }
    }

    pub fn set_player_quality(&mut self, oversample: c_int) {
      unsafe {
        ffi::Chiptune_SetPlayerQuality(self.P, oversample);
      }
    }

    pub fn set_volume(&mut self, volume: c_int) {
      unsafe {
        ffi::Chiptune_SetVolume(self.P, volume);
      }
    }

    pub fn set_looping(&mut self, looping: c_int) {
      unsafe {
        ffi::Chiptune_SetLooping(self.P, looping);
      }
    }

    pub fn get_music_position(&mut self) -> c_int {
      unsafe {
        return ffi::Chiptune_GetMusicPlayPosition(self.P);
      }
    }

    pub fn get_sound_position(&mut self, chan: c_int) -> c_int {
      unsafe {
        return ffi::Chiptune_GetSoundPlayPosition(self.P, chan);
      }
    }

    pub fn set_sound_program(&mut self, sound: ChiptuneSound, value: u16, position: u32) -> bool {
      if position > ffi::MUS_PROG_LEN as u32 {
        return false;
      }
      
      unsafe {
          (*sound.S).program[position as usize] = value;
      }

      true
    }

    pub fn get_sound_program(&mut self, sound: ChiptuneSound) -> [u16; 32] {
      let mut program: [u16; 32] = [0; 32];
      
      unsafe {
          for i in 0..32 {
            program[i] = (*sound.S).program[i];
          }
      }
      program
    }

    pub fn set_base_note(&mut self, sound: ChiptuneSound, note: u8) {
      unsafe {
        (*sound.S).base_note = note;
      }
    }

    pub fn get_base_note(&mut self, sound: ChiptuneSound) -> u8 {
      unsafe {
        (*sound.S).base_note
      }
    }
    
    pub fn get_attack(&mut self, sound: ChiptuneSound) -> u8 {
      unsafe {
        (*sound.S).musadsr_a
      }
    }

    pub fn set_attack(&mut self, sound: ChiptuneSound, value: u8) {
      unsafe {
        (*sound.S).musadsr_a = value;
      }
    }

    pub fn get_decay(&mut self, sound: ChiptuneSound) -> u8 {
      unsafe {
        (*sound.S).musadsr_d
      }
    }

    pub fn set_decay(&mut self, sound: ChiptuneSound, value: u8) {
      unsafe {
        (*sound.S).musadsr_d = value;
      }
    }
  }
}

#[cfg(not(feature = "libksnd"))]
pub mod chiptune {
  #[derive(Debug, Clone, Copy)]
  pub enum ChiptuneError {
  }

  pub struct Chiptune {

  }

  impl Chiptune {
    pub fn new() -> Chiptune {
      Chiptune {

      }
    }

    pub fn pause(&mut self, state: i32) {
    }

    pub fn stop(&mut self) {
    }

    pub fn stop_chan(&mut self, chan: i32) {
    }

    pub fn new_sound(&mut self, name: String) -> Result<ChiptuneSound, ChiptuneError> {
      Ok(ChiptuneSound{})
    }
  
    pub fn load_sound(&mut self, path: String) -> Result<ChiptuneSound, ChiptuneError> {
      Ok(ChiptuneSound{})
    }

    pub fn load_sound_from_memory(&mut self, data: Vec<u8>) -> Result<ChiptuneSound, ChiptuneError> {
      Ok(ChiptuneSound{})
    }

    pub fn load_music(&mut self, path: String) -> Result<ChiptuneSong, ChiptuneError> {
      Ok(ChiptuneSong{})
    }

    pub fn get_num_songs(&mut self, song: &mut ChiptuneSong) -> i32 {
      0
    }

    pub fn get_song(&mut self, song: &mut ChiptuneSong, idx: i32) -> Result<ChiptuneSound, ChiptuneError> {
      Ok(ChiptuneSound{})
    }

    pub fn play_sound(&mut self, sound: &mut ChiptuneSound, chan: i32, note: u16, panning: i32, rate: i32) -> i32 {
      0
    }

    pub fn play_music(&mut self, song: &mut ChiptuneSong, start_position: i32) {
    }

    pub fn get_name(&mut self, sound: ChiptuneSound) -> String {
      "".to_string()
    }

    pub fn set_volume(&mut self, volume: i32) {
    }

    pub fn set_looping(&mut self, looping: i32) {
    }

    pub fn get_music_position(&mut self) -> i32 {
      0
    }
  
  }

  #[derive(Clone, Copy)]
  pub struct ChiptuneSong {
  }

  #[derive(Clone, Copy)]
  pub struct ChiptuneSound {
  }
}