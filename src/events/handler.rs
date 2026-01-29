use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::actions::Action;
use crate::app::InputMode;

pub fn handle_key_event(key: KeyEvent, input_mode: &InputMode) -> Action {
    match input_mode {
        InputMode::Normal => handle_normal_mode(key),
        InputMode::Search => handle_search_mode(key),
        InputMode::Confirm => handle_confirm_mode(key),
    }
}

fn handle_normal_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('q') => Action::Quit,

        // Navigation
        KeyCode::Char('j') | KeyCode::Down => Action::NextItem,
        KeyCode::Char('k') | KeyCode::Up => Action::PreviousItem,
        KeyCode::Char('g') | KeyCode::Home => Action::FirstItem,
        KeyCode::Char('G') | KeyCode::End => Action::LastItem,
        KeyCode::PageDown | KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Action::PageDown
        }
        KeyCode::PageUp | KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            Action::PageUp
        }

        // Tabs
        KeyCode::Tab => Action::NextTab,
        KeyCode::BackTab => Action::PreviousTab,
        KeyCode::Char('1') => Action::SelectTab(0),
        KeyCode::Char('2') => Action::SelectTab(1),

        // Actions
        KeyCode::Enter | KeyCode::Char('d') => Action::ToggleDetails,
        KeyCode::Char('?') => Action::ToggleHelp,
        KeyCode::Char('/') => Action::EnterSearch,
        KeyCode::Char('o') => Action::ToggleOutdatedFilter,
        KeyCode::Char('u') => Action::UpdateSelected,
        KeyCode::Char('U') => Action::UpdateAll,
        KeyCode::Char('x') | KeyCode::Delete => Action::RemoveSelected,
        KeyCode::Char('p') => Action::TogglePin,
        KeyCode::Char('c') => Action::Cleanup,
        KeyCode::Char('r') => Action::Refresh,
        KeyCode::Char('t') => Action::ToggleDependencyTree,

        _ => Action::Noop,
    }
}

fn handle_search_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Esc => Action::ExitSearch,
        KeyCode::Enter => Action::ExitSearch,
        KeyCode::Backspace => Action::SearchBackspace,
        KeyCode::Char(c) => Action::SearchInput(c),
        _ => Action::Noop,
    }
}

fn handle_confirm_mode(key: KeyEvent) -> Action {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => Action::ConfirmAction,
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => Action::CancelAction,
        _ => Action::Noop,
    }
}
