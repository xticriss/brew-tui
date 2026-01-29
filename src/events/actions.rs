#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
    NextItem,
    PreviousItem,
    FirstItem,
    LastItem,
    NextTab,
    PreviousTab,
    SelectTab(usize),
    ToggleDetails,
    ToggleHelp,
    ToggleOutdatedFilter,
    EnterSearch,
    ExitSearch,
    SearchInput(char),
    SearchBackspace,
    UpdateSelected,
    UpdateAll,
    RemoveSelected,
    ConfirmAction,
    CancelAction,
    TogglePin,
    Cleanup,
    Refresh,
    ToggleDependencyTree,
    PageDown,
    PageUp,
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfirmAction {
    Update { name: String, is_cask: bool },
    UpdateAll,
    Remove { name: String, is_cask: bool },
    Cleanup,
}

impl ConfirmAction {
    pub fn message(&self) -> String {
        match self {
            ConfirmAction::Update { name, is_cask } => {
                let pkg_type = if *is_cask { "cask" } else { "formula" };
                format!("Update {} ({})? ", name, pkg_type)
            }
            ConfirmAction::UpdateAll => "Update all outdated packages?".to_string(),
            ConfirmAction::Remove { name, is_cask } => {
                let pkg_type = if *is_cask { "cask" } else { "formula" };
                format!("Remove {} ({})? This cannot be undone.", name, pkg_type)
            }
            ConfirmAction::Cleanup => "Remove all old versions and clear cache?".to_string(),
        }
    }
}
