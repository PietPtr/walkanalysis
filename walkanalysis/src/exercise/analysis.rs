use std::{collections::HashMap, fmt::Display};

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
        degree_in_key: key::Degree,
        role_in_chord: chord::ChordTone,
    },
    NoteDuringSilence {
        note: Note,
    },
}
impl NoteAnalysis {
    pub fn note(&self) -> Option<Note> {
        match self {
            NoteAnalysis::Silence => None,
            NoteAnalysis::Note {
                note,
                degree_in_key: _,
                role_in_chord: _,
            } => Some(*note),
            NoteAnalysis::NoteDuringSilence { note } => Some(*note),
        }
    }
}

impl Analysis {
    /// Analyzes the roles of the notes played according to the transcription
    /// in the key and chord context of the given form
    pub fn analyze(transcription: Transcription, form: &Form) -> Analysis {
        let mut beat_analysis = HashMap::new();
        let mut form_analysis = vec![];

        let mut key = form.key().unwrite();
        let mut note_iter = transcription.notes.iter();

        let mut beat_number = 0;
        for form_piece in form.music().iter() {
            if let FormPiece::Key(new_key) = form_piece {
                key = *new_key
            }

            let notes_in_this_form_piece: Vec<_> = note_iter
                .by_ref()
                .take(form_piece.length_in_beats() as usize)
                .collect();

            let analyze_with_chord = |note, chord: &Chord| NoteAnalysis::Note {
                note,
                degree_in_key: key.role(note),
                role_in_chord: chord.role(note),
            };

            let analyses = notes_in_this_form_piece
                .iter()
                .map(|&&note| match note {
                    PlayedNote::Surely(note) => match form_piece {
                        FormPiece::Key(_) => unreachable!(),
                        FormPiece::CountOff => NoteAnalysis::NoteDuringSilence { note },
                        FormPiece::ChordBar(chord) => analyze_with_chord(note, chord),
                        FormPiece::HalfBar(chord1, chord2) => match beat_number % 4 {
                            0 | 1 => analyze_with_chord(note, chord1),
                            2 | 3 => analyze_with_chord(note, chord2),
                            _ => unreachable!(),
                        },
                        FormPiece::LineBreak => unreachable!(),
                    },
                    PlayedNote::Silence => match form_piece {
                        FormPiece::Key(_) => unreachable!(),
                        FormPiece::CountOff => NoteAnalysis::Silence,
                        FormPiece::ChordBar(_) => NoteAnalysis::Silence,
                        FormPiece::HalfBar(_, _) => NoteAnalysis::Silence,
                        FormPiece::LineBreak => unreachable!(),
                    },
                    PlayedNote::Unknown => NoteAnalysis::Silence,
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

/// To what extent an analysis of a transcription conformes to the exercise
#[derive(Debug, Clone)]
pub struct Correction {
    pub amount_of_beats: usize,
    /// All beats where something incorrect was played
    /// Maps beat to mistake (mistakes also save the beat, so there's some double administration that needs to be done correctly)
    pub mistakes: HashMap<u32, Mistake>,
}

impl Correction {
    pub fn score(&self) -> f32 {
        1.0 - (self.mistakes.len() as f32 / self.amount_of_beats as f32)
    }
}

impl Display for Correction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mistakes = self.mistakes.clone();
        let mut sortable = mistakes.values().collect::<Vec<_>>();
        sortable.sort_by(|m1, m2| m1.beat.cmp(&m2.beat));

        for mistake in sortable.iter() {
            writeln!(f, "{}", mistake)?
        }
        writeln!(f, "{:.1}% correct.", self.score() * 100.)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mistake {
    pub beat: u32,
    pub mistake: MistakeKind,
}

impl Display for Mistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}.{}] {}",
            self.beat / 4 + 1,
            self.beat % 4 + 1,
            self.mistake
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MistakeKind {
    WrongNote { played: Note, expected: Note },
    ExpectedSilence { found: NoteAnalysis },
    ExpectedNote { found: NoteAnalysis },
}

impl Display for MistakeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MistakeKind::WrongNote { played, expected } => write!(
                f,
                "Wrong note, played {} expected {}.",
                played.flat(),
                expected.flat()
            )?,
            MistakeKind::ExpectedSilence { found } => {
                write!(f, "Expected silence, found {:?}", found)?
            }
            MistakeKind::ExpectedNote { found } => {
                write!(f, "Expected some note, found {:?}", found)?
            }
        }
        Ok(())
    }
}
