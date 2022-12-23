use std::{process, sync::Arc, time::Duration};

use arrayvec::ArrayVec;
use gamercade_audio::{
    Chain, ChainId, EnvelopeDefinition, IndexInterpolator, InstrumentDataDefinition, InstrumentId,
    LoopMode, PatchDefinition, Phrase, PhraseId, SampleBitDepth, SampleDefinition, Song, SongId,
    SoundRom, WavetableDefinition, WavetableGenerator, WavetableWaveform,
};
use gamercade_sound_engine::{SoundEngine, SoundEngineData, SoundRomInstance};
use hound::WavReader;

const FPS: usize = 60;

pub fn main() {
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        panic_hook(panic_info);
        process::exit(1);
    }));

    let test_rom = Arc::new(test_rom());

    let mut engine = SoundEngine::new(FPS, &test_rom, 8);
    let output_sample_rate = engine.output_sample_rate();
    let mut data = SoundEngineData::new(output_sample_rate, &test_rom);

    data.play_bgm(Some(SongId(0)));
    engine.sync_audio_thread(&data);

    std::thread::sleep(Duration::from_secs_f32(20.0));
}

// Initialize our sound sources
// This isn't the intended use case, only a temporary solution until
// the editor gets audio support.
fn test_rom() -> SoundRomInstance {
    let instruments = vec![
        Some(InstrumentDataDefinition::FMSynth(PatchDefinition::default())),
        Some(InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            interpolator: gamercade_audio::IndexInterpolator::Linear,
        })),
        Some(InstrumentDataDefinition::Sampler(sampler_no_pitch())),
        Some(InstrumentDataDefinition::Sampler(sampler_pitched())),
    ];

    let mut chains0 = ArrayVec::new();
    chains0.push(Some(PhraseId(0)));
    chains0.push(Some(PhraseId(1)));

    let mut chains1 = ArrayVec::new();
    chains1.push(Some(PhraseId(0)));
    chains1.push(Some(PhraseId(0)));
    chains1.push(Some(PhraseId(2)));
    chains1.push(Some(PhraseId(2)));

    let mut chains2 = ArrayVec::new();
    chains2.push(Some(PhraseId(4)));
    chains2.push(Some(PhraseId(4)));
    chains2.push(Some(PhraseId(4)));

    let mut chains3 = ArrayVec::new();
    chains3.push(Some(PhraseId(5)));
    chains3.push(Some(PhraseId(6)));
    chains3.push(Some(PhraseId(5)));
    chains3.push(Some(PhraseId(6)));

    let songs = vec![Song {
        bpm: 120.0,
        tracks: vec![
            // FM / Wavetable synths
            [
                Some(ChainId(0)),
                Some(ChainId(1)),
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            // Sample NonPitched
            [Some(ChainId(2)), None, None, None, None, None, None, None],
            // Pitched Sample
            [Some(ChainId(3)), None, None, None, None, None, None, None],
        ]
        .into_boxed_slice(),
    }]
    .into_boxed_slice();

    let rom = SoundRom {
        songs,
        chains: vec![
            Some(Chain { entries: chains0 }),
            Some(Chain { entries: chains1 }),
            Some(Chain { entries: chains2 }),
            Some(Chain { entries: chains3 }),
        ]
        .into_boxed_slice(),
        phrases: vec![
            Some(Phrase::c_scale(InstrumentId(0))),
            Some(Phrase::c_scale_reverse(InstrumentId(0))),
            Some(Phrase::c_scale(InstrumentId(1))),
            Some(Phrase::c_scale_reverse(InstrumentId(1))),
            Some(Phrase::c_scale(InstrumentId(2))),
            Some(Phrase::c_scale(InstrumentId(3))),
            Some(Phrase::c_scale_reverse(InstrumentId(3))),
        ]
        .into_boxed_slice(),
        instruments: instruments.into_boxed_slice(),
        sfx: vec![].into_boxed_slice(),
    };

    SoundRomInstance::new(&rom)
}

fn sampler_no_pitch() -> SampleDefinition {
    let reader = WavReader::open("./gamercade_sound_engine/CantinaBand3.wav").unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let source_sample_rate = spec.sample_rate as usize;
    let data = reader
        .into_samples::<SampleBitDepth>()
        .flatten()
        .collect::<Vec<_>>();

    println!("Sampler no pitch: ");
    println!("bit depth: {:?}", spec.bits_per_sample);
    println!("sample format: {:?}", spec.sample_format);
    println!("channels: {}", channels);
    println!("source sample rate {:?}", source_sample_rate);
    println!("-----");

    SampleDefinition {
        data: data.into_boxed_slice(),
        source_sample_rate,
        sample_frequency: None,
        envelope_definition: EnvelopeDefinition::always_on(),
        interpolator: IndexInterpolator::default(),
        loop_mode: LoopMode::Loop,
    }
}

fn sampler_pitched() -> SampleDefinition {
    let reader = WavReader::open("./gamercade_sound_engine/1_piano_mid.wav").unwrap();
    let spec = reader.spec();
    let channels = spec.channels;
    let source_sample_rate = spec.sample_rate as usize;
    let data = reader
        .into_samples::<SampleBitDepth>()
        .flatten()
        .collect::<Vec<_>>();

    println!("Sampler pitched pitch: ");
    println!("bit depth: {:?}", spec.bits_per_sample);
    println!("sample format: {:?}", spec.sample_format);
    println!("channels: {}", channels);
    println!("source sample rate {:?}", source_sample_rate);
    println!("-----");

    SampleDefinition {
        data: data.into_boxed_slice(),
        source_sample_rate,
        sample_frequency: Some(523.251), //This sample is pitched to C
        envelope_definition: EnvelopeDefinition::interesting(),
        interpolator: IndexInterpolator::default(),
        loop_mode: LoopMode::Oneshot,
    }
}
