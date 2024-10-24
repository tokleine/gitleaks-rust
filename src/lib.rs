mod constants;

use log::warn;

pub struct MatchDetail<'a> {
    pub pattern_id: usize,
    pub start_idx: usize,
    pub end_idx: usize,
    pub matched_str: &'a str,
    pub line: usize,
}

pub fn check_sequential<'a>(
    regexes: Vec<(&'a usize, regex::Regex)>,
    file_content: &'a str,
) -> Vec<MatchDetail<'a>> {
    let mut found_matches: Vec<MatchDetail> = Vec::new();
    // check for patterns without itering over lines
    for (id, regex) in regexes.iter() {
        if regex.is_match(&file_content) {
            // find all matches for a given regex
            let matches: Vec<MatchDetail> = regex
                .find_iter(&file_content)
                .map(|m| {
                    MatchDetail {
                        pattern_id: **id,       // pattern id
                        start_idx: m.start(),    // start index of match
                        end_idx: m.end(),        // end index of match
                        matched_str: m.as_str(), // matched string
                        line: &file_content[..m.start()].matches('\n').count() + 1, // line number, as counted by line breaks
                    }
                }) // get match as well as start and end index
                .collect();
            found_matches.extend(matches);
        }
    }
    return found_matches;
}

pub fn print_leaks(found_matches: Vec<MatchDetail>, file_content: &str) -> u8 {
    if found_matches.is_empty() {
        return 0;
    } else {
        warn!("Leaks found! 💀");
        // print found matches
        for match_detail in found_matches {
            warn!(
                "Found pattern '{}' at line {}: ...{}{}{}{}{}...",
                match_detail.pattern_id,
                match_detail.line,
                &file_content[match_detail.start_idx - constants::LOOK_AHEAD_AND_BEHIND_SIZE
                    ..match_detail.start_idx]
                    .replace("\n", ""),
                "\x1b[1;31m", // ANSI escape code for bold red text
                &file_content[match_detail.start_idx..match_detail.end_idx].replace("\n", ""),
                "\x1b[0m", // ANSI escape code to reset
                &file_content[match_detail.end_idx
                    ..match_detail.end_idx + constants::LOOK_AHEAD_AND_BEHIND_SIZE]
                    .replace("\n", "")
            );
        }
        return 1;
    }
}
