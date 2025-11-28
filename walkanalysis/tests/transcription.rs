use std::path::{Path, PathBuf};

use walkanalysis::{
    exercise::{analysis::Analysis, arpeggios_up::ArpeggiosUp, Exercise},
    form::form::Form,
    transcribe::transcribe::{save_beat_data, Transcription, DEFAULT_SETTINGS},
};

#[test]
fn test_transcription() {
    let autumn_leaves_path = Path::new("tests/data/audio/autumn_leaves.wav");
    let (transcription, beat_data) =
        Transcription::transcribe_from_wav(autumn_leaves_path, 110., DEFAULT_SETTINGS).unwrap();

    let mut autumn_leaves_beat_data_path = PathBuf::from(autumn_leaves_path);
    autumn_leaves_beat_data_path.set_extension("beat_data");
    save_beat_data(&beat_data, &autumn_leaves_beat_data_path).unwrap();

    let form = Form::open("tests/data/forms/autumn_leaves.json").unwrap();

    let analysis = Analysis::analyze(transcription, &form);

    let mut exercise = ArpeggiosUp {};

    println!("{}", exercise.correct(analysis));
}
