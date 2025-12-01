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

pub struct ArpeggiosUp {}

impl Exercise for ArpeggiosUp {
    fn explain(&self) -> String {
        "On every first beat, play the root of the chord. On every second beat the third, then the fifth, then the seventh. If the chord defines no seventh, play the root again.".into()
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
            let correct_role_in_chord = match beat_in_bar {
                0 => ChordTone::Root,
                1 => ChordTone::Third,
                2 => ChordTone::Fifth,
                3 => {
                    if chord.has_seventh() {
                        ChordTone::Seventh
                    } else {
                        ChordTone::Root
                    }
                }
                _ => unreachable!(),
            };

            if chord_tone != correct_role_in_chord {
                let mistake = Mistake {
                    beat,
                    mistake: MistakeKind::WrongNote {
                        played: note,
                        expected: chord.note(correct_role_in_chord).expect(&format!(
                            "Expect {}/{} to have a {:?}",
                            chord.sharp(),
                            chord.flat(),
                            correct_role_in_chord
                        )),
                    },
                };
                mistakes.insert(beat, mistake);
            }
        }

        Correction {
            amount_of_beats: analysis.beat_analysis.len(),
            mistakes,
        }
    }
}
