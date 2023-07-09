use std::{fs, io};

pub fn search_file_path_for_string(
    file_path: &str,
    search_string: &str,
) -> Result<Vec<String>, io::Error> {
    let matches = fs::read_to_string(file_path)?
        .lines()
        .filter(|l| l.contains(search_string))
        .map(|s| s.to_string())
        .collect();
    Ok(matches)
}
