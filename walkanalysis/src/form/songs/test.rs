use crate::form::{
    form::{bar, Form, FormPiece},
    key::{Key, Quality},
    note::*,
};

// A short test song
pub fn test() -> Form {
    Form {
        tempo: 110,
        music: vec![
            FormPiece::Key(Key::new(G, Quality::Major)),
            FormPiece::CountOff,
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            FormPiece::LineBreak,
        ],
    }
}
