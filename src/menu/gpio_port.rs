use std::sync::mpsc::Receiver;

use rboy::input::Event;
use tuirealm::NoUserEvent;
use tuirealm::event::{Event as TuiEvent, Key as TuiKey, KeyEvent as TuiKeyEvent, KeyModifiers};
use tuirealm::listener::{ListenerResult, Poll};

pub struct GpioPort {
    event_receiver: Receiver<Event>,
}

impl GpioPort {
    pub fn new(event_receiver: Receiver<Event>) -> Self {
        Self { event_receiver }
    }
}

impl Poll<NoUserEvent> for GpioPort {
    fn poll(&mut self) -> ListenerResult<Option<tuirealm::Event<NoUserEvent>>> {
        let Ok((event, key)) = self.event_receiver.try_recv() else {
            return Ok(None);
        };

        if event != crate::KeyEvent::Down {
            return Ok(None);
        }

        match key {
            rboy::KeypadKey::A | rboy::KeypadKey::Start => Ok(Some(TuiEvent::Keyboard(
                TuiKeyEvent::new(TuiKey::Enter, KeyModifiers::NONE),
            ))),
            rboy::KeypadKey::B | rboy::KeypadKey::Select => Ok(Some(TuiEvent::Keyboard(
                TuiKeyEvent::new(TuiKey::Esc, KeyModifiers::NONE),
            ))),
            rboy::KeypadKey::Up => Ok(Some(TuiEvent::Keyboard(TuiKeyEvent::new(
                TuiKey::Up,
                KeyModifiers::NONE,
            )))),
            rboy::KeypadKey::Down => Ok(Some(TuiEvent::Keyboard(TuiKeyEvent::new(
                TuiKey::Down,
                KeyModifiers::NONE,
            )))),
            _ => Ok(None),
        }
    }
}
