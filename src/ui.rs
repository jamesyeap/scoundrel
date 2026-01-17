use crate::app::{App, CurrentScreen};
use ratatui::layout::Constraint::Percentage;
use ratatui::prelude::{Color, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::BeforeRoom => {
            let block = Block::default().borders(Borders::ALL).title("Avoid room? (y/n)");

            let mut list_items = Vec::new();
            for card in app.hand.iter() {
                match card {
                    Some(card) => list_items.push(ListItem::new(Text::styled(
                        card.to_string(),
                        Style::default().fg(Color::Blue),
                    ))),
                    None => list_items.push(ListItem::new(Text::styled(
                        "USED",
                        Style::default().fg(Color::DarkGray),
                    ))),
                }
            }
            let list = List::new(list_items).block(block);

            frame.render_widget(list, frame.area());
        }

        CurrentScreen::ChooseCard => {
            let block = Block::default().borders(Borders::ALL).title("Scoundrel");

            let mut list_items = Vec::new();
            for card in app.hand.iter() {
                match card {
                    Some(card) => list_items.push(ListItem::new(Text::styled(
                        card.to_string(),
                        Style::default().fg(Color::Blue),
                    ))),
                    None => list_items.push(ListItem::new(Text::styled(
                        "USED",
                        Style::default().fg(Color::DarkGray),
                    ))),
                }
            }
            let list = List::new(list_items).block(block);

            frame.render_widget(list, frame.area());
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
            )).block(Block::default().borders(Borders::ALL).title_bottom("Your equipped weapon"));
            let creature = Paragraph::new(Text::styled(
                format!("{}", app.in_combat_with_creature.as_ref().unwrap()),
                Style::default().fg(Color::Blue),
            )).block(Block::default().borders(Borders::ALL).title_bottom("Creature"));

            frame.render_widget(weapon, chunks[0]);
            frame.render_widget(creature, chunks[1]);
        }

        _ => {}
    }
}
