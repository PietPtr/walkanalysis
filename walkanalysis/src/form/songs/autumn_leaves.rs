use crate::form::{
    form::{bar, half_bar, Form, FormPiece},
    key::{Key, Quality},
    note::{A, B_FLAT, C, D, E, E_FLAT, F, G, G_FLAT},
};

pub fn autumn_leaves() -> Form {
    Form {
        tempo: 110,
        music: vec![
            FormPiece::Key(Key::new(G, Quality::Minor)),
            FormPiece::CountInBar,
            FormPiece::CountInBar,
            // A section
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min()),
            bar(G.min()),
            // repeat of A section
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min()),
            bar(G.min()),
            // B section
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min()),
            bar(G.min()),
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(B_FLAT.maj7()),
            // End of B section
            bar(A.m7b5()),
            bar(D.dominant7()),
            half_bar(G.min7()),
            half_bar(G_FLAT.dominant7()),
            half_bar(F.min7()),
            half_bar(E.dominant7()),
            bar(E_FLAT.min7()),
            half_bar(A.m7b5()),
            half_bar(D.dominant7()),
            bar(G.min7()),
            bar(G.min7()),
        ],
    }
}
