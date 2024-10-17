use regex::Regex;

fn main() {
    let re = r"abc";
    let text = "ab123";
    let result: bool = match_pattern_in_haystack(re, text);
    println!("Result: {}", result);
}

fn match_pattern_in_haystack(pattern: &str, haystack: &str) -> bool {
    let re = Regex::new(pattern).unwrap();
    re.is_match(haystack)
}

/*
 * First iteration: function to identify 5 common patterns in strings
 * Second itegration: function to read a file and check for patterns
 * Third iteration: CLI to real all files in a directory and check for patterns
 * ...
 *
 * Feature list:
 * regex pattern matching
 * false positive exlusion
 * filepath & pre-fix exclusion
 *
*/
