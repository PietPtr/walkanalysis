use walkanalysis::form::{
    key::{Key, Quality},
    note::*,
};

#[test]
fn test_keys() {
    let c_major = Key::new(C, Quality::Major);
    assert_eq!(format!("{}", c_major.scale().sharp()), "C D E F G A B ");
    assert_eq!(format!("{}", c_major.scale().flat()), "C D E F G A B ");

    let c_minor = Key::new(C, Quality::Minor);
    assert_eq!(format!("{}", c_minor.scale().flat()), "C D E♭ F G A♭ B♭ ");

    let g_major = Key::new(G, Quality::Major);
    assert_eq!(format!("{}", g_major.scale().sharp()), "G A B C D E F♯ ");

    let d_major = Key::new(D, Quality::Major);
    assert_eq!(format!("{}", d_major.scale().sharp()), "D E F♯ G A B C♯ ");

    let a_major = Key::new(A, Quality::Major);
    assert_eq!(format!("{}", a_major.scale().sharp()), "A B C♯ D E F♯ G♯ ");

    let e_major = Key::new(E, Quality::Major);
    assert_eq!(format!("{}", e_major.scale().sharp()), "E F♯ G♯ A B C♯ D♯ ");

    let b_major = Key::new(B, Quality::Major);
    assert_eq!(
        format!("{}", b_major.scale().sharp()),
        "B C♯ D♯ E F♯ G♯ A♯ "
    );

    let f_major = Key::new(F, Quality::Major);
    assert_eq!(format!("{}", f_major.scale().flat()), "F G A B♭ C D E ");

    let bb_major = Key::new(B_FLAT, Quality::Major);
    assert_eq!(format!("{}", bb_major.scale().flat()), "B♭ C D E♭ F G A ");

    let eb_major = Key::new(E_FLAT, Quality::Major);
    assert_eq!(format!("{}", eb_major.scale().flat()), "E♭ F G A♭ B♭ C D ");

    let ab_major = Key::new(A_FLAT, Quality::Major);
    assert_eq!(format!("{}", ab_major.scale().flat()), "A♭ B♭ C D♭ E♭ F G ");

    let db_major = Key::new(D_FLAT, Quality::Major);
    assert_eq!(
        format!("{}", db_major.scale().flat()),
        "D♭ E♭ F G♭ A♭ B♭ C "
    );

    let a_minor = Key::new(A, Quality::Minor);
    assert_eq!(format!("{}", a_minor.scale().sharp()), "A B C D E F G ");
    assert_eq!(format!("{}", a_minor.scale().flat()), "A B C D E F G ");

    let e_minor = Key::new(E, Quality::Minor);
    assert_eq!(format!("{}", e_minor.scale().sharp()), "E F♯ G A B C D ");

    let b_minor = Key::new(B, Quality::Minor);
    assert_eq!(format!("{}", b_minor.scale().sharp()), "B C♯ D E F♯ G A ");

    let fs_minor = Key::new(F_SHARP, Quality::Minor);
    assert_eq!(format!("{}", fs_minor.scale().sharp()), "F♯ G♯ A B C♯ D E ");

    let cs_minor = Key::new(C_SHARP, Quality::Minor);
    assert_eq!(
        format!("{}", cs_minor.scale().sharp()),
        "C♯ D♯ E F♯ G♯ A B "
    );

    let gs_minor = Key::new(G_SHARP, Quality::Minor);
    assert_eq!(
        format!("{}", gs_minor.scale().sharp()),
        "G♯ A♯ B C♯ D♯ E F♯ "
    );

    let d_minor = Key::new(D, Quality::Minor);
    assert_eq!(format!("{}", d_minor.scale().flat()), "D E F G A B♭ C ");

    let g_minor = Key::new(G, Quality::Minor);
    assert_eq!(format!("{}", g_minor.scale().flat()), "G A B♭ C D E♭ F ");

    let f_minor = Key::new(F, Quality::Minor);
    assert_eq!(format!("{}", f_minor.scale().flat()), "F G A♭ B♭ C D♭ E♭ ");

    let bb_minor = Key::new(B_FLAT, Quality::Minor);
    assert_eq!(
        format!("{}", bb_minor.scale().flat()),
        "B♭ C D♭ E♭ F G♭ A♭ "
    );
}
