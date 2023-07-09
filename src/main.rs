use minigrep::search_file_path_for_string;
use std::{env, eprintln, println};

fn main() {
    let args: Vec<String> = env::args().collect();
    // first index is executable path
    if args.len() != 3 {
        eprintln!("Two arguments needed: file path, and search string");
        return;
    }

    let file_path = &args[1];
    let search_string = &args[2];
    println!(
        "Specified file path: {}, search string: {}",
        file_path, search_string
    );

    match search_file_path_for_string(file_path, search_string) {
        Ok(matches) => {
            println!("Found {} matches:", matches.len());
            for (i, m) in matches.iter().enumerate() {
                println!("{}: {m}", i + 1);
            }
        }
        Err(e) => eprintln!("Encountered error {}", e),
    }
}
