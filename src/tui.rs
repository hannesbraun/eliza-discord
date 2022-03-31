use std::collections::HashMap;
use std::io;

use also::Also;
use chrono::{DateTime, Local};
use crossterm::cursor::MoveToRow;
use crossterm::terminal::{Clear, ClearType};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, terminal};
use serenity::Error;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct ActiveChannel {
    pub last_activity: DateTime<Local>,
    pub name: String,
}

pub fn setup() {
    execute!(io::stdout(), EnterAlternateScreen).expect("Terminal setup");
    println!("{}", header());
}

fn header() -> String {
    format!("eliza-discord {}", env!("CARGO_PKG_VERSION"))
}

pub fn render(active_channels: &HashMap<u64, ActiveChannel>) {
    let (_, rows) = terminal::size().unwrap_or((80, 24));
    let filtered_channels: Vec<ActiveChannel> = active_channels
        .values()
        .cloned()
        .collect::<Vec<ActiveChannel>>()
        .also(|it| it.sort())
        .lets(|it| {
            it.into_iter()
                .rev()
                .take(std::cmp::max(4, rows) as usize - 4)
                .collect()
        });

    let _ = execute!(io::stdout(), Clear(ClearType::All), MoveToRow(1));
    if rows > 0 {
        println!("{}", header());
    }
    if rows > 1 {
        println!();
    }
    for channel in filtered_channels {
        println!("{} -> {}", channel.last_activity, channel.name);
    }
}

pub fn display_err(msg: &str, err: Error) {
    let (_, rows) = terminal::size().unwrap_or((80, 24));
    let _ = execute!(io::stdout(), MoveToRow(rows));
    println!("{}: {:?}", msg, err);
}

pub fn restore() {
    execute!(io::stdout(), LeaveAlternateScreen).expect("Restoring terminal");
}
