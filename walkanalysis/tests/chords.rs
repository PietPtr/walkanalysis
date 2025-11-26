use walkanalysis::form::note::{A, B_FLAT, C_SHARP};

#[test]
fn test_chords() {
    println!("{}", A.maj7().sharp());
    println!("{}", A.dominant7().sharp());
    println!("{}", C_SHARP.maj().sharp());
    println!("{}", B_FLAT.maj7().flat());
}

#[test]
fn test_spelling() {
    println!("{}", A.maj7().spell());
    println!("{}", A.dominant7().spell());
    println!("{}", C_SHARP.maj().spell());
    println!("{}", B_FLAT.maj7().spell());
    println!("{}", B_FLAT.dim7().spell());
}
