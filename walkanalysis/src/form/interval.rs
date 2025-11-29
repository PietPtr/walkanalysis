use crate::form::note::Note;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    Unison,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    Tritone,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    Octave,
}
impl Interval {
    pub fn steps(&self) -> i32 {
        match self {
            Interval::Unison => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::AugmentedFourth | Interval::Tritone | Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::Octave => 12,
        }
    }

    pub fn from_steps(steps: i32) -> Option<Interval> {
        match steps {
            0 => Some(Interval::Unison),
            1 => Some(Interval::MinorSecond),
            2 => Some(Interval::MajorSecond),
            3 => Some(Interval::MinorThird),
            4 => Some(Interval::MajorThird),
            5 => Some(Interval::PerfectFourth),
            6 => Some(Interval::Tritone),
            7 => Some(Interval::PerfectFifth),
            8 => Some(Interval::MinorSixth),
            9 => Some(Interval::MajorSixth),
            10 => Some(Interval::MinorSeventh),
            11 => Some(Interval::MajorSeventh),
            12 => Some(Interval::Octave),
            _ => None,
        }
    }

    pub fn find(bottom: Note, top: Note) -> Option<Interval> {
        let mut top_index = top.index.rem_euclid(12);
        let bottom_index = bottom.index.rem_euclid(12);

        if top_index < bottom_index {
            top_index += 12;
        }
        Interval::from_steps(top_index - bottom_index)
    }
}
