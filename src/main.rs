use regex::Regex;
use std::time::Instant;

fn main() {
    let re = [
        (r"abc", "abc123"),
        (
            r"https?://.+:.+@dev.azure.com.*",
            "https://username:password@dev.azure.com",
        ),
        (
            r"https?://.+:.+@dev.azure.com.*",
            "https://username.password@dev.azure.com",
        ),
        (
            r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+", // OAuth JWT
            "eyj0eXAiOiJKV1.eyjhdWQiOMi4wIn0.OnvSu-8w",
        ),
        (r"token", "asd the token is: eyj...."),
    ];

    let start = Instant::now();
    let examples: Vec<_> = re
        .iter()
        .map(|(re, text)| (Regex::new(re).expect("Invalid regex string"), text))
        .collect();
    println!("Time to compile regex: {:?}", start.elapsed());

    let start = Instant::now();
    for (re, text) in examples.iter() {
        println!(
            "Pattern: {:?}, Text: {:?}, Result: {:?}",
            re,
            text,
            match_pattern_in_haystack(re, text)
        );
    }
    println!("Time to match regex: {:?}", start.elapsed());
}

fn match_pattern_in_haystack(pattern: &Regex, haystack: &str) -> bool {
    pattern.is_match(haystack)
}

/*
 * First iteration: function to identify 5 common patterns in strings - check
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
