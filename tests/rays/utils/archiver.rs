#![allow(unused_must_use)]

use std::fs;

use mirror::rays::utils::Archiver;
use mirror::slg::State;

const ARCHIVER_TEST_FILE: &str = "tests/rays/utils/test-archiver-state";

#[test]
fn test_archiver_open() {
    let filename = format!("{}-{}", ARCHIVER_TEST_FILE, "open");

    let archiver = Archiver::open(&filename.as_str());
    assert_eq!(archiver.stats.reads, 0);

    // Clean up
    fs::remove_file(filename);
}

#[test]
fn test_archiver_serialize() {
    let filename = format!("{}-{}", ARCHIVER_TEST_FILE, "serialize");

    let state: State = State::new("test");
    assert_eq!(state.get_engine_tag(), "test");

    let mut archiver = Archiver::open(&filename.as_str());

    let result = archiver.serialize(&state);
    assert_eq!(result.ok(), Some(()));

    archiver.flush();

    assert_eq!(archiver.stats.writes, 12);

    let data: Vec<u8> = vec![4, 0, 0, 0, 0, 0, 0, 0, 116, 101, 115, 116];
    assert_eq!(*archiver.data(), data);

    // Clean up
    fs::remove_file(filename);
}

#[test]
fn test_archiver_deserialize() {
    let filename = format!("{}-{}", ARCHIVER_TEST_FILE, "deserialize");

    // #1 Setup
    let state: State = State::new("test");
    let mut a1 = Archiver::open(&filename.as_str());
    a1.serialize(&state);
    a1.flush();

    // #2 read it again
    let mut a2 = Archiver::open(&filename.as_str());
    assert_eq!(a2.stats.reads, 12);

    let result = a2.deserialize();
    assert_eq!(result.ok(), Some(State::new("test")));

    // Clean up
    fs::remove_file(filename);
}
