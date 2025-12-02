use std::fmt::Display;

use nih_plug::prelude::Enum;
use walkanalysis::form::{
    form::Form,
    songs::{
        autumn_leaves::autumn_leaves, but_beautiful::but_beautiful, test::longer_test, test::test,
    },
};

#[derive(Default, Debug, Enum, PartialEq, Clone, Copy, Eq)]
pub enum FormKind {
    #[default]
    Test,
    LongerTest,
    AutumnLeaves,
    AllTheThingsYouAre,
    ButBeautiful,
}

impl Display for FormKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormKind::Test => write!(f, "Test"),
            FormKind::AutumnLeaves => write!(f, "Autumn Leaves"),
            FormKind::AllTheThingsYouAre => write!(f, "All The Things You Are"),
            FormKind::ButBeautiful => write!(f, "But Beautiful"),
            FormKind::LongerTest => write!(f, "Longer Test"),
        }
    }
}

impl FormKind {
    pub fn form(&self) -> Form {
        match self {
            FormKind::AutumnLeaves => autumn_leaves(),
            FormKind::Test => test(),
            FormKind::AllTheThingsYouAre => todo!(),
            FormKind::ButBeautiful => but_beautiful(),
            FormKind::LongerTest => longer_test(),
        }
    }

    pub const ALL: [FormKind; 4] = [
        FormKind::LongerTest,
        FormKind::Test,
        // FormKind::AllTheThingsYouAre,
        FormKind::AutumnLeaves,
        FormKind::ButBeautiful,
    ];
}

unsafe impl Sync for FormKind {}
