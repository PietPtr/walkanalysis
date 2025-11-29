use walkanalysis::form::note::{A, B_FLAT, C, C_SHARP, D};

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

#[test]
fn test_symbols() {
    println!("{}", A.m7b5().flat_symbol());
    println!("{}", A.m7b5().sharp_symbol());
    println!("{}", D.dominant7().sharp_symbol());
    println!("{}", B_FLAT.dominant7().sharp_symbol());
    println!("{}", B_FLAT.dominant7().flat_symbol());
    println!("{}", C.maj7().flat_symbol());
}
