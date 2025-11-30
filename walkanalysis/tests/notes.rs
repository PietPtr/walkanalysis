use walkanalysis::form::note::Note;

#[test]
fn test_notes() {
    for n in 0..12 {
        println!("{}", Note::from(n).sharp());
    }
    for n in 0..12 {
        println!("{}", Note::from(n).flat());
    }
}

#[test]
fn test_equality() {
    dbg!(Note::from(-2) == Note::from(10));
    dbg!(Note::from(-2).flat(), Note::from(10).flat());
}
