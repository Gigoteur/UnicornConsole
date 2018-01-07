extern crate chiptune;
extern crate sdl2;

use std::thread;
use std::time::Duration;

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn play_sound(player: &mut chiptune::Chiptune, path: String) -> Result<chiptune::ChiptuneSound, chiptune::ChiptuneError> {
    let sound = player.load_sound(path);
    match sound {
        Ok(mut chip_sound) => {
            println!("Playing sound");
            player.play_sound(&mut chip_sound, -1, 13312, chiptune::CYD_PAN_CENTER, 50);
        }
        Err(e) => println!("ERROR {:?}", e),
    }
    sound
}

fn play_sound_from_memory(player: &mut chiptune::Chiptune, path: String) -> Result<chiptune::ChiptuneSound, chiptune::ChiptuneError> {
    let mut data = Vec::new();
    let mut f = File::open(path.as_str()).unwrap();
    f.read_to_end(&mut data).unwrap();

    println!("DATA = {:?}", data.len());
    
    let sound = player.load_sound_from_memory(data);
    match sound {
        Ok(mut chip_sound) => {
            println!("Playing sound");
            player.play_sound(&mut chip_sound, -1, 13312, chiptune::CYD_PAN_CENTER, 50);
        }
        Err(e) => println!("ERROR {:?}", e),
    }
    sound
}        

fn main() {
    sdl2::init();

    let mut player = chiptune::Chiptune::new();

    println!("Play music");
    let song = player.load_music("./src/chiptune/libksnd-source/src/assets/ringmod.kt".to_string());
    match song {
        Ok(mut chip_song) => {
            player.play_music(&mut chip_song, 0);
            ///println!("NUM INSTRUMENTS = {:?}", player.get_num_instruments(chip_song));
            for i in 0..player.get_num_instruments(&mut chip_song) {
                let instru = player.get_instrument(&mut chip_song, i).unwrap();
                println!("INSTRU {:?} {:?}", i, player.get_name(instru));
            }
        }
        Err(e) => println!("ERROR {:?}", e),
    }

    thread::sleep(Duration::from_secs(1));
    
    println!("SOUND POSITION = {:?}", player.get_sound_position(0));

    println!("Play sound");
    /*let sound = play_sound(&mut player, "./src/chiptune/libksnd-source/src/assets/sounds/major.ki".to_string());
    match sound {
        Ok(mut chip_sound) => {
            let program = player.get_sound_program(chip_sound);
            for i in 0..32 {
                println!("Program[{:?}] {:X}", i, program[i]);
                match chiptune::get_instruction(program[i] as i32) {
                    Ok(v) => {
                        //println!("{:?}", v);
                        match chiptune::notename(program[i] as i32, player.get_base_note(chip_sound)) {
                            Ok(name) => println!("NAME {:?}", name),
                            Err(_) => (),
                        }
                    },
                    Err(e) => println!("Error {:?}", e),
                }

            }
        }
        Err(e) => println!("ERROR {:?}", e),
    }
*/

    thread::sleep(Duration::from_secs(3));
    println!("CLAP");
    play_sound_from_memory(&mut player, "./src/chiptune/libksnd-source/src/assets/sounds/clap.ki".to_string());

    println!("SOUND POSITION = {:?}", player.get_sound_position(0));

    println!("MUSIC POSITION = {:?}", player.get_music_position());
    player.set_volume(64);

    thread::sleep(Duration::from_secs(10));

    println!("MUSIC POSITION = {:?}", player.get_music_position());
}