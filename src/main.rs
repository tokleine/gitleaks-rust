use regex::Regex;
use std::time::Instant;

fn main() {
    let re: [(Regex, &str); 5] = [
        (Regex::new(r"abc").unwrap(), "abc123"),
        (
            Regex::new(r"https?://.+:.+@dev.azure.com.*").unwrap(),
            "https://username:password@dev.azure.com",
        ),
        (
            Regex::new(r"https?://.+:.+@dev.azure.com.*").unwrap(),
            "https://username.password@dev.azure.com",
        ),
        (
            Regex::new(r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+").unwrap(), // OAuth JWT
            "eyj0eXAiOiJKV1.eyjhdWQiOMi4wIn0.OnvSu-8w",
        ),
        (Regex::new(r"token").unwrap(), "asd the token is: eyj...."),
    ];

    for (pattern, text) in re.iter() {
        let start = Instant::now();
        let result: bool = match_pattern_in_haystack(pattern, text);
        let duration = start.elapsed();

        println!("Result: {}. Time taken: {:?}", result, duration);
    }
}

fn match_pattern_in_haystack(pattern: &Regex, haystack: &str) -> bool {
    pattern.is_match(haystack)
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
