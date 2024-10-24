use regex::Regex;
mod constants;
use clap::Parser;
use clap_verbosity::{InfoLevel, Verbosity};
use gitleaks_rs::{check_sequential, print_leaks};
use log::{debug, error, info};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the file to read
    #[arg(short = 'p', long = "path", default_value = "data/clean_test_file.txt")]
    path: std::path::PathBuf,
    /// How verbose do you want the program to be?
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() {
    let start_time = std::time::Instant::now();
    // Parse the command line arguments
    let args = Cli::parse();

    // Initialize the logger
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(&args.verbose.log_level_filter().to_string()),
    )
    .target(env_logger::Target::Stdout)
    .init();

    info!("Welcome to gitleaks-rs 🔒");

    let regexes: Vec<_> = constants::PATTERNS
        .iter()
        .map(|(id, re)| (id, Regex::new(re).expect("Invalid regex string")))
        .collect();

    let file_content = std::fs::read_to_string(&args.path).expect("Unable to read file");

    let found_matches = check_sequential(regexes, &file_content);

    let status = print_leaks(found_matches, &file_content);
    match status {
        0 => info!("No leaks found! 🎉"),
        1 => std::process::exit(1),
        2_u8..=u8::MAX => error!("Unexpected behavior ocurred! 😢"),
    }

    debug!("Time elapsed: {}s", start_time.elapsed().as_secs_f32());
}
