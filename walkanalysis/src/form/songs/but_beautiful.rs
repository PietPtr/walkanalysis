use crate::form::{
    form::{bar, half_bar, Form, FormPiece},
    key::{Key, Quality},
    note::*,
};

pub fn but_beautiful() -> Form {
    Form::new(
        70,
        Key::new(G, Quality::Major).sharp(),
        vec![
            bar(G.maj7()), // TODO: Gmaj9
            bar(G_SHARP.dim7()),
            bar(A.min7()), // TODO: Amin9
            bar(B_FLAT.dim7()),
            FormPiece::LineBreak,
            bar(G.maj()),
            half_bar(D.min(), E.dominant7()), // TODO: Dmin6
            bar(A.dominant7()),               // TODO: A9
            bar(A.dominant7()),               // TODO: A9
            FormPiece::LineBreak,
            bar(D.dominant7()),
            half_bar(G.maj(), E.min7()),
            half_bar(A.min7(), D.dominant7()), // TODO: rhythmic variation: 𝅗𝅥. ♩
            bar(G.maj()),
            FormPiece::LineBreak,
            bar(E.min7()),      // TODO: Em6 Em7
            bar(A.dominant7()), // TODO: A9
            bar(A.min7()),
            half_bar(A.min7(), D.dominant7()), // TODO: rhythmic variation: 𝅗𝅥. ♩
            FormPiece::LineBreak,
            bar(G.maj7()), // TODO: Gmaj9
            bar(G_SHARP.dim7()),
            bar(A.min7()), // TODO: Amin9
            bar(B_FLAT.dim7()),
            FormPiece::LineBreak,
            bar(G.maj()),
            half_bar(D.min(), E.dominant7()), // TODO: Dmin6
            bar(A.dominant7()),               // TODO: A9
            bar(A.dominant7()),               // TODO: A9
            FormPiece::LineBreak,
            bar(D.dominant7()),
            half_bar(G.maj(), E.min7()),
            half_bar(A.min7(), B.dominant7()), // TODO: B7#5
            half_bar(E.min(), F.dominant7()),  // TODO: rhythmic variation: 𝅗𝅥. ♩
            FormPiece::LineBreak,
            half_bar(G.maj(), B_FLAT.dominant7()), // TODO: rhythmic variation: 𝅗𝅥. ♩
            half_bar(A.min7(), A_FLAT.dominant7()),
            bar(G.maj()),
            bar(D.dominant7()),
        ],
    )
}
