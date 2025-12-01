use rand::seq::IndexedRandom;
use std::collections::HashMap;

use crate::{
    analysis::{
        analysis::Analysis,
        correction::Correction,
        mistake::{Mistake, MistakeKind},
    },
    exercise::Exercise,
    form::chord::ChordTone,
};

use super::common_mistakes;

pub struct ChordTones {}

impl Exercise for ChordTones {
    fn explain(&self) -> String {
        "Play the root on the 1, and any chord tone on any beat.".into()
    }

    fn correct(&mut self, analysis: &Analysis) -> Correction {
        let mut mistakes = HashMap::new();

        for (&beat, (form_piece, note_analysis)) in analysis.beat_analysis.iter() {
            let Some((note, _degree, chord_tone, chord)) =
                common_mistakes(&mut mistakes, beat, form_piece, *note_analysis)
            else {
                continue;
            };

            let beat_in_bar = beat % 4;
            if beat_in_bar == 0 {
                if chord_tone != ChordTone::Root {
                    mistakes.insert(
                        beat,
                        Mistake {
                            beat,
                            mistake: MistakeKind::WrongNote {
                                played: note,
                                expected: chord.note(ChordTone::Root).unwrap(), // TODO: exercise incompatibility with a form causes unwraps like these to trigger
                            },
                        },
                    );
                }
            } else {
                if chord_tone == ChordTone::NoChordTone {
                    mistakes.insert(
                        beat,
                        Mistake {
                            beat,
                            mistake: MistakeKind::ExpectedChordTone {
                                played_chord_tone: chord_tone,
                                played_note: note,
                                expected_example: *chord.notes.choose(&mut rand::rng()).unwrap(),
                            },
                        },
                    );
                }
            }
        }

        Correction {
            amount_of_beats: analysis.beat_analysis.len(),
            mistakes,
        }
    }
}
