use crate::app::{App, CurrentScreen, HAND_SIZE};
use ratatui::Frame;
use ratatui::layout::Constraint::Percentage;
use ratatui::prelude::{Color, Direction, Layout, Line, Rect, Span};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Welcome => {
            let welcome_message = Paragraph::new(Text::styled(
                "Welcome! Press any button to continue; or press (q) to quit.",
                Style::default().fg(Color::Green),
            ));

            frame.render_widget(welcome_message, frame.area());
        }

        CurrentScreen::BeforeRoom => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Enter room? (y/n)")
                .style(Style::default().bg(Color::Gray));
            frame.render_widget(block, frame.area());

            let card_area = Layout::default().margin(1).constraints(vec![Percentage(100)]).split(frame.area());
            render_cards(frame, card_area[0], app);
        }

        CurrentScreen::ChooseCard => {
            let block = Block::default().borders(Borders::ALL).title("Scoundrel");

            let card_area = Layout::default().margin(1).constraints(vec![Percentage(100)]).split(frame.area());
            render_cards(frame, card_area[0], app);
        }

        CurrentScreen::ChooseWeaponOrBareKnuckle => {
            let block = Block::default()
                .borders(Borders::ALL)
                .title("Use equipped weapon? (y/n)");

            frame.render_widget(block, frame.area());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Percentage(50), Percentage(50)])
                .margin(2)
                .split(frame.area());

            let weapon = Paragraph::new(Text::styled(
                format!("{}", app.equipped_weapon.as_ref().unwrap()),
                Style::default().fg(Color::Blue),
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_bottom("Your equipped weapon"),
            );
            let creature = Paragraph::new(Text::styled(
                format!("{}", app.in_combat_with_creature.as_ref().unwrap()),
                Style::default().fg(Color::Blue),
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_bottom("Creature"),
            );

            frame.render_widget(weapon, chunks[0]);
            frame.render_widget(creature, chunks[1]);
        }

        _ => {}
    }
}

pub fn render_cards(frame: &mut Frame, area: Rect, app: &App) {
    let cards_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Percentage((100 / HAND_SIZE) as u16); HAND_SIZE])
        .split(area);

    app.hand
        .iter()
        .map(|card| match card {
            Some(card) => {
                Paragraph::new(Span::styled(
                    card.to_string(),
                    Style::default().fg(Color::Blue),
                ))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::LightGreen)),
                    )
            },
            None => {
                Paragraph::new(Span::styled("USED", Style::default().fg(Color::DarkGray)))
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default().fg(Color::DarkGray)),
                    )
            }
        })
        .enumerate()
        .for_each(|(idx, card_widget)| frame.render_widget(card_widget, cards_layout[idx]));
}
