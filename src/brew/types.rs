use serde::{Deserialize, Serialize};

/// Trait for common package display operations
pub trait PackageDisplay {
    fn display_name(&self) -> &str;
    fn display_version(&self) -> &str;
    fn display_description(&self) -> &str;
    fn status_display(&self) -> &str;
    fn is_outdated(&self) -> bool;
    fn is_pinned(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub full_name: String,
    #[serde(default)]
    pub versions: Versions,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub pinned: bool,
    #[serde(default)]
    pub outdated: bool,
    #[serde(default)]
    pub installed: Vec<InstalledVersion>,
    #[serde(default)]
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Versions {
    pub stable: Option<String>,
    pub head: Option<String>,
    #[serde(default)]
    pub bottle: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledVersion {
    pub version: String,
    #[serde(default)]
    pub installed_on_request: bool,
    #[serde(default)]
    pub installed_as_dependency: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cask {
    pub token: String,
    #[serde(default)]
    pub name: Vec<String>,
    pub version: String,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub outdated: bool,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub pinned: bool,
    pub outdated: bool,
}

impl PackageDisplay for Package {
    fn display_name(&self) -> &str {
        &self.name
    }

    fn display_version(&self) -> &str {
        // First try installed version, then stable version
        if let Some(installed) = self.installed.first() {
            &installed.version
        } else {
            self.versions.stable.as_deref().unwrap_or("unknown")
        }
    }

    fn display_description(&self) -> &str {
        self.desc.as_deref().unwrap_or("")
    }

    fn status_display(&self) -> &str {
        use crate::ui::constants::{STATUS_CURRENT, STATUS_OUTDATED, STATUS_PINNED};
        if self.pinned {
            STATUS_PINNED
        } else if self.outdated {
            STATUS_OUTDATED
        } else {
            STATUS_CURRENT
        }
    }

    fn is_outdated(&self) -> bool {
        self.outdated
    }

    fn is_pinned(&self) -> bool {
        self.pinned
    }
}

#[derive(Debug, Clone)]
pub struct DependencyTreeData {
    pub package_name: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
}

impl PackageDisplay for Cask {
    fn display_name(&self) -> &str {
        self.name.first().map(|s| s.as_str()).unwrap_or(&self.token)
    }

    fn display_version(&self) -> &str {
        &self.version
    }

    fn display_description(&self) -> &str {
        self.desc.as_deref().unwrap_or("")
    }

    fn status_display(&self) -> &str {
        use crate::ui::constants::{STATUS_CURRENT, STATUS_OUTDATED};
        if self.outdated {
            STATUS_OUTDATED
        } else {
            STATUS_CURRENT
        }
    }

    fn is_outdated(&self) -> bool {
        self.outdated
    }
}
