use crate::{
    exercise::{
        analysis::{Analysis, Correction, Mistake, MistakeKind, NoteAnalysis},
        Exercise,
    },
    form::{chord::ChordTone, form::FormPiece},
};

pub struct ArpeggiosUp {}

impl Exercise for ArpeggiosUp {
    // const EXPLANATION: &str = "On every first beat, play the root of the chord. On every second beat the third, then the fifth, then the seventh. If the chord defines no seventh, play the root again.";

    fn correct(&mut self, analysis: &Analysis) -> Correction {
        let mut mistakes = Vec::new();

        for (&beat, (form_piece, note_analysis)) in analysis.beat_analysis.iter() {
            // every first beat of a bar is the root, second is the third, third is the fifth, fourth is the seventh or root if no seventh
            match form_piece {
                FormPiece::Key(_) => (),
                FormPiece::CountInBar => {
                    if *note_analysis != NoteAnalysis::Silence {
                        mistakes.push(Mistake {
                            beat,
                            mistake: MistakeKind::ExpectedSilence {
                                found: *note_analysis,
                            },
                        });
                    }
                }
                FormPiece::ChordBar(chord) => {
                    let NoteAnalysis::Note {
                        note,
                        degree_in_key: _,
                        role_in_chord,
                    } = *note_analysis
                    else {
                        mistakes.push(Mistake {
                            beat,
                            mistake: MistakeKind::ExpectedNote {
                                found: *note_analysis,
                            },
                        });
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

                    if role_in_chord != correct_role_in_chord {
                        mistakes.push(Mistake {
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
                        });
                    }
                }
                FormPiece::HalfBar(chord1, chord2) => {
                    // TODO: lots of duplicated code with the above
                    let NoteAnalysis::Note {
                        note,
                        degree_in_key: _,
                        role_in_chord,
                    } = *note_analysis
                    else {
                        mistakes.push(Mistake {
                            beat,
                            mistake: MistakeKind::ExpectedNote {
                                found: *note_analysis,
                            },
                        });
                        continue;
                    };

                    let beat_in_bar = beat % 4;
                    let correct_role_in_chord = match beat_in_bar {
                        0 => ChordTone::Root,
                        1 => ChordTone::Third,
                        2 => ChordTone::Fifth,
                        3 => {
                            if chord2.has_seventh() {
                                ChordTone::Seventh
                            } else {
                                ChordTone::Root
                            }
                        }
                        _ => unreachable!(),
                    };

                    if role_in_chord != correct_role_in_chord {
                        mistakes.push(Mistake {
                            beat,
                            mistake: MistakeKind::WrongNote {
                                played: note,
                                expected: {
                                    match beat_in_bar {
                                        0 | 1 => chord1.note(correct_role_in_chord).unwrap(),
                                        2 | 3 => chord2.note(correct_role_in_chord).unwrap(),
                                        _ => unreachable!(),
                                    }
                                },
                            },
                        });
                    }
                }
                FormPiece::LineBreak => (),
            }
        }

        Correction {
            amount_of_beats: analysis.beat_analysis.len(),
            mistakes,
        }
    }
}
