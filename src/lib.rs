use std::{fs, io};

pub fn search_file_path_for_string(
    file_path: &str,
    search_string: &str,
) -> Result<Vec<String>, io::Error> {
    let file_as_string = fs::read_to_string(file_path)?;
    Ok(
        search_string_lines_for_string(&file_as_string, search_string)
            .iter()
            .map(|&s| String::from(s))
            .collect(),
    )
}

fn search_string_lines_for_string<'a>(source_sting: &'a str, search_string: &str) -> Vec<&'a str> {
    source_sting
        .lines()
        .filter(|l| l.contains(search_string))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_string_lines_0_matches() {
        assert_eq!(
            search_string_lines_for_string(get_source_string(), "limousine"),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn search_string_lines_1_match() {
        assert_eq!(
            search_string_lines_for_string(get_source_string(), "every"),
            vec!["everything"]
        );
    }

    #[test]
    fn search_string_lines_multiple_matches() {
        assert_eq!(
            search_string_lines_for_string(get_source_string(), "to"),
            vec![
                "you'd like to me to go over",
                "they say that time's supposed to heal you"
            ]
        );
    }

    fn get_source_string() -> &'static str {
        "hello, it's me
I was wondering if after all this time
you'd like to me to go over
everything
they say that time's supposed to heal you
but I ain't done much healing
        "
    }
}
