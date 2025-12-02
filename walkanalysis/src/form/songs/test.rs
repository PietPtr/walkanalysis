use crate::form::{
    form::{bar, Form, FormPiece},
    key::{Key, Quality},
    note::*,
};

// A short test song
pub fn test() -> Form {
    Form::new(
        110,
        Key::new(G, Quality::Minor).flat(),
        vec![bar(C.min7()), FormPiece::LineBreak],
    )
}
pub fn longer_test() -> Form {
    Form::new(
        110,
        Key::new(G, Quality::Minor).flat(),
        vec![
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            FormPiece::LineBreak,
        ],
    )
}
