use std::path::{Path, PathBuf};

use walkanalysis::{
    analysis::{analysis::Analysis, correction::Correction},
    exercise::{arpeggios_up::ArpeggiosUp, two_beat::two_beat_thirds, Exercise},
    form::{
        form::Form,
        songs::{autumn_leaves::autumn_leaves, test::longer_test},
    },
    transcribe::transcribe::{Transcription, DEFAULT_SETTINGS},
};

fn test_transcription(form: Form, wav: &str, mut exercise: Box<dyn Exercise>) -> Correction {
    let wav = Path::new(wav);
    let (transcription, data) =
        Transcription::transcribe_from_wav(&wav, 110., DEFAULT_SETTINGS).unwrap();

    let mut autumn_leaves_beat_data_path = PathBuf::from(wav);
    autumn_leaves_beat_data_path.set_extension("beat_data");
    data.save(&autumn_leaves_beat_data_path).unwrap();

    let analysis = Analysis::analyze(transcription, &form);

    exercise.correct(&analysis)
}

#[test]
fn test_autumn_leaves() {
    let correction = test_transcription(
        autumn_leaves(),
        "tests/data/audio/autumn_leaves.wav",
        Box::new(ArpeggiosUp {}),
    );
    println!("{}", correction);
}

#[test]
fn test_first_bit_of_autumn_leaves() {
    let correction = test_transcription(
        longer_test(),
        "tests/data/audio/autumn_leaves.wav",
        Box::new(ArpeggiosUp {}),
    );
    println!("{}", correction);
}

#[test]
fn test_longer_test() {
    let correction = test_transcription(
        longer_test(),
        "tests/data/audio/longer_test_twobeat_thirds.wav",
        Box::new(two_beat_thirds()),
    );
    println!("{}", correction);
}

#[test]
fn test_longer_test_arpeggios_up() {
    let correction = test_transcription(
        longer_test(),
        "tests/data/audio/longer_test_arpeggios_up.wav",
        Box::new(ArpeggiosUp {}),
    );
    println!("{}", correction);
}

#[test]
fn test_longer_test_arpeggios_up2() {
    let correction = test_transcription(
        longer_test(),
        "tests/data/audio/longer_test_arpeggios_up_2.wav",
        Box::new(ArpeggiosUp {}),
    );
    println!("{}", correction);
}
