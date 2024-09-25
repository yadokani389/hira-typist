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
        CurrentScreen::Exiting => vec![" Exiting ".into()],
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
    let next_sentence = Paragraph::new(app.get_next_sentence()).centered();
    let correct = app.get_correct();
    let input = Paragraph::new(Line::from(vec![
        correct.as_str().green(),
        app.input.chars().skip(correct.chars().count()).collect::<String>().red(),
        app.get_next_sentence()
            .chars()
            .skip(app.input.chars().count())
            .collect::<String>()
            .gray(),
    ]))
    .centered();
    frame.render_widget(next_sentence, chunks[0]);
    frame.render_widget(input, chunks[1]);
}
