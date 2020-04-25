#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events, Config},
    TabsState,
};

use std::{error::Error, io, time::Duration, thread};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::{TermionBackend, Backend},
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Text, Tabs},
    Terminal,
    Frame
};

struct App<'a> {
    tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    fn on_tick(&mut self){

    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Setup event handlers
    let events = Events::with_config(Config {
        exit_key: Key::Char('q'),
        tick_rate: Duration::from_millis(150),
    });

    let mut app = App {
        tabs: TabsState::new(vec!["Enter the Matrix", "Red pill"]),
    };

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
                .split(f.size());

            // Top block
            {
                let block = Block::default();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(chunks[0]);

                let text = [
                    Text::raw("Wake up, N......\n"),
                    Text::raw("Wait a minute, who are you?\n")
                ];

                let paragraph = Paragraph::new(text.iter()).block(block)
                    .style(Style::default().fg(Color::LightGreen)).alignment(Alignment::Left);
                f.render_widget(paragraph, chunks[0]);

            }

            // Bottom tabs block
            {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                    .split(chunks[1]);
                let block = Block::default()
                    .title("Tabs")
                    .title_style(Style::default().fg(Color::Yellow))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                f.render_widget(block, chunks[0]);

                let tabs = Tabs::default()
                    .block(Block::default().borders(Borders::ALL).title("Tabs"))
                    .titles(&app.tabs.titles)
                    .select(app.tabs.index)
                    .style(Style::default().fg(Color::Cyan))
                    .highlight_style(Style::default().fg(Color::LightRed));
                f.render_widget(tabs, chunks[0]);

                match app.tabs.index {
                    0 => {
                        let block = Block::default().title("Enter the Matrix").borders(Borders::ALL);

                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(
                                [
                                    Constraint::Percentage(100)
                                ]
                                    .as_ref(),
                            )
                            .split(chunks[1]);
                        let text = [
                            Text::raw("Login here\n"),
                            Text::raw("UI\n")
                        ];

                        let paragraph = Paragraph::new(text.iter()).block(block)
                            .style(Style::default().fg(Color::LightGreen)).alignment(Alignment::Left);
                        f.render_widget(paragraph, chunks[0]);
                    },
                    1 => {
                        let block = Block::default().title("Red pill").borders(Borders::ALL);

                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints(
                                [
                                    Constraint::Percentage(100)
                                ]
                                    .as_ref(),
                            )
                            .split(chunks[1]);
                        let text = [
                            Text::raw("Registration here\n"),
                            Text::raw("UI\n")
                        ];

                        let paragraph = Paragraph::new(text.iter()).block(block)
                            .style(Style::default().fg(Color::LightGreen)).alignment(Alignment::Left);
                        f.render_widget(paragraph, chunks[0]);
                    },
                    _ => unreachable!(),
                };
            }
        })?;
        match events.next()? {
            Event::Input(key) => match key {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            Event::Tick => {
                app.on_tick();
            },
        }
    }
    Ok(())
}