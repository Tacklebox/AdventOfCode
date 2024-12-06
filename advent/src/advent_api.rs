use anyhow::{bail, Result};
use regex::Regex;
use reqwest::blocking;
use scraper::{Html, Selector};

// This email is here so the advent of code developer can contact me if this framework sends a
// bunch of potentially service disrupting requests. If you change anything in the advent crate,
// please put your email here, and pay attention to your email. Don't ruin adventofcode for others
// and save the effort of messing with this for solving the puzzles in the advent :)
const CONTACT_EMAIL: &str = "taqtb6p09@mozmail.com";

pub fn get_input(day: u8, year: i32, session_cookie: &str) -> Result<String> {
    let client = blocking::Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = client
        .get(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .header("User-Agent", CONTACT_EMAIL) // This is so if
        .send()?;
    if response.status().is_success() {
        response.text().map_err(anyhow::Error::from)
    } else {
        bail!("Failed to retrieve input: {}", response.status());
    }
}

#[derive(Debug)]
pub enum AdventResult {
    Correct,
    Incorrect(String),
    RateLimit(String),
    AlreadySubmitted,
}

pub fn submit_answer(
    day: u8,
    year: i32,
    level: u8,
    answer: &str,
    session_cookie: &str,
) -> Result<AdventResult> {
    let client = blocking::Client::new();
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let level = level.to_string();
    let params = [("level", &level[..]), ("answer", answer)];
    let response = client
        .post(&url)
        .header("Cookie", format!("session={session_cookie}"))
        .header("User-Agent", CONTACT_EMAIL)
        .form(&params)
        .send()?;
    if response.status().is_success() {
        let doc = Html::parse_document(&response.text()?);
        let selector = Selector::parse("body main article p").unwrap();
        if let Some(result_text) = doc.select(&selector).next() {
            let text = result_text.text().collect::<Vec<_>>().join(" ");
            if text.starts_with("That's the right answer!") {
                return Ok(AdventResult::Correct);
            } else if text.starts_with("That's not the right answer;") {
                return Ok(AdventResult::Incorrect(String::from(
                    text.split_once('.').unwrap().0,
                )));
            } else if text.starts_with("You gave an answer too recently;") {
                let time_remaining_pattern = Regex::new(r"You have (\w+) left to wait.").unwrap();
                let seconds_remaining = time_remaining_pattern.find(&text).unwrap();
                return Ok(AdventResult::RateLimit(
                    seconds_remaining.as_str().to_string(),
                ));
            } else if text.starts_with("You don't seem to be solving the right level.") {
                return Ok(AdventResult::AlreadySubmitted);
            }
        }
        bail!("Unexpected response. Please use the website instead of trying again")
    } else {
        bail!("Failed to retrieve input: {}", response.status());
    }
}
