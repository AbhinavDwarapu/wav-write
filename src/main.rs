use std::env;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const SAMPLE_RATE: u32 = 44100;
const DURATION_SECONDS: u32 = 2;
const MIDDLE_A: f32 = 440.0;

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

    for t in (0..SAMPLE_RATE).map(|x| x as f32 / SAMPLE_RATE as f32) {
        let sample = (t * MIDDLE_A * 2.0 * PI).sin();
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
