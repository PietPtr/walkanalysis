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
