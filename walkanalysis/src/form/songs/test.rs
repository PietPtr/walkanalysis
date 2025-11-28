use crate::form::{
    form::{bar, half_bar, Form, FormPiece},
    key::{Key, Quality},
    note::{A, D, G},
};

// A short test song
pub fn test() -> Form {
    Form {
        tempo: 110,
        music: vec![
            FormPiece::Key(Key::new(G, Quality::Major)),
            FormPiece::CountInBar,
            bar(G.maj7()),
            half_bar(A.m7b5()),
            half_bar(D.dominant7()),
            bar(G.min7()),
        ],
    }
}
