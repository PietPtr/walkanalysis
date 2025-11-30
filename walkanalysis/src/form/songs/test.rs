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
            // TODO: there's too much freedom in forms, each form MUST start with a key (keep key marker for key changes though), and each MUST start with a count off
            FormPiece::Key(Key::new(G, Quality::Minor)),
            FormPiece::CountOff,
            bar(C.min7()),
            // bar(F.dominant7()),
            // bar(B_FLAT.maj7()),
            // bar(E_FLAT.maj7()),
            FormPiece::LineBreak,
        ],
    }
}
