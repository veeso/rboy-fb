use tui_realm_stdlib::List;
use tuirealm::command::{Cmd, Direction};
use tuirealm::event::{Key, KeyEvent};
use tuirealm::props::Borders;
use tuirealm::{Component, MockComponent, NoUserEvent};

use crate::menu::{GameEntry, Msg, Platform};

#[derive(MockComponent)]
pub struct GamesList {
    component: List,
}

impl GamesList {
    pub fn new(games: &[GameEntry]) -> Self {
        let mut rows = vec![];
        for game in games {
            let mut row = vec![];
            row.push(game.name.clone().into());
            row.push(" - ".to_string().into());
            match game.platform {
                Platform::GameBoy => row.push("GB".to_string().into()),
                Platform::GameBoyColor => row.push("GBC".to_string().into()),
            }
            rows.push(row);
        }

        Self {
            component: List::default()
                .title("Select a game", tuirealm::props::Alignment::Left)
                .borders(Borders::default())
                .highlighted_color(tuirealm::props::Color::LightYellow)
                .highlighted_str("> ")
                .rewind(true)
                .scroll(true)
                .step(1)
                .rows(rows),
        }
    }
}

impl Component<Msg, NoUserEvent> for GamesList {
    fn on(&mut self, ev: tuirealm::Event<NoUserEvent>) -> Option<Msg> {
        match ev {
            tuirealm::Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Move(Direction::Down)),
            tuirealm::Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(Direction::Up))
            }
            tuirealm::Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                let selected = self.component.states.list_index;
                return Some(Msg::SelectGame(selected));
            }
            tuirealm::Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                return Some(Msg::Exit);
            }
            _ => return None,
        };

        Some(Msg::None)
    }
}
