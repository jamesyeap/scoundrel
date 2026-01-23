use crate::app::{App, CurrentScreen, HAND_SIZE};
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Constraint::{Length, Percentage, Ratio};
use ratatui::prelude::Constraint::{Fill, Min};
use ratatui::prelude::{Color, Direction, Layout, Rect, Span};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use std::io::BufRead;
use tui_cards::{Card, Rank, Suit};

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
                .title("Enter room? (y/n)")
                .style(Style::default().bg(Color::Gray));
            frame.render_widget(block, frame.area());

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Percentage(70), Percentage(20), Percentage(10)])
                .margin(3)
                .split(frame.area());
            let stats_area = layout[1]; // to display life points, number of cards left, etc
            let notifications_area = layout[2]; // to display messages (e.g. lost 4 health points, equipped weapon, etc)

            let cards_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Percentage(70), Percentage(30)])
                .split(layout[0]);
            let cards_in_hand_area = cards_layout[0];
            let equipped_weapon_area = cards_layout[1];

            render_cards(frame, app, cards_in_hand_area);
            render_stats(frame, app, stats_area);
            render_notifications(frame, app, notifications_area);
            render_equipped_weapon(frame, app, equipped_weapon_area);
        }

        CurrentScreen::ChooseCard => {
            // let block = Block::default().borders(Borders::ALL).title("Scoundrel");
            // frame.render_widget(block, frame.area());

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Percentage(70), Percentage(20), Percentage(10)])
                .margin(3)
                .split(frame.area());
            let stats_area = layout[1]; // to display life points, number of cards left, etc
            let notifications_area = layout[2]; // to display messages (e.g. lost 4 health points, equipped weapon, etc)

            let cards_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Percentage(70), Percentage(30)])
                .split(layout[0]);
            let cards_in_hand_area = cards_layout[0];
            let equipped_weapon_area = cards_layout[1];

            render_cards(frame, app, cards_in_hand_area);
            render_stats(frame, app, stats_area);
            render_notifications(frame, app, notifications_area);
            render_equipped_weapon(frame, app, equipped_weapon_area);
        }

        CurrentScreen::ChooseWeaponOrBareKnuckle => {
            let popup_area = centered_rect(70, 50, frame.area());

            let block = Block::default()
                .borders(Borders::ALL)
                .title("Use equipped weapon? (y/n)");

            frame.render_widget(block, popup_area);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Percentage(50), Percentage(50)])
                .margin(2)
                .split(popup_area);

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

        CurrentScreen::Won => {
            let won_message = Paragraph::new(Text::styled(
                format!("You survived!\n Your score: {}", app.calculate_score()),
                Style::default().fg(Color::Green),
            ));

            frame.render_widget(won_message, frame.area());
        }

        CurrentScreen::Lost => {
            let lost_message = Paragraph::new(Text::styled(
                format!("You died!\n Your score: {}", app.calculate_score()),
                Style::default().fg(Color::Green),
            ));

            frame.render_widget(lost_message, frame.area());
        }

        _ => {}
    }
}

fn render_notifications(frame: &mut Frame, app: &App, area: Rect) {
    let notification = if let Some(notification) = app.notifications.last() {
        Paragraph::new(Text::styled(
            notification,
            Style::default().fg(Color::LightBlue),
        ))
    } else {
        Paragraph::new(Text::styled("", Style::default().fg(Color::LightBlue)))
    };

    frame.render_widget(notification, area);
}

fn render_equipped_weapon(frame: &mut Frame, app: &App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Percentage(70), Percentage(30)])
        .split(area);

    let mut equipped_weapon_widget = match app.equipped_weapon.as_ref() {
        Some(equipped_weapon) => Paragraph::new(Text::styled(
            format!("{equipped_weapon}"),
            Style::default().fg(Color::Blue),
        )),
        None => Paragraph::new(Text::styled(
            "NO WEAPON EQUIPPED",
            Style::default().fg(Color::Blue),
        )),
    };
    equipped_weapon_widget = equipped_weapon_widget.block(
        Block::default()
            .borders(Borders::ALL)
            .title_top("Equipped weapon"),
    );
    frame.render_widget(equipped_weapon_widget, layout[0]);

    let mut last_creature_blocked = match app.blocked_creatures.last() {
        Some(blocked_creature) => Paragraph::new(Text::styled(
            format!("{blocked_creature}"),
            Style::default().fg(Color::Blue),
        )),
        None => Paragraph::new(Text::styled(
            "NO CREATURE BLOCKED",
            Style::default().fg(Color::Blue),
        )),
    };
    last_creature_blocked = last_creature_blocked.block(
        Block::default()
            .borders(Borders::ALL)
            .title_top("Last creature blocked"),
    );
    frame.render_widget(last_creature_blocked, layout[1]);
}

fn render_stats(frame: &mut Frame, app: &App, area: Rect) {
    let border = Block::new().borders(Borders::ALL);
    frame.render_widget(border, area);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Fill(1), Fill(1)])
        .split(area);
    let health_points = Paragraph::new(Text::styled(
        format!("life: {}", app.life),
        Style::default().fg(Color::Blue),
    ));
    let number_of_cards_in_deck = Paragraph::new(Text::styled(
        format!("cards in deck: {}", app.deck.len()),
        Style::default().fg(Color::Blue),
    ));
    frame.render_widget(health_points, layout[0]);
    frame.render_widget(number_of_cards_in_deck, layout[1]);
}

pub fn render_cards(frame: &mut Frame, app: &App, area: Rect) {
    let border = Block::new().borders(Borders::ALL).title("Your hand");
    frame.render_widget(border, area);

    let cards_layout = Layout::default()
        .direction(Direction::Horizontal)
        // .margin(1)
        .constraints(vec![Percentage((100 / HAND_SIZE) as u16); HAND_SIZE])
        .split(area);

    app.hand
        .iter()
        .map(|card| match card {
            // TODO: render proper cards here
            Some(card) => Text::styled(card.to_string(), Style::default().fg(Color::LightYellow)),
            None => Text::styled("USED", Style::default().fg(Color::LightYellow)),
        })
        .enumerate()
        .for_each(|(idx, card_widget)| {
            // render card
            frame.render_widget(&card_widget, centered_rect(50, 80, cards_layout[idx]));
        });
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Percentage((100 - percent_y) / 2),
            Percentage(percent_y),
            Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Percentage((100 - percent_x) / 2),
            Percentage(percent_x),
            Percentage((100 - percent_x) / 2),
        ])
        .split(vertical_chunks[1]);

    horizontal_chunks[1] // return the middle chunk
}
