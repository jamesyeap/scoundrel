use crate::app::{App, CurrentScreen, MAX_LIFE};
use crate::cards::deck::{Card, MAX_DECK_SIZE, Suite};
use color_eyre::owo_colors::OwoColorize;
use ratatui::Frame;
use ratatui::layout::Constraint::Percentage;
use ratatui::prelude::Color::Gray;
use ratatui::prelude::Constraint::Fill;
use ratatui::prelude::{Color, Direction, Layout, Line, Rect, Span};
use ratatui::style::Style;
use ratatui::style::palette::tailwind;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};
use std::io::BufRead;

trait ListStyle {
    fn get_list_style(&self) -> Style;
}

trait Emoji {
    fn get_emoji(&self) -> String;
}

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::Welcome => {
            let welcome_message = Paragraph::new(Text::from(vec![
                Line::styled(
                    "Welcome to Scoundrel, a single-player game that you can play with any deck of cards.",
                    Style::default().fg(Color::White),
                ).centered(),

                Line::styled(
                    "Press any button to continue; or press (q) to quit.",
                    Style::default().fg(Color::Magenta),
                ).centered(),

                Line::styled(
                    "How to play (links to YouTube - credits to @Rulies): https://www.youtube.com/watch?v=Gt2tYzM93h4",
                    Style::default().fg(Color::White),
                ).centered(),
            ]));

            frame.render_widget(welcome_message, centered_rect(80, 30, frame.area()));
        }

        CurrentScreen::BeforeRoom => {
            let block = Block::default()
                .title("Enter room? (y/n)")
                .style(Style::default().bg(Color::Gray));
            frame.render_widget(block, frame.area());

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Percentage(10),
                    Percentage(50),
                    Percentage(30),
                    Percentage(10),
                ])
                .margin(3)
                .split(frame.area());

            let number_of_cards_left_area = layout[0];
            render_number_of_cards_left(frame, app, number_of_cards_left_area);

            // display cards in hand
            let cards_in_hand_area = layout[1];
            render_cards(frame, app, cards_in_hand_area);

            // to display life points, number of cards left, etc
            let stats_and_weapons_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Fill(1), Fill(1)])
                .split(layout[2]);
            let stats_area = stats_and_weapons_area[0];
            let equipped_weapon_area = stats_and_weapons_area[1];
            render_health(frame, app, stats_area);
            render_equipped_weapon(frame, app, equipped_weapon_area);

            // to display messages (e.g. lost 4 health points, equipped weapon, etc)
            let notifications_area = layout[3];
            render_notifications(frame, app, notifications_area);
        }

        CurrentScreen::ChooseCard => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Percentage(10),
                    Percentage(50),
                    Percentage(30),
                    Percentage(10),
                ])
                .margin(3)
                .split(frame.area());

            let number_of_cards_left_area = layout[0];
            render_number_of_cards_left(frame, app, number_of_cards_left_area);

            // display cards in hand
            let cards_in_hand_area = layout[1];
            render_cards(frame, app, cards_in_hand_area);

            // to display life points, number of cards left, etc
            let stats_and_weapons_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Fill(1), Fill(1)])
                .split(layout[2]);
            let stats_area = stats_and_weapons_area[0];
            let equipped_weapon_area = stats_and_weapons_area[1];
            render_health(frame, app, stats_area);
            render_equipped_weapon(frame, app, equipped_weapon_area);

            // to display messages (e.g. lost 4 health points, equipped weapon, etc)
            let notifications_area = layout[3];
            render_notifications(frame, app, notifications_area);
        }

        CurrentScreen::ChooseWeaponOrBareKnuckle => {
            let popup_area = centered_rect(70, 50, frame.area());

            let block = Block::default().title("Use equipped weapon? (y/n)");

            frame.render_widget(block, popup_area);

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Percentage(50), Percentage(50)])
                .margin(2)
                .split(popup_area);

            let weapon = Paragraph::new(Text::styled(
                format!(
                    "{} {}",
                    app.equipped_weapon.as_ref().unwrap(),
                    app.equipped_weapon.as_ref().unwrap().get_emoji()
                ),
                Style::default().fg(Color::Blue),
            ))
            .centered()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_bottom("Your equipped weapon"),
            );
            let creature = Paragraph::new(Text::styled(
                format!(
                    "{} {}",
                    app.in_combat_with_creature.as_ref().unwrap(),
                    app.in_combat_with_creature.as_ref().unwrap().get_emoji()
                ),
                Style::default().fg(Color::Blue),
            ))
            .centered()
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

