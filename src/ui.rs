use crate::app::{App, CurrentScreen};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::{DefaultTerminal, Frame};

pub fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::BeforeRoom => {
            let notification = Paragraph::new(Text::styled("Entering new room (press any button to continue)", Color::Blue));
            frame.render_widget(notification, frame.area());
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
            let list = List::new(list_items).block(block).highlight_symbol(">> ");

            frame.render_widget(list, frame.area());
        }

        // TODO: add more screens
        _ => {}
    }
}
