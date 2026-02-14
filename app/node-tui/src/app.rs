use std::collections::VecDeque;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::event::AppEvent;

#[derive(PartialEq)]
pub enum Panel {
    Cpu,
    Greeter,
}

pub enum Action {
    Quit,
    Reconnect,
    SendGreeting(String),
}

pub struct App {
    pub active_panel: Panel,
    pub connected: bool,
    pub connection_error: Option<String>,
    pub cpu_count: u64,
    pub cpu_usage: f32,
    pub cpu_history: VecDeque<u64>,
    pub greeter_input: String,
    pub greeter_response: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            active_panel: Panel::Cpu,
            connected: false,
            connection_error: None,
            cpu_count: 0,
            cpu_usage: 0.0,
            cpu_history: VecDeque::with_capacity(120),
            greeter_input: String::from("world"),
            greeter_response: None,
        }
    }

    pub fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::CpuUpdate {
                cpu_count,
                total_usage,
            } => {
                self.connected = true;
                self.connection_error = None;
                self.cpu_count = cpu_count;
                self.cpu_usage = total_usage;
                if self.cpu_history.len() >= 120 {
                    self.cpu_history.pop_front();
                }
                self.cpu_history.push_back(total_usage as u64);
            }
            AppEvent::GreeterResponse(msg) => {
                self.greeter_response = Some(msg);
            }
            AppEvent::Disconnected(err) => {
                self.connected = false;
                self.connection_error = Some(err);
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<Action> {
        // Global keys — work regardless of active panel
        match key.code {
            KeyCode::Esc => return Some(Action::Quit),
            KeyCode::Tab => {
                self.active_panel = match self.active_panel {
                    Panel::Cpu => Panel::Greeter,
                    Panel::Greeter => Panel::Cpu,
                };
                return None;
            }
            KeyCode::Char('c') if self.active_panel == Panel::Cpu => {
                return Some(Action::Reconnect);
            }
            _ => {}
        }

        // Panel-specific keys
        match self.active_panel {
            Panel::Cpu => match key.code {
                KeyCode::Char('q') => Some(Action::Quit),
                _ => None,
            },
            Panel::Greeter => match key.code {
                KeyCode::Enter => Some(Action::SendGreeting(self.greeter_input.clone())),
                KeyCode::Backspace => {
                    self.greeter_input.pop();
                    None
                }
                KeyCode::Char(c) => {
                    self.greeter_input.push(c);
                    None
                }
                _ => None,
            },
        }
    }
}
