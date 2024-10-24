#[allow(dead_code)]
pub const PATTERNS: [(usize, &str); 5] = [
    (1, r"abc"),
    (2, r"https?://.+:.+@dev.azure.com.*"),
    (3, r"https?://.+:.+@dev.azure.com.*"),
    (
        4,
        r"eyj[a-zA-Z0-9\-_%]+\.eyj[a-zA-Z0-9\-_%]+\.[a-zA-Z0-9\-_%]+",
    ), // OAuth JWT
    (5, r"[tT]oken"),
];

#[allow(dead_code)]
pub const LOOK_AHEAD_AND_BEHIND_SIZE: usize = 5;
