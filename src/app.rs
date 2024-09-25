use std::{io, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};

use crate::ui::ui;

pub enum CurrentScreen {
    Home,
    Game,
    Exiting,
}

pub struct App {
    pub input: String,
    pub next_sentence: usize,
    pub point: i32,
    pub current_screen: CurrentScreen,
}

impl App {
    const SENTENCES: [&str; 3] = ["hello hira-typist", "こんにちは世界", "打倒寿司打"];

    pub fn new() -> Self {
        App {
            input: String::new(),
            next_sentence: 0,
            point: 0,
            current_screen: CurrentScreen::Home,
        }
    }

    fn change_sentence(&mut self) {
        self.next_sentence += 1;
        self.next_sentence %= Self::SENTENCES.len();
    }

    fn check_input(&mut self) {
        if self.input == Self::SENTENCES[self.next_sentence] {
            self.point += 1;
            self.change_sentence();
            self.input.clear();
            while poll(Duration::from_secs(0)).unwrap() {
                let _ = event::read();
            }
        }
    }

    pub fn get_next_sentence(&self) -> &str {
        Self::SENTENCES[self.next_sentence]
    }

    pub fn get_correct(&self) -> String {
        let mut index = 0;
        let mut chars1 = self.input.chars();
        let mut chars2 = self.get_next_sentence().chars();

        while let (Some(c1), Some(c2)) = (chars1.next(), chars2.next()) {
            if c1 != c2 {
                break;
            }
            index += 1;
        }

        self.input.chars().take(index).collect()
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, &self))?;
            if let Event::Key(key) = event::read()? {
                match self.current_screen {
                    CurrentScreen::Home => match key.code {
                        KeyCode::Enter => {
                            self.current_screen = CurrentScreen::Game;
                        }
                        KeyCode::Esc | KeyCode::Char('q') => {
                            self.current_screen = CurrentScreen::Exiting;
                            break;
                        }
                        _ => {}
                    },
                    CurrentScreen::Game if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            self.check_input();
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Home;
                        }
                        _ => {}
                    },
                    _ => break,
                };
            }
        }
        Ok(())
    }
}
