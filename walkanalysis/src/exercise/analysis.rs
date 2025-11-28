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
    pub fn analyze(transcription: Transcription, form: &Form) -> Analysis {
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
    pub mistakes: Vec<Mistake>,
}

impl Correction {
    pub fn score(&self) -> f32 {
        1.0 - (self.mistakes.len() as f32 / self.amount_of_beats as f32)
    }
}

impl Display for Correction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sortable = self.mistakes.clone();
        sortable.sort_by(|m1, m2| m1.beat.cmp(&m2.beat));

        for mistake in sortable.iter() {
            writeln!(f, "{}", mistake)?
        }
        writeln!(f, "{:.1}% correct.", self.score() * 100.)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Mistake {
    pub beat: u32,
    pub mistake: MistakeKind,
}

impl Display for Mistake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}.{}] {}",
            self.beat / 4,
            self.beat % 4 + 1,
            self.mistake
        )
    }
}

#[derive(Debug, Clone)]
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
