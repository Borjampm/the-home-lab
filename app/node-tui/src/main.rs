use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{Event, KeyEventKind};
use tokio::sync::mpsc;

mod app;
mod event;
mod grpc;
mod ui;

use app::{Action, App};
use event::AppEvent;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let (tx, mut rx) = mpsc::channel::<AppEvent>(32);
    let mut app = App::new();

    grpc::spawn_cpu_stream(tx.clone());

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if ratatui::crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = ratatui::crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    if let Some(action) = app.handle_key(key) {
                        match action {
                            Action::Quit => break,
                            Action::Reconnect => grpc::spawn_cpu_stream(tx.clone()),
                            Action::SendGreeting(name) => grpc::send_greeting(name, tx.clone()),
                        }
                    }
                }
            }
        }

        while let Ok(ev) = rx.try_recv() {
            app.apply_event(ev);
        }
    }

    ratatui::restore();
    Ok(())
}
