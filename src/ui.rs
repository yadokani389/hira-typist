use block::*;
use layout::*;
use ratatui::*;
use style::*;
use symbols::border;
use text::*;
use widgets::*;

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(10),
            Constraint::Fill(1),
        ])
        .split(frame.area());
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(50),
            Constraint::Fill(1),
        ])
        .split(area[1]);

    let title = Title::from(" hira-typist ".bold());
    let instructions = Title::from(Line::from(match app.current_screen {
        CurrentScreen::Home => vec![
            " Start ".into(),
            "<Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ],
        CurrentScreen::Game => vec![" Exit ".into(), "<ESC> ".blue().bold()],
        CurrentScreen::Result => vec![
            " Home ".into(),
            "<Enter> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ],
    }));
    let block = Block::bordered()
        .title(title.alignment(Alignment::Center))
        .title(
            instructions
                .alignment(Alignment::Center)
                .position(block::Position::Bottom),
        )
        .border_set(border::THICK);

    frame.render_widget(block, area[1]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area[1]);

    match app.current_screen {
        CurrentScreen::Home => {
            let msg = Paragraph::new("Welcome to hira-typist".bold()).centered();
            frame.render_widget(msg, chunks[1]);
        }
        CurrentScreen::Game => {
            let correct = app.get_correct();
            let input = Paragraph::new(Line::from(vec![
                correct.as_str().green(),
                app.input
                    .chars()
                    .skip(correct.chars().count())
                    .collect::<String>()
                    .red(),
                app.get_next_sentence()
                    .chars()
                    .skip(app.input.chars().count())
                    .collect::<String>()
                    .gray(),
            ]))
            .centered();
            let upper_text = Text::from(vec![
                Line::from(format!("Point: {:3}", app.point + correct.chars().count())),
                Line::from(format!(
                    "Left time: {:3}",
                    if app.timer.elapsed() < app.left_time {
                        (app.left_time - app.timer.elapsed()).as_secs()
                    } else {
                        0
                    }
                )),
            ])
            .centered();
            frame.render_widget(upper_text, chunks[0]);
            frame.render_widget(input, chunks[1]);
        }
        CurrentScreen::Result => {
            let point = Paragraph::new(format!("Your point is {}!", app.point)).centered();
            let msg = Paragraph::new(match app.point {
                0..30 => "You can do better!".red(),
                30..60 => "Not bad!".yellow(),
                60..90 => "Good job!".green(),
                90..120 => "Excellent!".green(),
                120..=150 => "Perfect!".green(),
                _ => "Cheater!".red(),
            })
            .bold()
            .centered();
            frame.render_widget(point, chunks[0]);
            frame.render_widget(msg, chunks[1]);
        }
    }
}
