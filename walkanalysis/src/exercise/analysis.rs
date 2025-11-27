use std::collections::HashMap;

use crate::{
    form::{
        chord::{self, Chord},
        form::{Form, FormPiece},
        key,
        note::Note,
    },
    transcribe::transcribe::{PlayedNote, Transcription},
};

#[derive(Debug, Clone)]
pub struct Analysis {
    /// Maps beats to a note analysis
    pub beat_analysis: HashMap<u32, (FormPiece, NoteAnalysis)>,
    pub form_analysis: Vec<(FormPiece, Vec<NoteAnalysis>)>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoteAnalysis {
    Silence,
    Note {
        note: Note,
        degree_in_key: Option<key::Degree>,
        role_in_chord: chord::ChordTone,
    },
    NoteDuringSilence {
        note: Note,
    },
}

impl Analysis {
    /// Analyzes the roles of the notes played according to the transcription
    /// in the key and chord context of the given form
    pub fn analyze(transcription: Transcription, form: Form) -> Analysis {
        let mut beat_analysis = HashMap::new();
        let mut form_analysis = vec![];

        let mut key = None;
        let mut note_iter = transcription.notes.iter();

        let mut beat_number = 0;
        for form_piece in form.music.iter() {
            if let FormPiece::Key(new_key) = form_piece {
                key = Some(new_key)
            }

            let notes_in_this_form_piece: Vec<_> = note_iter
                .by_ref()
                .take(form_piece.length_in_beats())
                .collect();

            let analyze_with_chord = |note, chord: &Chord| NoteAnalysis::Note {
                note,
                degree_in_key: key.map(|k| k.role(note)),
                role_in_chord: chord.role(note),
            };

            let analyses = notes_in_this_form_piece
                .iter()
                .map(|&&note| match note {
                    PlayedNote::Surely(note) => match form_piece {
                        FormPiece::Key(_) => unreachable!(),
                        FormPiece::CountInBar => NoteAnalysis::NoteDuringSilence { note },
                        FormPiece::ChordBar(chord) => analyze_with_chord(note, chord),
                        FormPiece::HalfBar(chord) => analyze_with_chord(note, chord),
                    },
                    PlayedNote::Silence => match form_piece {
                        FormPiece::Key(_) => unreachable!(),
                        FormPiece::CountInBar => NoteAnalysis::Silence,
                        FormPiece::ChordBar(_) => NoteAnalysis::Silence,
                        FormPiece::HalfBar(_) => NoteAnalysis::Silence,
                    },
                })
                .collect::<Vec<_>>();

            form_analysis.push((form_piece.clone(), analyses.clone()));

            for analysis in analyses.into_iter() {
                beat_analysis.insert(beat_number, (form_piece.clone(), analysis));
                beat_number += 1;
            }
        }

        Self {
            beat_analysis,
            form_analysis,
        }
    }
}
