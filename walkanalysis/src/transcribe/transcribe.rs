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
#[derive(Debug, Clone)]
pub struct Transcription {
    pub notes: Vec<PlayedNote>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayedNote {
    Surely(Note),
    Unknown,
    Silence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeatData {
    number: usize,
    dominant_frequency: f32,
    root_frequency: Option<f32>,
    maximum_amplitude: f32,
    note: Option<Note>,
    human_readable_note: String,
    fft: Vec<f32>,
    samples: Vec<f32>,
}

pub fn save_beat_data(beat_data: &Vec<BeatData>, path: &Path) -> Result<(), Box<dyn Error>> {
    // Save all the beat data for debugging purposes
    let mut file = File::create(PathBuf::from(path))?;
    file.write_all(serde_json::to_string(beat_data)?.as_bytes())?;

    Ok(())
}

pub struct TranscriptionSettings {
    pub silence_threshold: f32,
}

pub struct AudioSettings {
    pub sample_rate: u32,
}

pub const DEFAULT_SETTINGS: TranscriptionSettings = TranscriptionSettings {
    silence_threshold: 200. / i32::MAX as f32,
};

impl Transcription {
    pub fn transcribe_from_wav(
        path: &Path,
        tempo: f32,
        transcription_settings: TranscriptionSettings,
    ) -> Result<(Self, Vec<BeatData>), Box<dyn Error>> {
        // open file
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        let channel0_samples: Vec<_> = reader
            .samples::<i32>()
            .step_by(spec.channels as usize)
            .map(|x| x.unwrap())
            .map(|sample_as_integer| sample_as_integer as f32 / i32::MAX as f32)
            .collect();

        Ok(Transcription::transcribe(
            &channel0_samples,
            tempo,
            transcription_settings,
            AudioSettings {
                sample_rate: spec.sample_rate,
            },
        ))
    }

    pub fn transcribe(
        samples: &[f32],
        tempo: f32,
        transcription_settings: TranscriptionSettings,
        audio_settings: AudioSettings,
    ) -> (Self, Vec<BeatData>) {
        let samples_per_second = audio_settings.sample_rate as f64;
        let beats_per_second = tempo as f64 / 60.;
        let samples_per_beat = (samples_per_second / beats_per_second).round() as usize;

        let samples_split_per_beat = samples.chunks_exact(samples_per_beat);

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

            let dominant_freq_hz =
                max_idx as f32 * audio_settings.sample_rate as f32 / buffer.len() as f32;

            // Dominant frequency is usually not the root, take every peakt that's over some percentage of the height of dominant,
            // the lowest frequency there is the root
            const POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT: f32 = 0.8;
            let possible_roots = buffer
                .iter()
                .take(half)
                .enumerate()
                .filter(|(_i, bin)| {
                    bin.norm_sqr() > POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT * max_mag
                })
                .collect::<Vec<_>>();

            let root_freq_hz = possible_roots.first().map(|(root_index, _)| {
                (*root_index as f32) * audio_settings.sample_rate as f32 / buffer.len() as f32
            });

            let note = root_freq_hz.map(|freq| Note::from_frequency(freq));

            beat_data.push(BeatData {
                number: beat_number,
                samples: Vec::from(relevant_samples),
                fft: buffer.iter().map(|c| c.norm()).collect(),
                dominant_frequency: dominant_freq_hz,
                root_frequency: root_freq_hz,
                maximum_amplitude: relevant_samples.iter().copied().reduce(f32::max).unwrap(),
                human_readable_note: note
                    .map(|n| format!("{}", n.flat()))
                    .unwrap_or("".to_string()),
                note,
            });
        }

        let mut result = vec![];

        for beat in beat_data.iter() {
            let Some(note) = beat.note else {
                result.push(PlayedNote::Unknown);
                continue;
            };

            if beat.maximum_amplitude < transcription_settings.silence_threshold {
                result.push(PlayedNote::Silence);
                continue;
            }
            // TODO: when doing the from_frequency computation we can add a deviation from perfect
            // so we have a metric to how sure we are it's this note
            result.push(PlayedNote::Surely(note));
        }

        (Transcription { notes: result }, beat_data)
    }
}
