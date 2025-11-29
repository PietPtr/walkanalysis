use crate::form::{
    form::{bar, Form, FormPiece},
    key::{Key, Quality},
    note::{A, D, G},
};

// A short test song
pub fn test() -> Form {
    Form {
        tempo: 110,
        music: vec![
            FormPiece::Key(Key::new(G, Quality::Major)),
            FormPiece::CountOff,
            bar(G.maj7()),
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min7()),
            FormPiece::LineBreak,
        ],
    }
}
