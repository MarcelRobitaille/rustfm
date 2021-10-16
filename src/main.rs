use std::env;
use std::error::Error;
use std::fs;
use std::io;
use termion::event::Key;
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::backend::TermionBackend;
use tui::style::Color;
use tui::style::{Modifier, Style};
use tui::widgets::{List, ListItem, ListState};
use tui::Terminal;

mod events;
use events::{Event, Events};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    let mut cwd = env::current_dir()?;

    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        let paths = fs::read_dir(cwd.to_owned())?.collect::<Result<Vec<_>, io::Error>>()?;

        let list_items = paths
            .iter()
            .map(|e| {
                ListItem::new(e.file_name().into_string().unwrap()).style(if e.path().is_dir() {
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                })
            })
            .collect::<Vec<_>>();
        let list = List::new(list_items.to_owned())
            .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED));

        terminal.draw(|f| {
            let size = f.size();
            // let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_stateful_widget(list, size, &mut state);
        })?;

        let i = state.selected().unwrap();
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') | Key::Ctrl('c') => {
                    break;
                }
                Key::Down | Key::Char('j') => {
                    state.select(Some((i + 1) % paths.len()));
                }
                Key::Up | Key::Char('k') => {
                    state.select(Some((i + (paths.len() - 1)) % paths.len()));
                }
                Key::Right | Key::Char('l') => {
                    cwd.push(paths[i].path());
                    state.select(Some(0));
                }
                Key::Left | Key::Char('h') => {
                    cwd.pop();
                    state.select(Some(0));
                }
                _ => {
                    println!("Key: {:?}", input);
                }
            },
            Event::Tick => {
                // app.advance();
            }
        }
    }

    Ok(())
}
