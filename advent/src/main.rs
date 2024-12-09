#![allow(dead_code)]
use anyhow::bail;
use chrono::Datelike;
use inquire::Select;
use std::{env, path::PathBuf};
use tracing::{debug, info, level_filters::LevelFilter};
use tracing_subscriber::util::SubscriberInitExt;

mod advent_api;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Command,
}

#[derive(Args, Debug)]
struct DateArgs {
    /// Which year of the advent
    #[arg(short, long)]
    year: Option<i32>,

    /// Which day of the advent
    #[arg(short, long)]
    day: Option<u8>,
}

#[derive(Subcommand)]
enum Command {
    /// Create a crate for attempting an advent challenge
    New {
        #[clap(flatten)]
        date: DateArgs,
    },
    /// Submits output from the current challenge
    Submit {
        /// Which part to submit
        #[arg(short, long, default_value_t = 1)]
        part: u8,

        /// File to pass to the advent as input
        #[arg(short, long)]
        input_file: Option<PathBuf>,

        #[clap(flatten)]
        date: DateArgs,
    },
    /// Like submit, but without submitting the answer.
    Run {
        /// Which part to submit
        #[arg(short, long, default_value_t = 1)]
        part: u8,

        /// File to pass to the advent as input
        #[arg(short, long)]
        input_file: Option<PathBuf>,

        #[clap(flatten)]
        date: DateArgs,
    },
    /// Save authentication cookie to allow for automatically retrieving your
    /// challenge inputs and attempting challenges.
    Authenticate { session_cookie: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.debug > 0 {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    tracing_subscriber::fmt()
        .pretty()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .init();

    debug!("DEBUG ENABLED");

    let today = chrono::Local::now().date_naive();
    let current_year = if today.month() == 12 {
        today.year()
    } else {
        today.year() - 1
    };
    let available_years: Vec<_> = (2015..=current_year).rev().collect();
    let ws_root = workspace_root();

    match cli.command {
        Command::New { date } => {
            let year = if let Some(year) = date.year {
                year
            } else {
                // use current/most recent year by default
                let prompt_index: usize = std::env::current_dir()?
                    .strip_prefix(&ws_root)
                    .ok()
                    .and_then(|active_year| active_year.components().next())
                    .and_then(|year| {
                        year.as_os_str()
                            .to_string_lossy()
                            .parse::<i32>()
                            .ok()
                            .and_then(|default_year| {
                                available_years.iter().position(|&y| y == default_year)
                            })
                    })
                    .unwrap_or(0);

                Select::new("Year:", available_years)
                    .with_starting_cursor(prompt_index)
                    .prompt()?
            };

            let day = if let Some(day) = date.day {
                day
            } else {
                let year_dir = &ws_root.join(year.to_string());
                if !year_dir.is_dir() && year_dir.exists() {
                    bail!("{year_dir:?} exists but is not a directory");
                }
                let available_days = if !year_dir.exists() {
                    debug!("creating directory [{}]", year_dir.display());
                    std::fs::create_dir(year_dir)?;
                    (1..=25).collect()
                } else {
                    (1..=25)
                        .filter(|n| !year_dir.join(n.to_string()).is_dir())
                        .collect()
                };
                Select::new("Day:", available_days).prompt()?
            };

            // Only important for the relative pathing.
            // It's possible to just figure out what the correct relative path is from CWD, but
            // this is just easier.
            std::env::set_current_dir(&ws_root)?;

            let mut child = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("new")
                .arg(ws_root.join(year.to_string()).join(day.to_string()))
                .arg("--name")
                .arg(format!("advent_{year}_{day}"))
                .spawn()?;
            child.wait()?;

            let mut child = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("add")
                .arg("--path")
                .arg("common")
                .arg("--package")
                .arg(format!("advent_{year}_{day}"))
                .spawn()?;
            child.wait()?;
            let mut child = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("add")
                .arg("anyhow")
                .arg("--package")
                .arg(format!("advent_{year}_{day}"))
                .spawn()?;
            child.wait()?;
            let mut child = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("add")
                .arg("tracing")
                .arg("--package")
                .arg(format!("advent_{year}_{day}"))
                .spawn()?;
            child.wait()?;
            let main_template = include_bytes!("../solution_template.rs");
            std::fs::write(
                ws_root
                    .join(year.to_string())
                    .join(day.to_string())
                    .join("src")
                    .join("main.rs"),
                main_template,
            )?;
        }

        Command::Submit {
            part,
            date,
            input_file,
        } => {
            let Some(session_cookie) = cached_session_cookie()? else {
                bail!("Can't submit without a session cookie. Configure one with the `authenticate` command")
            };

            let ws_root = workspace_root();

            let (year, day) = match (date.year, date.day) {
                (Some(year), Some(day)) => (year, day),
                (None, None) => {
                    let cwd = std::env::current_dir()?;
                    let mut dir_components = cwd
                        .strip_prefix(&ws_root)
                        .expect("This must be run inside the advent workspace")
                        .components();
                    if let (
                        Some(std::path::Component::Normal(year_dir)),
                        Some(std::path::Component::Normal(day_dir)),
                    ) = (dir_components.next(), dir_components.next())
                    {
                        (
                            year_dir.to_string_lossy().parse::<i32>()?,
                            day_dir.to_string_lossy().parse::<u8>()?,
                        )
                    } else {
                        bail!("Unable to determine which advent to run");
                    }
                }
                (None, Some(day)) => (std::env::current_dir()?
                    .strip_prefix(&ws_root)
                    .ok()
                    .and_then(|active_year| active_year.components().next())
                    .and_then(|active_year| {
                        if let std::path::Component::Normal(active_year) = active_year {
                            active_year.to_string_lossy().parse::<i32>().ok()
                        } else {
                            None
                        }
                    }).expect("Couldn't determine which year to run from current directory or args. Aborting."), day
                ),
                (Some(_), None) => bail!("If --year is specified, --day is also required"),
            };

            let input_file = if let Some(input_file) = input_file {
                std::fs::File::open(input_file)?
            } else {
                let cache_dir = ws_root.join(".cache");
                if !cache_dir.is_dir() {
                    std::fs::create_dir(&cache_dir)?;
                }
                let cached_input_path = cache_dir.join(format!("input_{year}_{day}.txt"));
                if !cached_input_path.exists() {
                    info!("Input file not found for advent {year} day {day}, downloading...");
                    let input = advent_api::get_input(day, year, &session_cookie)?;
                    std::fs::write(&cached_input_path, input)?;
                }
                std::fs::File::open(cached_input_path)?
            };

            debug!("Going to run: cargo run --package advent_{year}_{day}");
            let output = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("run")
                .arg("--package")
                .arg(format!("advent_{year}_{day}"))
                .arg("--")
                .arg(format!("--part={part}"))
                .stdin(input_file)
                .output()?;
            let answer = String::from_utf8(output.stdout)?;
            debug!("Solution ran without error, submitting: {answer} for part {part}...");
            let result =
                advent_api::submit_answer(day, year, part, answer.trim(), &session_cookie)?;
            match result {
                advent_api::AdventResult::Correct => println!("Success!"),
                advent_api::AdventResult::Incorrect(r) => println!("{r}"),
                advent_api::AdventResult::RateLimit(r) => {
                    println!("Too many submissions. Please wait {r} before trying again")
                }
                advent_api::AdventResult::AlreadySubmitted => {
                    println!("You already completed this challenge!")
                }
            }
        }

        Command::Run {
            part,
            date,
            input_file,
        } => {
            let Some(session_cookie) = cached_session_cookie()? else {
                bail!("Can't submit without a session cookie. Configure one with the `authenticate` command")
            };

            let ws_root = workspace_root();

            let (year, day) = match (date.year, date.day) {
                (Some(year), Some(day)) => (year, day),
                (None, None) => {
                    let cwd = std::env::current_dir()?;
                    let mut dir_components = cwd
                        .strip_prefix(&ws_root)
                        .expect("This must be run inside the advent workspace")
                        .components();
                    if let (
                        Some(std::path::Component::Normal(year_dir)),
                        Some(std::path::Component::Normal(day_dir)),
                    ) = (dir_components.next(), dir_components.next())
                    {
                        (
                            year_dir.to_string_lossy().parse::<i32>()?,
                            day_dir.to_string_lossy().parse::<u8>()?,
                        )
                    } else {
                        bail!("Unable to determine which advent to run");
                    }
                }
                (None, Some(day)) => (std::env::current_dir()?
                    .strip_prefix(&ws_root)
                    .ok()
                    .and_then(|active_year| active_year.components().next())
                    .and_then(|active_year| {
                        if let std::path::Component::Normal(active_year) = active_year {
                            active_year.to_string_lossy().parse::<i32>().ok()
                        } else {
                            None
                        }
                    }).expect("Couldn't determine which year to run from current directory or args. Aborting."), day
                ),
                (Some(_), None) => bail!("If --year is specified, --day is also required"),
            };

            let input_file = if let Some(input_file) = input_file {
                std::fs::File::open(input_file)?
            } else {
                let cache_dir = ws_root.join(".cache");
                if !cache_dir.is_dir() {
                    std::fs::create_dir(&cache_dir)?;
                }
                let cached_input_path = cache_dir.join(format!("input_{year}_{day}.txt"));
                if !cached_input_path.exists() {
                    info!("Input file not found for advent {year} day {day}, downloading...");
                    let input = advent_api::get_input(day, year, &session_cookie)?;
                    std::fs::write(&cached_input_path, input)?;
                }
                std::fs::File::open(cached_input_path)?
            };

            debug!("Going to run: cargo run --package advent_{year}_{day}");
            let output = std::process::Command::new(std::env::var("CARGO").unwrap())
                .arg("run")
                .arg("--package")
                .arg(format!("advent_{year}_{day}"))
                .arg("--")
                .arg(format!("--part={part}"))
                .stdin(input_file)
                .output()?;
            let answer = String::from_utf8(output.stdout)?;
            println!("Solution ran without error, produced: {answer} for part {part}");
        }
        Command::Authenticate { session_cookie } => {
            std::fs::write(session_cookie_cache_path(), session_cookie)?;
        }
    }

    Ok(())
}

fn cached_session_cookie() -> anyhow::Result<Option<String>> {
    let session_cookie_cache_path = session_cookie_cache_path();
    if session_cookie_cache_path.exists() {
        let session_cookie =
            String::from(std::fs::read_to_string(session_cookie_cache_path)?.trim());
        Ok(Some(session_cookie))
    } else {
        Ok(None)
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Is this not a cargo workspace?"))
            .parent()
            .unwrap(),
    )
}

fn session_cookie_cache_path() -> PathBuf {
    workspace_root().join(".cache").join("session_cookie.txt")
}
