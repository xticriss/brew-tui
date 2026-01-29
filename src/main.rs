mod app;
mod brew;
mod events;
mod state;
mod ui;

use std::io;
use std::panic;
use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use app::App;
use events::handle_key_event;

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal before printing panic info
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup panic hook to restore terminal on crash
    setup_panic_hook();

    color_eyre::install()?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();

    // Run app and capture result
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show)?;

    // Print any errors after restoring terminal
    if let Err(ref e) = result {
        eprintln!("Error: {}", e);
    }

    result
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    // Start loading packages in background
    app.start_load_packages();

    // Main event loop
    loop {
        // Update tick for animations
        app.tick();

        // Poll for background operation completion
        app.poll_operation();

        // Draw UI
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle events with timeout
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = handle_key_event(key, &app.input_mode);
                    app.handle_action(action);
                }
            }
        }

        // Check for quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}
