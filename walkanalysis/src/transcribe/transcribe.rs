use std::{
    error::Error,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rustfft::{num_complex::Complex, FftPlanner};
use serde::{Deserialize, Serialize};

use crate::form::note::Note;

const POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT: f32 = 0.13;

// Look at these portions of the beat to determine the note
const START_OFFSET: f64 = 0.05;
const END_OFFSET: f64 = 0.60;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct TranscriptionData {
    beat_data: Vec<BeatData>,
    sample_rate: u32,
}

impl TranscriptionData {
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        // Save all the beat data for debugging purposes
        let mut file = File::create(PathBuf::from(path))?;
        file.write_all(serde_json::to_string(&self)?.as_bytes())?;

        Ok(())
    }
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
    ) -> Result<(Self, TranscriptionData), Box<dyn Error>> {
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
    ) -> (Self, TranscriptionData) {
        let samples_per_second = audio_settings.sample_rate as f64;
        let beats_per_second = tempo as f64 / 60.;
        let samples_per_beat = (samples_per_second / beats_per_second).round() as usize;

        let samples_split_per_beat = samples.chunks_exact(samples_per_beat);

        let start_position = (START_OFFSET * samples_per_beat as f64).round() as usize;
        let end_position = (END_OFFSET * samples_per_beat as f64).round() as usize;
        let window_len = end_position - start_position;
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(window_len);

        let mut transcription_data = TranscriptionData {
            beat_data: Vec::new(),
            sample_rate: audio_settings.sample_rate,
        };

        for (beat_number, beat) in samples_split_per_beat.enumerate() {
            let relevant_samples = &beat[start_position..end_position];
            let upsampled = relevant_samples;
            let sample_rate = audio_settings.sample_rate;

            let mut buffer: Vec<Complex<f32>> = upsampled
                .iter()
                .map(|&x| Complex::new(x as f32, 0.0))
                .collect();

            fft.process(&mut buffer);

            let dc_component = buffer.get_mut(0).unwrap();
            *dc_component = Complex::new(0., 0.); // Get rid of DC part

            let half = buffer.len() / 2;
            let (max_idx, max_mag) = buffer
                .iter()
                .take(half)
                .map(|b| b.norm_sqr())
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap();

            let dominant_freq_hz = max_idx as f32 * sample_rate as f32 / buffer.len() as f32;

            // Dominant frequency is usually not the root, take every peakt that's over some percentage of the height of dominant,
            // the lowest frequency there is the root
            let possible_roots = buffer
                .iter()
                .take(half)
                .enumerate()
                .filter(|(_i, bin)| {
                    bin.norm_sqr() > POSSIBLE_ROOT_RELATIVE_HEIGHT_TO_DOMINANT * max_mag
                })
                .collect::<Vec<_>>();

            let bin_as_freq = |bin| (bin as f32) * sample_rate as f32 / buffer.len() as f32;

            let root_freq_hz: Option<f32> = possible_roots.first().and_then(|(bin, _power)| {
                if *bin == 0 {
                    println!("DC showed up as possible root, ignoring.");
                    return None; // DC, useless, and filtered out earlier
                }

                let center_bin = *bin;
                let left_bin = center_bin - 1;
                let right_bin = center_bin + 1;

                let center_power = buffer.get(center_bin)?.norm_sqr();
                let left_power = buffer.get(left_bin)?.norm_sqr();
                let right_power = buffer.get(right_bin)?.norm_sqr();

                let (other_power, other_bin, sign) = if left_power > right_power {
                    (left_power, left_bin, -1.)
                } else {
                    (right_power, right_bin, 1.)
                };

                let center_freq = bin_as_freq(center_bin);
                let other_freq = bin_as_freq(other_bin);

                let freq_diff = (center_freq - other_freq).abs();

                let freq_offset = (1.0 - (center_power / (center_power + other_power))) * freq_diff;

                Some(center_freq + freq_offset * sign)
            });

            let note = root_freq_hz.and_then(|freq| {
                let (note, error) = Note::from_frequency(freq);
                if error.abs() > 0.25 { TODO: set this very low and fix the UI bug of silence not appearing
                    // Note is too sharp or flat
                    println!(
                        "{beat_number} Found {} with large error: {error}",
                        note.flat()
                    );
                    None
                } else {
                    Some(note)
                }
            });

            transcription_data.beat_data.push(BeatData {
                number: beat_number,
                samples: Vec::from(relevant_samples),
                fft: buffer.iter().map(|c| c.norm_sqr()).collect(),
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

        for beat in transcription_data.beat_data.iter() {
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

        (Transcription { notes: result }, transcription_data)
    }
}
