mod games_list;
mod gpio_port;

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use tuirealm::listener::SyncPort;
use tuirealm::ratatui::layout::{Constraint, Layout};
use tuirealm::terminal::{CrosstermTerminalAdapter, TerminalBridge};
use tuirealm::{Application, EventListenerCfg, NoUserEvent, PollStrategy};

use crate::AppState;
use crate::app_config::AppConfig;
use crate::menu::games_list::GamesList;
use crate::menu::gpio_port::GpioPort;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentId {
    GamesList,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Msg {
    None,
    Exit,
    SelectGame(usize),
}

pub struct AppMenu {
    app: Application<ComponentId, Msg, NoUserEvent>,
    config: Rc<AppConfig>,
    exit: Arc<AtomicBool>,
    games: Vec<GameEntry>,
    terminal_bridge: TerminalBridge<CrosstermTerminalAdapter>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Platform {
    GameBoy,
    GameBoyColor,
}

#[derive(Debug, Clone)]
struct GameEntry {
    name: String,
    path: PathBuf,
    platform: Platform,
}

impl AppMenu {
    pub fn new(
        config: Rc<AppConfig>,
        exit: Arc<AtomicBool>,
        event_receiver: Receiver<rboy::input::Event>,
    ) -> anyhow::Result<Self> {
        // scan directory
        let mut games = vec![];
        if let Ok(entries) = std::fs::read_dir(&config.roms_directory) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() {
                    debug!("Skipping non-file entry: {:?}", path);
                    continue;
                }
                let Some(extension) = path.extension() else {
                    warn!("File without extension: {:?}", path);
                    continue;
                };
                // get name
                let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                    warn!("Invalid file name: {:?}", path);
                    continue;
                };
                let extension = extension.to_string_lossy().to_lowercase();
                let platform = match extension.as_str() {
                    "gb" => Platform::GameBoy,
                    "gbc" => Platform::GameBoyColor,
                    _ => {
                        debug!("Unsupported file extension: {:?}", path);
                        continue;
                    }
                };
                games.push(GameEntry {
                    name: name.to_string(),
                    path,
                    platform,
                });
            }
        }

        let mut terminal_bridge = TerminalBridge::init_crossterm()?;
        let _ = terminal_bridge.disable_mouse_capture();
        let gpio_port = GpioPort::new(event_receiver);
        let gpio_port = SyncPort::new(Box::new(gpio_port), config.poll_interval(), 1);
        let app = Application::init(
            EventListenerCfg::default()
                .crossterm_input_listener(Duration::from_millis(100), 1)
                .port(gpio_port),
        );

        Ok(Self {
            app,
            config,
            exit,
            games,
            terminal_bridge,
        })
    }

    pub fn run(mut self) -> anyhow::Result<AppState> {
        let mut redraw = true;
        self.init_ui();

        loop {
            if self.exit.load(std::sync::atomic::Ordering::Relaxed) {
                return Ok(AppState::Exit);
            }

            match self.app.tick(PollStrategy::Once) {
                Ok(messages) if messages.is_empty() => {}
                Ok(messages) => {
                    redraw = true;
                    for msg in messages {
                        match msg {
                            Msg::Exit => {
                                self.stop()?;
                                return Ok(AppState::Exit);
                            }
                            Msg::SelectGame(index) => {
                                let Some(path) = self.games.get(index).map(|g| g.path.clone())
                                else {
                                    continue;
                                };
                                self.stop()?;
                                return Ok(AppState::Emulator {
                                    rom_file: path,
                                    config: self.config,
                                });
                            }
                            Msg::None => {}
                        }
                    }
                }
                Err(e) => {
                    error!("Error in TUI application: {}", e);
                }
            }

            if redraw {
                self.redraw();
                redraw = false;
            }
        }
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        self.terminal_bridge.restore()?;
        Ok(())
    }

    fn init_ui(&mut self) {
        assert!(
            self.app
                .mount(
                    ComponentId::GamesList,
                    Box::new(GamesList::new(&self.games)),
                    vec![]
                )
                .is_ok()
        );
        assert!(self.app.active(&ComponentId::GamesList).is_ok());
    }

    fn redraw(&mut self) {
        let _ = self.terminal_bridge.raw_mut().draw(|f| {
            let chunks = Layout::default()
                .direction(tuirealm::ratatui::layout::Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(100)])
                .split(f.area());

            self.app.view(&ComponentId::GamesList, f, chunks[0]);
        });
    }
}
