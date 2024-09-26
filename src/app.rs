use std::io;

use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};

use crate::ui::ui;

#[derive(PartialEq)]
pub enum CurrentScreen {
    Home,
    Game,
    Result,
}

pub struct App {
    pub input: String,
    pub next_sentence: usize,
    pub point: usize,
    pub current_screen: CurrentScreen,
    pub timer: std::time::Instant,
    pub left_time: std::time::Duration,
}

impl App {
    const SENTENCES: [&str; 6] = [
        "hello hira-typist",
        "こんにちは世界",
        "打倒寿司打",
        "とてもながいひらがなをうちたいな",
        "このあぷりはえすけーけーとあいしょうがいい",
        "もともとAZIK使ってたら寿司打ができなそうだなと思った",
    ];

    pub fn new() -> Self {
        App {
            input: String::new(),
            next_sentence: 0,
            point: 0,
            current_screen: CurrentScreen::Home,
            timer: std::time::Instant::now(),
            left_time: std::time::Duration::from_secs(60),
        }
    }

    fn change_sentence(&mut self) {
        self.next_sentence += 1;
        self.next_sentence %= Self::SENTENCES.len();
    }

    fn check_input(&mut self) {
        if self.input == Self::SENTENCES[self.next_sentence] {
            self.point += self.input.chars().count();
            self.change_sentence();
            self.input.clear();
            while poll(std::time::Duration::from_secs(0)).unwrap() {
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

    fn start_game(&mut self) {
        self.current_screen = CurrentScreen::Game;
        self.input.clear();
        self.next_sentence = 0;
        self.point = 0;
        self.timer = std::time::Instant::now();
    }

    fn end_game(&mut self) {
        self.current_screen = CurrentScreen::Result;
        self.point += self.get_correct().chars().count();
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| ui(frame, &self))?;
            if self.current_screen == CurrentScreen::Game && self.left_time < self.timer.elapsed() {
                self.end_game();
            }
            if !poll(std::time::Duration::from_secs(0)).unwrap() {
                continue;
            }
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match self.current_screen {
                    CurrentScreen::Home => match key.code {
                        KeyCode::Enter => self.start_game(),
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        _ => {}
                    },
                    CurrentScreen::Game => match key.code {
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            self.check_input();
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => self.end_game(),
                        _ => {}
                    },
                    CurrentScreen::Result => match key.code {
                        KeyCode::Enter => self.current_screen = CurrentScreen::Home,
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        _ => {}
                    },
                };
            }
        }
        Ok(())
    }
}
