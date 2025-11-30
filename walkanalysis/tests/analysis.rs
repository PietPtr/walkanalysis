use walkanalysis::{
    exercise::analysis::{Analysis, NoteAnalysis},
    form::{
        chord::ChordTone,
        form::{bar, Form},
        key::{self, Key, Quality},
        note::*,
    },
    transcribe::transcribe::{PlayedNote, Transcription},
};

fn assert_role(expected_degree: key::Degree, expected_role: ChordTone, na: NoteAnalysis) {
    let NoteAnalysis::Note {
        note: _,
        degree_in_key,
        role_in_chord,
    } = na
    else {
        panic!("Not noteanalysis")
    };

    assert_eq!(role_in_chord, expected_role);
    assert_eq!(expected_degree, degree_in_key);
}

#[test]
fn test_analysis() {
    let form = Form::new(110, Key::new(G, Quality::Minor).flat(), vec![bar(C.min7())]);

    let transcription = Transcription {
        notes: vec![
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Surely(C),
            PlayedNote::Surely(E_FLAT),
            PlayedNote::Surely(G),
            PlayedNote::Surely(G_FLAT),
        ],
    };

    let analysis = Analysis::analyze(transcription, &form);

    dbg!(&analysis);

    assert_role(
        key::Degree::Fourth,
        ChordTone::Root,
        analysis.beat_analysis.get(&0).unwrap().1,
    );
    assert_role(
        key::Degree::Sixth,
        ChordTone::Third,
        analysis.beat_analysis.get(&1).unwrap().1,
    );
    assert_role(
        key::Degree::First,
        ChordTone::Fifth,
        analysis.beat_analysis.get(&2).unwrap().1,
    );
    assert_role(
        key::Degree::Chromatic,
        ChordTone::NoChordTone,
        analysis.beat_analysis.get(&3).unwrap().1,
    );
}

#[test]
fn test_gmin() {
    let transcription = Transcription {
        notes: vec![
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Silence,
            PlayedNote::Surely(G),
            PlayedNote::Surely(D_FLAT),
            PlayedNote::Surely(B_FLAT),
            PlayedNote::Surely(G),
            PlayedNote::Surely(G),
            PlayedNote::Surely(B_FLAT),
            PlayedNote::Surely(D_FLAT),
            PlayedNote::Surely(F),
        ],
    };
    let form = Form::new(
        110,
        Key::new(G, Quality::Minor).flat(),
        vec![bar(G.min()), bar(G.min())],
    );

    let analysis = Analysis::analyze(transcription, &form);

    dbg!(analysis);
}
