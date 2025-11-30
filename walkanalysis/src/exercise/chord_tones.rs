use std::collections::HashMap;

use crate::{
    exercise::{
        analysis::{Analysis, Correction, Mistake, MistakeKind, NoteAnalysis},
        Exercise,
    },
    form::{
        chord::{Chord, ChordTone},
        form::FormPiece,
        key::Degree,
        note::Note,
    },
};

pub struct ChordTones {}

impl Exercise for ChordTones {
    fn explain(&self) -> String {
        "Play the root on the 1, and any chord tone on any beat.".into()
    }

    fn correct(&mut self, analysis: &Analysis) -> Correction {
        let mut mistakes = HashMap::new();

        for (&beat, (form_piece, note_analysis)) in analysis.beat_analysis.iter() {
            common_mistakes(&mut mistakes, beat, form_piece, *note_analysis);
            match form_piece {
                FormPiece::ChordBar(chord) => todo!(),
                FormPiece::HalfBar(chord, chord1) => todo!(),
                _ => (),
            }
        }

        todo!()
    }
}

pub fn common_bar_mistakes(
    mistakes: &mut HashMap<u32, Mistake>,
    note_analysis: NoteAnalysis,
) -> Option<(Note, Degree, ChordTone)> {
    //
}

pub fn common_mistakes(
    mistakes: &mut HashMap<u32, Mistake>,
    beat: u32,
    form_piece: &FormPiece,
    note_analysis: NoteAnalysis,
) -> Option<(Note, Degree, ChordTone)> {
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
        FormPiece::ChordBar(_) => {
            let NoteAnalysis::Note {
                note,
                degree_in_key: _degree_in_key,
                role_in_chord,
            } = note_analysis
            else {
                mistakes.insert(
                    beat,
                    Mistake {
                        beat,
                        mistake: MistakeKind::ExpectedNote {
                            found: note_analysis,
                        },
                    },
                );
                return None;
            };

            Some((note, _degree_in_key, role_in_chord))
        }
        _ => None,
    }
}
