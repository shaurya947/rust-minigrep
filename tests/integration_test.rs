use std::{
    io::{self, Write},
    writeln,
};

use tempfile::{self, NamedTempFile};

fn generate_test_file() -> io::Result<NamedTempFile> {
    let mut file = tempfile::NamedTempFile::new()?;
    writeln!(file, "hello, it's me")?;
    writeln!(file, "I was wondering if after all this time")?;
    writeln!(file, "you'd like to me to go over")?;
    writeln!(file, "everything")?;
    writeln!(file, "they say that time's supposed to heal you")?;
    writeln!(file, "but I ain't done much healing")?;
    Ok(file)
}

#[test]
fn error_when_file_not_fond() {
    let result = minigrep::search_file_path_for_string("bad_path", "thing").map_err(|e| e.kind());
    assert_eq!(result, Err(io::ErrorKind::NotFound));
}

#[test]
fn search_file_path_0_matches() {
    let file = generate_test_file().expect("Test file should be generated");
    let path = file
        .path()
        .to_str()
        .expect("Test file path should be valid");
    let result = minigrep::search_file_path_for_string(path, "limousine")
        .expect("Searching file path should produce no errors");
    assert_eq!(result, Vec::<String>::new());
}

#[test]
fn search_file_path_1_match() {
    let file = generate_test_file().expect("Test file should be generated");
    let path = file
        .path()
        .to_str()
        .expect("Test file path should be valid");
    let result = minigrep::search_file_path_for_string(path, "thing")
        .expect("Searching file path should produce no errors");
    assert_eq!(result, vec!["everything"]);
}

#[test]
fn search_file_path_multiple_matches() {
    let file = generate_test_file().expect("Test file should be generated");
    let path = file
        .path()
        .to_str()
        .expect("Test file path should be valid");
    let result = minigrep::search_file_path_for_string(path, "to")
        .expect("Searching file path should produce no errors");
    assert_eq!(
        result,
        vec![
            "you'd like to me to go over",
            "they say that time's supposed to heal you"
        ]
    );
}
