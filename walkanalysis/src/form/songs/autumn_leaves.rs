use crate::form::{
    form::{bar, half_bar, Form, FormPiece},
    key::{Key, Quality},
    note::{A, B_FLAT, C, D, E, E_FLAT, F, G, G_FLAT},
};

pub fn autumn_leaves() -> Form {
    Form::new(
        110,
        Key::new(G, Quality::Minor).flat(),
        vec![
            // A section
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            FormPiece::LineBreak,
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min()),
            bar(G.min()),
            FormPiece::LineBreak,
            // repeat of A section
            bar(C.min7()),
            bar(F.dominant7()),
            bar(B_FLAT.maj7()),
            bar(E_FLAT.maj7()),
            FormPiece::LineBreak,
            bar(A.m7b5()),
            bar(D.dominant7()),
            bar(G.min()),
            bar(G.min()),
            // FormPiece::LineBreak,
            // // B section
            // bar(A.m7b5()),
            // bar(D.dominant7()),
            // bar(G.min()),
            // bar(G.min()),
            // FormPiece::LineBreak,
            // bar(C.min7()),
            // bar(F.dominant7()),
            // bar(B_FLAT.maj7()),
            // bar(B_FLAT.maj7()),
            // FormPiece::LineBreak,
            // // End of B section
            // bar(A.m7b5()),
            // bar(D.dominant7()),
            // half_bar(G.min7(), G_FLAT.dominant7()),
            // half_bar(F.min7(), E.dominant7()),
            // FormPiece::LineBreak,
            // bar(E_FLAT.min7()),
            // half_bar(A.m7b5(), D.dominant7()),
            // bar(G.min7()),
            // bar(G.min7()),
            FormPiece::LineBreak,
        ],
    )
}
