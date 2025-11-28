use std::{fs::File, io::Write};

use walkanalysis::form::{
    form::Form,
    songs::{autumn_leaves::autumn_leaves, test::test},
};

#[test]
fn test_serialize_form() {
    let json = serde_json::to_string(&autumn_leaves()).unwrap();
    let mut file = File::create("tests/data/forms/autumn_leaves.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

#[test]
fn test_deserialize_form() {
    let file = File::open("tests/data/forms/autumn_leaves.json").unwrap();
    let data: Form = serde_json::from_reader(file).unwrap();
    assert_eq!(autumn_leaves(), data);
}

#[test]
fn test_form_lengths() {
    assert_eq!(autumn_leaves().length_in_beats(), 136);
    assert_eq!(test().length_in_beats(), 16);
}
