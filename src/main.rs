use anyhow::Result;
use fancy_regex::Regex;
use regex_rustler::parser::parse_args;
use regex_rustler::process_file;
use std::fs;
use std::io::Write;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let config = parse_args()?;

    if config.real_time {
        real_time_regex_testing(&config.file_path)?;
    } else {
        // Handle the case when regex_pattern is None
        if let Some(regex_pattern) = &config.regex_pattern {
            process_file(&config.file_path, regex_pattern)?;
        } else {
            return Err(anyhow::anyhow!("Regex pattern is required"));
        }
    }

    Ok(())
}

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

fn real_time_regex_testing(file_path: &str) -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let file_contents = fs::read_to_string(file_path)?;
    let mut regex_pattern = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => regex_pattern.push(c),
                    KeyCode::Backspace => {
                        regex_pattern.pop();
                    }
                    KeyCode::Esc => break,
                    _ => (),
                }
            }
        }

        // Clear screen and reset cursor position
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        writeln!(stdout, "Regex pattern: {}", regex_pattern)?;

        // Apply regex to the text and display it
        if let Ok(regex) = Regex::new(&regex_pattern) {
            for line in file_contents.lines() {
                let processed_line =
                    regex_rustler::processor::process_regex(line, &regex, &mut false)?;
                writeln!(stdout, "{}", processed_line)?;
            }
        }

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
