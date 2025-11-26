use std::path::Path;

use walkanalysis::{form, transcribe::transcribe::{DEFAULT_SETTINGS, Transcription}};

#[test]
fn test_transcription() {
    let transcription = Transcription::transcribe(
        Path::new("tests/data/audio/autumn_leaves.wav"),
        110,
        DEFAULT_SETTINGS,
    )
    .unwrap();

    TODO: write functions that can fit a transcription to a key and to a form
    for a key: need to know the role of each note and annotate that
    for a form: need to know the role of each note during each chord and annotate that
}
