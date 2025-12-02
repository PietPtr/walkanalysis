use walkanalysis::form::note::{Note, F};

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

#[test]
fn f_from_freq() {
    assert_eq!(Note::from_frequency(43.64).0, F);
}

#[test]
fn freq_errors() {
    const DIV: usize = 20;
    for i in (40 * DIV)..(47 * DIV) {
        let freq = i as f32 / DIV as f32;
        let (n, err) = Note::from_frequency(freq);
        println!("{freq:.2}Hz => {:?} {}", n.flat(), err)
    }
}
