use std::path::Path;

use walkanalysis::{
    exercise::{analysis::Analysis, arpeggios_up::ArpeggiosUp, Exercise},
    form::form::Form,
    transcribe::transcribe::{Transcription, DEFAULT_SETTINGS},
};

#[test]
fn test_transcription() {
    let transcription = Transcription::transcribe(
        Path::new("tests/data/audio/autumn_leaves.wav"),
        110,
        DEFAULT_SETTINGS,
    )
    .unwrap();

    let form = Form::open("tests/data/forms/autumn_leaves.json").unwrap();

    let analysis = Analysis::analyze(transcription, form);

    let mut exercise = ArpeggiosUp {};

    println!("{}", exercise.correct(analysis));
}
