use std::collections::HashMap;
use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SAMPLE_RATE: u32 = 44100;
const DURATION_SECONDS: u32 = 3;

pub struct Notes;

impl Notes {
    pub fn all() -> HashMap<&'static str, f32> {
        HashMap::from([
            ("C", 261.63),
            ("D", 293.66),
            ("E", 329.63),
            ("F", 349.23),
            ("G", 392.00),
            ("A", 440.00),
            ("B", 493.88),
        ])
    }
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    let path = Path::new("generated_wave_file.wav");
    let display = path.display();

    let file = match File::create(&path) {
        Err(e) => panic!("Couldn't create file: {display}: {e}"),
        Ok(file) => file,
    };

    let num_samples = SAMPLE_RATE * DURATION_SECONDS;

    let header = build_wav_header(num_samples);
    write_header(&file, header);

    let samples = generate_samples(num_samples);
    write_samples(&file, samples);

    println!("Created file: {display}");
}

// From https://en.wikipedia.org/wiki/WAV#WAV_file_header
fn build_wav_header(num_samples: u32) -> Vec<u8> {
    let file_size = 44 + num_samples * 2; // 44 bytes for header, 2 bytes per sample

    let mut header: Vec<u8> = Vec::with_capacity(44);
    header.extend_from_slice(b"RIFF"); // FileTypeBlocID
    header.extend_from_slice(&(file_size - 8).to_le_bytes()); // FileSize
    header.extend_from_slice(b"WAVE"); // FileFormatID

    header.extend_from_slice(b"fmt "); // FormatBlocID
    header.extend_from_slice(&16u32.to_le_bytes()); // BlocSize
    header.extend_from_slice(&1u16.to_le_bytes()); // AudioFormat (PCM)
    header.extend_from_slice(&1u16.to_le_bytes()); // NbrChannels (Mono)
    header.extend_from_slice(&SAMPLE_RATE.to_le_bytes()); // Frequency
    header.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes()); // BytePerSec
    header.extend_from_slice(&2u16.to_le_bytes()); // BytePerBloc
    header.extend_from_slice(&16u16.to_le_bytes()); // BitsPerSample

    header.extend_from_slice(b"data"); // DataBlocID
    header.extend_from_slice(&(num_samples * 2).to_le_bytes()); // DataSize

    header
}

fn write_header(mut f: &File, header: Vec<u8>) {
    f.write_all(header.as_slice())
        .expect("Failed to write WAV header");
}

fn generate_samples(num_samples: u32) -> Vec<i16> {
    let mut samples: Vec<i16> = Vec::with_capacity(num_samples as usize);
    let notes = Notes::all();

    let note_duration = num_samples / 5 as u32;

    for (i, t) in (0..num_samples)
        .map(|x| x as f32 / SAMPLE_RATE as f32)
        .enumerate()
    {
        let section = i / note_duration as usize;
        let sample = match section {
            0 => (t * notes["C"] * 2.0 * PI).sin(),
            1 => (t * notes["E"] * 2.0 * PI).sin(),
            2 => (t * notes["G"] * 2.0 * PI).sin(),
            _ => {
                let c = (t * notes["C"] * 2.0 * PI).sin();
                let e = (t * notes["E"] * 2.0 * PI).sin();
                let g = (t * notes["G"] * 2.0 * PI).sin();
                (c + e + g) / 3.0 // avoid clipping
            }
        };

        let amplitude = i16::MAX as f32;
        samples.push((sample * amplitude) as i16);
    }

    samples
}

fn write_samples(mut f: &File, samples: Vec<i16>) {
    for sample in samples {
        f.write_all(&sample.to_le_bytes())
            .expect("Failed to write sample");
    }
}
