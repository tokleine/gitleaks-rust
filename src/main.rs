use regex::Regex;
// use std::time::Instant;

fn main() {
    //define patterns
    let patterns = [
        (1, r"abc"),
        (2, r"https?://.+:.+@dev.azure.com.*"),
        (3, r"https?://.+:.+@dev.azure.com.*"),
        (
            4,
            r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+",
        ), // OAuth JWT
        (5, r"[tT]oken"),
    ];

    const LOOK_AHEAD_AND_BEHIND_SIZE: usize = 5;

    // compile regexes, slow operation
    let regexes: Vec<_> = patterns
        .iter()
        .map(|(id, re)| (id, Regex::new(re).expect("Invalid regex string")))
        .collect();

    // read file
    let file_content = std::fs::read_to_string("data/test_file.txt").expect("Unable to read file");

    // check for patterns without itering over lines
    for (id, regex) in regexes.iter() {
        if regex.is_match(&file_content) {
            // find all matches for a given regex
            let found_matches: Vec<(usize, usize, &str, usize)> = regex
                .find_iter(&file_content)
                .map(|m| {
                    (
                        m.start(), // start index of match
                        m.end(),   // end index of match
                        m.as_str(),// matched string
                        &file_content[..m.start()].matches('\n').count() + 1, // line number, as counted by line breaks
                    )
                }) // get match as well as start and end index
                .collect();

            // print found matches
            for (start_idx, end_idx, _, line) in found_matches {
                println!(
                    "Found pattern '{}' at line {}: ...{}{}{}{}{}...",
                    id,
                    line,
                    &file_content[start_idx - LOOK_AHEAD_AND_BEHIND_SIZE..start_idx]
                        .replace("\n", ""),
                    "\x1b[1;31m", // ANSI escape code for bold red text
                    &file_content[start_idx..end_idx].replace("\n", ""),
                    "\x1b[0m", // ANSI escape code to reset
                    &file_content[end_idx..end_idx + LOOK_AHEAD_AND_BEHIND_SIZE].replace("\n", "")
                );
            }
        }
    }
}
