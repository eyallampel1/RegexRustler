/// This module provides the functionality for real-time regex testing.
/// It allows users to enter a regex pattern and see the matches as they type.
use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use fancy_regex::Regex;
use std::fs;
use std::io::{self, Write}; // mybe add BufRead

use crate::processor;

// Removed unused import

pub fn real_time_regex_testing(file_path: &str) -> Result<()> {
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
                    processor::process_regex(line, &regex, &mut false)?;
                writeln!(stdout, "{}", processed_line)?;
            }
        }

        stdout.flush()?;
    }

    terminal::disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