fn render_number_of_cards_left(frame: &mut Frame, app: &App, area: Rect) {
    let number_of_cards_cleared = MAX_DECK_SIZE - app.deck.len();
    let percentage_cleared = (number_of_cards_cleared as f64 / MAX_DECK_SIZE as f64) * 100.0;

    let gauge = Gauge::default()
        .gauge_style(tailwind::BLUE.c800)
        .percent(percentage_cleared.round() as u16)
        .block(Block::default().title("Progress"));

    frame.render_widget(gauge, area);
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
    let equipped_weapon_key = Span::styled("Equipped weapon: ", Style::default().fg(Color::White));
    let equipped_weapon_value = match app.equipped_weapon.as_ref() {
        Some(equipped_weapon) => Span::styled(
            format!(
                "{} {}",
                equipped_weapon.to_string(),
                equipped_weapon.get_emoji()
            ),
            Style::default().fg(Color::Blue),
        ),
        None => Span::styled("NO WEAPON EQUIPPED", Style::default().fg(Color::Blue)),
    };
    let equipped_weapon_line = Line::from(vec![equipped_weapon_key, equipped_weapon_value]);

    let last_creature_blocked_key =
        Span::styled("Last creature blocked: ", Style::default().fg(Color::White));
    let last_creature_blocked_value = match app.blocked_creatures.last() {
        Some(blocked_creature) => Span::styled(
            format!(
                "{} {}",
                blocked_creature.to_string(),
                blocked_creature.get_emoji()
            ),
            Style::default().fg(Color::Blue),
        ),
        None => Span::styled("NO CREATURE BLOCKED", Style::default().fg(Color::Blue)),
    };
    let last_creature_blocked_line =
        Line::from(vec![last_creature_blocked_key, last_creature_blocked_value]);
    frame.render_widget(
        Paragraph::new(Text::from(vec![
            equipped_weapon_line,
            last_creature_blocked_line,
        ]))
        .block(Block::default().borders(Borders::ALL).title("Weapon")),
        area,
    );
}

fn render_health(frame: &mut Frame, app: &App, area: Rect) {
    let health_gauge = Gauge::default()
        .gauge_style(tailwind::GREEN.c800)
        .ratio(app.life as f64 / MAX_LIFE as f64)
        .label(Span::styled(
            format!("{} / {}", app.life, MAX_LIFE),
            Style::default().fg(Color::White),
        ))
        .block(Block::default().borders(Borders::ALL).title("Health"));

    frame.render_widget(health_gauge, area);
}

impl ListStyle for Card {
    fn get_list_style(&self) -> Style {
        match self.suite {
            Suite::Spade | Suite::Club => Style::default().fg(Color::White).bg(Color::Red),
            Suite::Diamond => Style::default().fg(Color::White).bg(Color::Magenta),
            Suite::Heart => Style::default().fg(Color::White).bg(Color::Green),
        }
    }
}

impl Emoji for Card {
    fn get_emoji(&self) -> String {
        match self.suite {
            Suite::Spade | Suite::Club => "👺".to_string(),
            Suite::Diamond => "⚔️".to_string(),
            Suite::Heart => "❤️".to_string(),
        }
    }
}

pub fn render_cards(frame: &mut Frame, app: &App, area: Rect) {
    let border = Block::new()
        .borders(Borders::ALL)
        .title(Line::from("Room - select card (1/2/3/4)").centered());

    let list_items: Vec<ListItem> = app
        .hand
        .iter()
        .enumerate()
        .map(|(idx, card)| match card {
            Some(card) => ListItem::new(Text::styled(
                format!("[{}]: {} {}", idx + 1, card.to_string(), card.get_emoji()),
                card.get_list_style(),
            )),
            None => ListItem::new(Text::styled(
                "USED",
                Style::default().fg(Color::Black).bg(Gray),
            )),
        })
        .collect();

    let list_widget = List::new(list_items).block(border);
    frame.render_widget(&list_widget, centered_rect(50, 80, area));
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
