use ratatui::widgets::TableState;
use tokio::sync::mpsc;

use crate::brew::{self, Cask, DependencyTreeData, Package, PackageInfo};
use crate::events::{Action, ConfirmAction};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum DependencyTreeMode {
    #[default]
    Dependencies,
    Dependents,
}

pub type OperationResult = Result<OperationSuccess, String>;

#[derive(Debug, Clone)]
pub enum OperationSuccess {
    Message(String),
    PackagesLoaded { packages: Vec<Package>, casks: Vec<Cask> },
    PackageInfo(Box<PackageInfo>),
    PinToggled { name: String, pinned: bool },
    DependencyTree(DependencyTreeData),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Tab {
    #[default]
    Formulae,
    Casks,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
    Confirm,
}

struct FilterCache {
    filter_text: String,
    show_outdated: bool,
    package_indices: Vec<usize>,
    cask_indices: Vec<usize>,
}

pub struct App {
    pub packages: Vec<Package>,
    pub casks: Vec<Cask>,
    pub active_tab: Tab,
    pub formulae_state: TableState,
    pub casks_state: TableState,
    pub filter: String,
    pub show_outdated_only: bool,
    pub show_help: bool,
    pub show_details: bool,
    pub show_dependency_tree: bool,
    pub dependency_tree_mode: DependencyTreeMode,
    pub dependency_tree_data: Option<DependencyTreeData>,
    pub input_mode: InputMode,
    pub pending_action: Option<ConfirmAction>,
    pub loading: bool,
    pub status_message: Option<String>,
    pub selected_package_info: Option<PackageInfo>,
    pub should_quit: bool,
    pub tick: usize,
    pub operation_rx: Option<mpsc::Receiver<OperationResult>>,
    filter_cache: Option<FilterCache>,
}

impl Default for App {
    fn default() -> Self {
        let mut formulae_state = TableState::default();
        formulae_state.select(Some(0));

        let mut casks_state = TableState::default();
        casks_state.select(Some(0));

        Self {
            packages: Vec::new(),
            casks: Vec::new(),
            active_tab: Tab::Formulae,
            formulae_state,
            casks_state,
            filter: String::new(),
            show_outdated_only: false,
            show_help: false,
            show_details: false,
            show_dependency_tree: false,
            dependency_tree_mode: DependencyTreeMode::default(),
            dependency_tree_data: None,
            input_mode: InputMode::Normal,
            pending_action: None,
            loading: false,
            status_message: None,
            selected_package_info: None,
            should_quit: false,
            tick: 0,
            operation_rx: None,
            filter_cache: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn spinner_char(&self) -> char {
        const SPINNER: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        SPINNER[self.tick % SPINNER.len()]
    }

    /// Check if a background operation has completed
    pub fn poll_operation(&mut self) {
        if let Some(rx) = &mut self.operation_rx {
            match rx.try_recv() {
                Ok(result) => {
                    self.loading = false;
                    self.operation_rx = None;
                    match result {
                        Ok(success) => {
                            match success {
                                OperationSuccess::Message(msg) => {
                                    self.status_message = Some(msg);
                                    // Trigger a refresh after package operations
                                    self.start_load_packages();
                                }
                                OperationSuccess::PackagesLoaded { packages, casks } => {
                                    self.packages = packages;
                                    self.casks = casks;
                                    self.invalidate_filter_cache();
                                    self.status_message = None;
                                    self.validate_selection();
                                }
                                OperationSuccess::PackageInfo(info) => {
                                    self.selected_package_info = Some(*info);
                                }
                                OperationSuccess::PinToggled { name, pinned } => {
                                    if let Some(pkg) = self.packages.iter_mut().find(|p| p.name == name) {
                                        pkg.pinned = pinned;
                                    }
                                    let action = if pinned { "Pinned" } else { "Unpinned" };
                                    self.status_message = Some(format!("{} {}", action, name));
                                }
                                OperationSuccess::DependencyTree(data) => {
                                    self.dependency_tree_data = Some(data);
                                    self.status_message = None;
                                }
                            }
                        }
                        Err(e) => {
                            self.status_message = Some(format!("Error: {}", e));
                        }
                    }
                }
                Err(mpsc::error::TryRecvError::Empty) => {
                    // Still running, nothing to do
                }
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    // Task crashed or was dropped
                    self.loading = false;
                    self.operation_rx = None;
                    self.status_message = Some("Operation failed unexpectedly".to_string());
                }
            }
        }
    }

    /// Start loading packages in the background
    pub fn start_load_packages(&mut self) {
        if self.loading {
            return; // Don't start if already loading
        }

        self.loading = true;
        self.status_message = Some("Loading packages...".to_string());

        let (tx, rx) = mpsc::channel(1);
        self.operation_rx = Some(rx);

        tokio::spawn(async move {
            let result = Self::load_packages_async().await;
            let _ = tx.send(result).await;
        });
    }

    async fn load_packages_async() -> OperationResult {
        // Parallel loading - all 4 brew calls execute concurrently
        let (formulae_result, casks_result, outdated_f_result, outdated_c_result) = tokio::join!(
            brew::list_formulae(),
            brew::list_casks(),
            brew::get_outdated_formulae(),
            brew::get_outdated_casks()
        );

        // Process formulae
        let packages = formulae_result
            .map_err(|e| format!("Error loading formulae: {}", e))?;
        let outdated_f = outdated_f_result.unwrap_or_default();

        let packages: Vec<Package> = packages
            .into_iter()
            .map(|mut p| {
                p.outdated = outdated_f.contains(&p.name);
                p
            })
            .collect();

        // Process casks
        let casks = casks_result
            .map_err(|e| format!("Error loading casks: {}", e))?;
        let outdated_c = outdated_c_result.unwrap_or_default();

        let casks: Vec<Cask> = casks
            .into_iter()
            .map(|mut c| {
                c.outdated = outdated_c.contains(&c.token);
                c
            })
            .collect();

        Ok(OperationSuccess::PackagesLoaded { packages, casks })
    }

    pub fn filtered_packages(&mut self) -> Vec<&Package> {
        // Check cache validity
        if let Some(cache) = &self.filter_cache {
            if cache.filter_text == self.filter && cache.show_outdated == self.show_outdated_only {
                return cache.package_indices.iter()
                    .map(|&i| &self.packages[i])
                    .collect();
            }
        }

        // Rebuild cache
        let filter_lower = self.filter.to_lowercase();
        let indices: Vec<usize> = self.packages
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                let matches_search = if self.filter.is_empty() {
                    true
                } else {
                    p.name.to_lowercase().contains(&filter_lower)
                        || p.desc
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter_lower))
                            .unwrap_or(false)
                };
                let matches_outdated = !self.show_outdated_only || p.outdated;
                matches_search && matches_outdated
            })
            .map(|(i, _)| i)
            .collect();

        let result = indices.iter().map(|&i| &self.packages[i]).collect();

        // Update cache (preserve cask_indices if cache existed)
        let cask_indices = self.filter_cache.as_ref()
            .map(|c| c.cask_indices.clone())
            .unwrap_or_default();

        self.filter_cache = Some(FilterCache {
            filter_text: self.filter.clone(),
            show_outdated: self.show_outdated_only,
            package_indices: indices,
            cask_indices,
        });

        result
    }

    pub fn filtered_casks(&mut self) -> Vec<&Cask> {
        // Check cache validity
        if let Some(cache) = &self.filter_cache {
            if cache.filter_text == self.filter && cache.show_outdated == self.show_outdated_only {
                return cache.cask_indices.iter()
                    .map(|&i| &self.casks[i])
                    .collect();
            }
        }

        // Rebuild cache
        let filter_lower = self.filter.to_lowercase();
        let indices: Vec<usize> = self.casks
            .iter()
            .enumerate()
            .filter(|(_, c)| {
                let matches_search = if self.filter.is_empty() {
                    true
                } else {
                    c.token.to_lowercase().contains(&filter_lower)
                        || c.name.iter().any(|n| n.to_lowercase().contains(&filter_lower))
                        || c.desc
                            .as_ref()
                            .map(|d| d.to_lowercase().contains(&filter_lower))
                            .unwrap_or(false)
                };
                let matches_outdated = !self.show_outdated_only || c.outdated;
                matches_search && matches_outdated
            })
            .map(|(i, _)| i)
            .collect();

        let result = indices.iter().map(|&i| &self.casks[i]).collect();

        // Update cache (preserve package_indices if cache existed)
        let package_indices = self.filter_cache.as_ref()
            .map(|c| c.package_indices.clone())
            .unwrap_or_default();

        self.filter_cache = Some(FilterCache {
            filter_text: self.filter.clone(),
            show_outdated: self.show_outdated_only,
            package_indices,
            cask_indices: indices,
        });

        result
    }

    fn invalidate_filter_cache(&mut self) {
        self.filter_cache = None;
    }

    fn current_list_len(&mut self) -> usize {
        match self.active_tab {
            Tab::Formulae => self.filtered_packages().len(),
            Tab::Casks => self.filtered_casks().len(),
        }
    }

    fn current_state(&mut self) -> &mut TableState {
        match self.active_tab {
            Tab::Formulae => &mut self.formulae_state,
            Tab::Casks => &mut self.casks_state,
        }
    }

    fn validate_selection(&mut self) {
        let len = self.current_list_len();
        if len == 0 {
            self.current_state().select(None);
        } else {
            let selected = self.current_state().selected().unwrap_or(0);
            if selected >= len {
                self.current_state().select(Some(len.saturating_sub(1)));
            }
        }
    }

    pub fn selected_package_name(&mut self) -> Option<String> {
        match self.active_tab {
            Tab::Formulae => {
                let selected = self.formulae_state.selected()?;
                let packages = self.filtered_packages();
                packages.get(selected).map(|p| p.name.clone())
            }
            Tab::Casks => {
                let selected = self.casks_state.selected()?;
                let casks = self.filtered_casks();
                casks.get(selected).map(|c| c.token.clone())
            }
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,

            // Navigation
            Action::NextItem => self.next_item(),
            Action::PreviousItem => self.previous_item(),
            Action::FirstItem => self.first_item(),
            Action::LastItem => self.last_item(),
            Action::PageDown => self.page_down(),
            Action::PageUp => self.page_up(),

            // Tabs (Tab also toggles dependency tree mode when tree is shown)
            Action::NextTab => {
                if self.show_dependency_tree {
                    self.toggle_dependency_tree_mode();
                } else {
                    self.next_tab();
                }
            }
            Action::PreviousTab => self.previous_tab(),
            Action::SelectTab(index) => self.select_tab(index),

            // UI toggles
            Action::ToggleDetails => self.toggle_details(),
            Action::ToggleHelp => self.show_help = !self.show_help,
            Action::ToggleDependencyTree => self.toggle_dependency_tree(),
            Action::ToggleOutdatedFilter => {
                self.show_outdated_only = !self.show_outdated_only;
                self.invalidate_filter_cache();
                self.validate_selection();
            }

            // Search
            Action::EnterSearch => {
                self.input_mode = InputMode::Search;
                self.filter.clear();
            }
            Action::ExitSearch => {
                self.input_mode = InputMode::Normal;
            }
            Action::SearchInput(c) => {
                self.filter.push(c);
                self.invalidate_filter_cache();
                self.validate_selection();
            }
            Action::SearchBackspace => {
                self.filter.pop();
                self.invalidate_filter_cache();
                self.validate_selection();
            }

            // Package operations
            Action::UpdateSelected => {
                if let Some(name) = self.selected_package_name() {
                    let is_cask = matches!(self.active_tab, Tab::Casks);
                    self.pending_action = Some(ConfirmAction::Update { name, is_cask });
                    self.input_mode = InputMode::Confirm;
                    self.show_help = false;
                }
            }
            Action::UpdateAll => {
                self.pending_action = Some(ConfirmAction::UpdateAll);
                self.input_mode = InputMode::Confirm;
                self.show_help = false;
            }
            Action::RemoveSelected => {
                if let Some(name) = self.selected_package_name() {
                    let is_cask = matches!(self.active_tab, Tab::Casks);
                    self.pending_action = Some(ConfirmAction::Remove { name, is_cask });
                    self.input_mode = InputMode::Confirm;
                    self.show_help = false;
                }
            }
            Action::Cleanup => {
                self.pending_action = Some(ConfirmAction::Cleanup);
                self.input_mode = InputMode::Confirm;
                self.show_help = false;
            }
            Action::TogglePin => self.toggle_pin(),
            Action::Refresh => {
                self.start_load_packages();
            }

            // Confirm dialog
            Action::ConfirmAction => {
                self.execute_pending_action();
                self.pending_action = None;
                self.input_mode = InputMode::Normal;
            }
            Action::CancelAction => {
                self.pending_action = None;
                self.input_mode = InputMode::Normal;
            }

            Action::Noop => {}
        }
    }

    fn next_item(&mut self) {
        let len = self.current_list_len();
        if len == 0 {
            return;
        }

        let state = self.current_state();
        let current = state.selected().unwrap_or(0);
        let next = if current >= len - 1 { 0 } else { current + 1 };
        state.select(Some(next));
    }

    fn previous_item(&mut self) {
        let len = self.current_list_len();
        if len == 0 {
            return;
        }

        let state = self.current_state();
        let current = state.selected().unwrap_or(0);
        let prev = if current == 0 { len - 1 } else { current - 1 };
        state.select(Some(prev));
    }

    fn first_item(&mut self) {
        if self.current_list_len() > 0 {
            self.current_state().select(Some(0));
        }
    }

    fn last_item(&mut self) {
        let len = self.current_list_len();
        if len > 0 {
            self.current_state().select(Some(len - 1));
        }
    }

    fn page_down(&mut self) {
        let len = self.current_list_len();
        if len == 0 {
            return;
        }

        let state = self.current_state();
        let current = state.selected().unwrap_or(0);
        let next = (current + 10).min(len - 1);
        state.select(Some(next));
    }

    fn page_up(&mut self) {
        let len = self.current_list_len();
        if len == 0 {
            return;
        }

        let state = self.current_state();
        let current = state.selected().unwrap_or(0);
        let prev = current.saturating_sub(10);
        state.select(Some(prev));
    }

    fn next_tab(&mut self) {
        self.active_tab = match self.active_tab {
            Tab::Formulae => Tab::Casks,
            Tab::Casks => Tab::Formulae,
        };
        self.selected_package_info = None;
    }

    fn previous_tab(&mut self) {
        self.next_tab(); // Only 2 tabs, so same as next
    }

    fn select_tab(&mut self, index: usize) {
        self.active_tab = match index {
            0 => Tab::Formulae,
            _ => Tab::Casks,
        };
        self.selected_package_info = None;
    }

    fn toggle_details(&mut self) {
        self.show_details = !self.show_details;

        if self.show_details {
            self.start_load_details();
        } else {
            self.selected_package_info = None;
        }
    }

    fn start_load_details(&mut self) {
        let Some(name) = self.selected_package_name() else {
            return;
        };

        if self.loading {
            return;
        }

        self.loading = true;
        self.status_message = Some(format!("Loading details for {}...", name));

        let (tx, rx) = mpsc::channel(1);
        self.operation_rx = Some(rx);

        tokio::spawn(async move {
            let result = match brew::get_package_info(&name).await {
                Ok(info) => Ok(OperationSuccess::PackageInfo(Box::new(info))),
                Err(e) => Err(e.to_string()),
            };
            let _ = tx.send(result).await;
        });
    }

    fn toggle_pin(&mut self) {
        if !matches!(self.active_tab, Tab::Formulae) {
            return;
        }

        let Some(name) = self.selected_package_name() else {
            return;
        };

        if self.loading {
            return;
        }

        let is_pinned = self
            .packages
            .iter()
            .find(|p| p.name == name)
            .map(|p| p.pinned)
            .unwrap_or(false);

        self.loading = true;
        let action = if is_pinned { "Unpinning" } else { "Pinning" };
        self.status_message = Some(format!("{} {}...", action, name));

        let (tx, rx) = mpsc::channel(1);
        self.operation_rx = Some(rx);
        let name_clone = name.clone();

        tokio::spawn(async move {
            let result = if is_pinned {
                brew::unpin_package(&name_clone).await
            } else {
                brew::pin_package(&name_clone).await
            };

            let op_result = match result {
                Ok(_) => Ok(OperationSuccess::PinToggled {
                    name: name_clone,
                    pinned: !is_pinned,
                }),
                Err(e) => Err(e.to_string()),
            };
            let _ = tx.send(op_result).await;
        });
    }

    fn toggle_dependency_tree(&mut self) {
        self.show_dependency_tree = !self.show_dependency_tree;

        if self.show_dependency_tree {
            // Close details panel if open
            self.show_details = false;
            self.selected_package_info = None;
            self.start_load_dependency_tree();
        } else {
            self.dependency_tree_data = None;
        }
    }

    fn start_load_dependency_tree(&mut self) {
        let Some(name) = self.selected_package_name() else {
            return;
        };

        if self.loading {
            return;
        }

        self.loading = true;
        self.status_message = Some(format!("Loading dependencies for {}...", name));

        let (tx, rx) = mpsc::channel(1);
        self.operation_rx = Some(rx);

        tokio::spawn(async move {
            let dependencies = brew::get_dependencies(&name).await.unwrap_or_default();
            let dependents = brew::get_dependents(&name).await.unwrap_or_default();

            let data = DependencyTreeData {
                package_name: name,
                dependencies,
                dependents,
            };

            let _ = tx.send(Ok(OperationSuccess::DependencyTree(data))).await;
        });
    }

    pub fn toggle_dependency_tree_mode(&mut self) {
        self.dependency_tree_mode = match self.dependency_tree_mode {
            DependencyTreeMode::Dependencies => DependencyTreeMode::Dependents,
            DependencyTreeMode::Dependents => DependencyTreeMode::Dependencies,
        };
    }

    fn execute_pending_action(&mut self) {
        let Some(action) = self.pending_action.clone() else {
            return;
        };

        if self.loading {
            return;
        }

        self.loading = true;

        // Show immediate feedback
        let operation_msg = match &action {
            ConfirmAction::Update { name, is_cask } => {
                let pkg_type = if *is_cask { "cask" } else { "formula" };
                format!("Updating {} ({})...", name, pkg_type)
            }
            ConfirmAction::UpdateAll => "Updating all packages...".to_string(),
            ConfirmAction::Remove { name, .. } => format!("Removing {}...", name),
            ConfirmAction::Cleanup => "Running cleanup...".to_string(),
        };
        self.status_message = Some(operation_msg);

        // Create channel for receiving result
        let (tx, rx) = mpsc::channel(1);
        self.operation_rx = Some(rx);

        // Spawn background task
        tokio::spawn(async move {
            let result = match action {
                ConfirmAction::Update { name, is_cask } => {
                    brew::update_package(&name, is_cask).await
                }
                ConfirmAction::UpdateAll => brew::update_all().await,
                ConfirmAction::Remove { name, .. } => brew::uninstall_package(&name).await,
                ConfirmAction::Cleanup => brew::cleanup().await,
            };

            let op_result = result
                .map(OperationSuccess::Message)
                .map_err(|e| e.to_string());

            let _ = tx.send(op_result).await;
        });
    }
}
