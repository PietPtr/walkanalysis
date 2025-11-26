use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rustfft::{num_complex::Complex, FftPlanner};
use serde::{Deserialize, Serialize};

use crate::form::note::Note;

/// Given a wav file and a tempo, works out the notes that were played,
/// leaving holes where it doesn't know
pub struct Transcription {
    notes: Vec<PlayedNote>,
}

pub enum PlayedNote {
    Surely(Note),
    Options(Vec<Note>),
    Silence,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatData {
    number: usize,
    dominant_frequency: f32,
    root_frequency: f32,
    maximum_amplitude: i32,
    note: Note,
    human_readable_note: String,
    fft: Vec<f32>,
    samples: Vec<i32>,
}

pub struct TranscriptionSettings {
    pub silence_threshold: i32,
}

pub const DEFAULT_SETTINGS: TranscriptionSettings = TranscriptionSettings {
    silence_threshold: 200,
};

impl Transcription {
    pub fn transcribe(
        path: &Path,
        tempo: u32,
        settings: TranscriptionSettings,
    ) -> Result<Self, Box<dyn Error>> {
        // open file
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        // split samples per beat
        let samples_per_second = spec.sample_rate as f64;
        let beats_per_second = tempo as f64 / 60.;
        let samples_per_beat = (samples_per_second / beats_per_second).round() as usize;

        let channel0_samples: Vec<_> = reader
            .samples::<i32>()
            .step_by(spec.channels as usize)
            .map(|x| x.unwrap())
            .collect();

        let samples_split_per_beat = channel0_samples.chunks_exact(samples_per_beat);

        // Look at these portions of the beat to determine the note
        const START_OFFSET: f64 = 0.05;
        const END_OFFSET: f64 = 0.60;

        let start_position = (START_OFFSET * samples_per_beat as f64).round() as usize;
        let end_position = (END_OFFSET * samples_per_beat as f64).round() as usize;
        let window_len = end_position - start_position;
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(window_len);

        let mut beat_data = Vec::new();

        for (beat_number, beat) in samples_split_per_beat.enumerate() {
            let relevant_samples = &beat[start_position..end_position];

            let mut buffer: Vec<Complex<f32>> = relevant_samples
                .iter()
                .map(|&x| Complex::new(x as f32, 0.0))
                .collect();

            fft.process(&mut buffer);

            let half = buffer.len() / 2;
            let (max_idx, max_mag) = buffer
                .iter()
                .take(half)
                .map(|b| b.norm_sqr())
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let dominant_freq_hz = max_idx as f32 * spec.sample_rate as f32 / buffer.len() as f32;

            // Dominant frequency is usually not the root, take every peakt that's over some percentage of the height of dominant,
            // the lowest frequency there is the root
            const POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT: f32 = 0.8;
            let possible_roots = buffer
                .iter()
                .take(half)
                .enumerate()
                .filter(|(i, bin)| {
                    bin.norm_sqr() > POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT * max_mag
                })
                .collect::<Vec<_>>();

            let (root_index, _root_bin) = possible_roots.first().unwrap(); // at least the dominant frequency is in there
            let root_freq_hz = (*root_index as f32) * spec.sample_rate as f32 / buffer.len() as f32;

            let note = Note::from_frequency(root_freq_hz);

            beat_data.push(BeatData {
                number: beat_number,
                samples: Vec::from(relevant_samples),
                fft: buffer.iter().map(|c| c.norm()).collect(),
                dominant_frequency: dominant_freq_hz,
                root_frequency: root_freq_hz,
                maximum_amplitude: relevant_samples.iter().map(|s| s.abs()).max().unwrap(),
                human_readable_note: format!("{}", note.flat()),
                note,
            });
        }

        // Save all the beat data for debugging purposes
        let mut data_path = PathBuf::from(path);
        data_path.set_extension("beat_data");

        let mut file = File::create(data_path)?;
        file.write_all(serde_json::to_string(&beat_data)?.as_bytes())
            .unwrap();

        let mut result = vec![];

        for beat in beat_data {
            if beat.maximum_amplitude < settings.silence_threshold {
                result.push(PlayedNote::Silence);
                continue;
            }
            println!(
                "[{}] {:.2} {:.2} => {}",
                beat.number,
                beat.dominant_frequency,
                beat.root_frequency,
                beat.note.flat()
            );
            // TODO: when doing the from_frequency computation we can add a deviation from perfect
            // so we have a metric to how sure we are it's this note
            result.push(PlayedNote::Surely(beat.note));
        }

        Ok(Transcription { notes: result })
    }
}
