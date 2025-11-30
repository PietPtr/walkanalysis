use crate::exercise::analysis::{Analysis, Correction};

pub mod analysis;
pub mod arpeggios_up;
pub mod chord_tones;

pub trait Exercise {
    /// User facing explanation about what must be played for a perfect score
    fn explain(&self) -> String;
    /// Correction function: given an analysis of a transcription, how many beats fit
    /// both the form and the exercise?
    fn correct(&mut self, analysis: &Analysis) -> Correction;
}
