use walkanalysis::{
    exercise::analysis::Analysis,
    form::{
        form::{bar, Form, FormPiece},
        key::{Key, Quality},
        note::*,
    },
    transcribe::transcribe::{PlayedNote, Transcription},
};

#[test]
fn test_analysis() {
    let form = Form {
        tempo: 110,
        music: vec![FormPiece::Key(Key::new(G, Quality::Minor)), bar(C.min7())],
    };

    let transcription = Transcription {
        notes: vec![
            PlayedNote::Surely(C),
            PlayedNote::Surely(E_FLAT),
            PlayedNote::Surely(G),
            PlayedNote::Surely(G_FLAT),
        ],
    };

    let analysis = Analysis::analyze(transcription, &form);

    dbg!(analysis);
}
