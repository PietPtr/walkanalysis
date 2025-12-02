use std::fmt::Display;

use nih_plug::prelude::Enum;
use walkanalysis::exercise::{
    arpeggios_up::ArpeggiosUp,
    chord_tones::ChordTones,
    two_beat::{two_beat, two_beat_fifths, two_beat_thirds},
    Exercise,
};

#[derive(Debug, Enum, PartialEq, Clone, Copy, Eq)]
pub enum ExerciseKind {
    ArpeggiosUp,
    ChordTones,
    TwoBeatFifths,
    TwoBeatThirds,
    TwoBeat,
}

unsafe impl Sync for ExerciseKind {}

impl ExerciseKind {
    pub fn exercise(&self) -> Box<dyn Exercise> {
        match self {
            ExerciseKind::ArpeggiosUp => Box::new(ArpeggiosUp {}),
            ExerciseKind::ChordTones => Box::new(ChordTones {}),
            ExerciseKind::TwoBeatFifths => Box::new(two_beat_fifths()),
            ExerciseKind::TwoBeatThirds => Box::new(two_beat_thirds()),
            ExerciseKind::TwoBeat => Box::new(two_beat()),
        }
    }

    pub const ALL: [ExerciseKind; 5] = [
        ExerciseKind::ArpeggiosUp,
        ExerciseKind::ChordTones,
        ExerciseKind::TwoBeatFifths,
        ExerciseKind::TwoBeatThirds,
        ExerciseKind::TwoBeat,
    ];
}

impl Display for ExerciseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseKind::ArpeggiosUp => write!(f, "Arpeggios Up"),
            ExerciseKind::ChordTones => write!(f, "Chord Tones"),
            ExerciseKind::TwoBeatFifths => write!(f, "Two Beat, Fifths"),
            ExerciseKind::TwoBeatThirds => write!(f, "Two Beat, Thirds"),
            ExerciseKind::TwoBeat => write!(f, "Two Beat, Any chord tone"),
        }
    }
}
