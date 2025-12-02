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

pub fn two_beat_thirds() -> TwoBeat {
    TwoBeat {
        allowed_chord_tones_in_second_half: vec![ChordTone::Third],
    }
}
pub fn two_beat_fifths() -> TwoBeat {
    TwoBeat {
        allowed_chord_tones_in_second_half: vec![ChordTone::Fifth],
    }
}
pub fn two_beat() -> TwoBeat {
    TwoBeat {
        allowed_chord_tones_in_second_half: vec![ChordTone::Third, ChordTone::Fifth],
    }
}

pub struct TwoBeat {
    pub allowed_chord_tones_in_second_half: Vec<ChordTone>,
}

impl Exercise for TwoBeat {
    fn explain(&self) -> String {
        let second_half_explanation = if self.allowed_chord_tones_in_second_half.len() == 1 {
            let only_allowed_tone = self
                .allowed_chord_tones_in_second_half
                .iter()
                .next()
                .expect("Expect more than 0 chord tones in TwoBeat exercise");

            format!("The second must be the {only_allowed_tone}")
        } else {
            let allowed_tones = self
                .allowed_chord_tones_in_second_half
                .iter()
                .fold("".to_string(), |acc, tone| format!("{}, {}", acc, tone));
            let trimmed = &allowed_tones[..allowed_tones.len() - 1];
            format!("The second must be either {trimmed}").into()
        };

        format!("Play two half notes per measure. The first half note must be the root. {second_half_explanation}").into()
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
                for b in 0..1 {
                    if chord_tone != ChordTone::Root {
                        mistakes.insert(
                            beat + b,
                            Mistake {
                                beat: beat + b,
                                mistake: MistakeKind::WrongNote {
                                    played: note,
                                    expected: chord.note(ChordTone::Root).unwrap(), // TODO: exercise incompatibility with a form causes unwraps like these to trigger
                                },
                            },
                        );
                    }
                }
            } else if beat_in_bar == 2 {
                if !self
                    .allowed_chord_tones_in_second_half
                    .contains(&chord_tone)
                {
                    for b in 0..1 {
                        mistakes.insert(
                            beat + b,
                            Mistake {
                                beat: beat + b,
                                mistake: MistakeKind::ExpectedChordTone {
                                    played_chord_tone: chord_tone,
                                    played_note: note,
                                    expected_example: chord
                                        .note(
                                            **self
                                                .allowed_chord_tones_in_second_half
                                                .iter()
                                                .collect::<Vec<_>>()
                                                .choose(&mut rand::rng())
                                                .unwrap(),
                                        )
                                        .unwrap(),
                                },
                            },
                        );
                    }
                }
            }
        }

        Correction {
            amount_of_beats: analysis.beat_analysis.len(),
            mistakes,
        }
    }
}
