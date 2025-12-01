use std::collections::HashMap;

use crate::{
    analysis::{
        analysis::{Analysis, NoteAnalysis},
        correction::Correction,
        mistake::{Mistake, MistakeKind},
    },
    form::{
        chord::{Chord, ChordTone},
        form::FormPiece,
        key::Degree,
        note::Note,
    },
};

pub mod arpeggios_up;
pub mod chord_tones;

pub trait Exercise {
    /// User facing explanation about what must be played for a perfect score
    fn explain(&self) -> String;
    /// Correction function: given an analysis of a transcription, how many beats fit
    /// both the form and the exercise?
    fn correct(&mut self, analysis: &Analysis) -> Correction;
}

pub fn common_mistakes<'a>(
    mistakes: &mut HashMap<u32, Mistake>,
    beat: u32,
    form_piece: &'a FormPiece,
    note_analysis: NoteAnalysis,
) -> Option<(Note, Degree, ChordTone, &'a Chord)> {
    let mut is_note_or_mistake = || {
        let NoteAnalysis::Note {
            note,
            degree_in_key: _degree_in_key,
            role_in_chord,
        } = note_analysis
        else {
            mistakes.insert(
                beat,
                Mistake {
                    beat: beat,
                    mistake: MistakeKind::ExpectedNote {
                        found: note_analysis,
                    },
                },
            );
            return None;
        };

        Some((note, _degree_in_key, role_in_chord))
    };

    match form_piece {
        FormPiece::Key(_) => None,
        FormPiece::CountOff => {
            if note_analysis != NoteAnalysis::Silence {
                mistakes.insert(
                    beat,
                    Mistake {
                        beat,
                        mistake: MistakeKind::ExpectedSilence {
                            found: note_analysis,
                        },
                    },
                );
            };
            None
        }
        FormPiece::ChordBar(chord) => is_note_or_mistake().map(|(n, d, c)| (n, d, c, chord)),
        FormPiece::HalfBar(chord1, chord2) => {
            let active_chord = match beat % 4 {
                0 | 1 => chord1,
                2 | 3 => chord2,
                _ => unreachable!(),
            };

            is_note_or_mistake().map(|(n, d, c)| (n, d, c, active_chord))
        }
        _ => None,
    }
}
