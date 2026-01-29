use color_eyre::eyre::{eyre, Result};
use std::process::Stdio;
use tokio::process::Command;

use super::types::{Cask, Package, PackageInfo};

/// Validates package names to prevent command injection and invalid operations
fn validate_package_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(eyre!("Package name cannot be empty"));
    }
    if name.contains('\0') || name.contains('\n') {
        return Err(eyre!("Invalid characters in package name"));
    }
    // Brew packages: alphanumeric + dash/underscore/@/./
    if !name.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | '@' | '/' | '.')) {
        return Err(eyre!("Package name contains invalid characters: {}", name));
    }
    Ok(())
}

pub async fn list_formulae() -> Result<Vec<Package>> {
    let output = Command::new("brew")
        .args(["info", "--json=v2", "--installed"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("Failed to list formulae: {}", stderr));
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let formulae = json
        .get("formulae")
        .and_then(|f| f.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    Ok(formulae)
}

pub async fn list_casks() -> Result<Vec<Cask>> {
    let output = Command::new("brew")
        .args(["info", "--json=v2", "--installed", "--cask"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("Failed to list casks: {}", stderr));
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    let casks = json
        .get("casks")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| serde_json::from_value(v.clone()).ok())
                .collect()
        })
        .unwrap_or_default();

    Ok(casks)
}

pub async fn get_outdated_formulae() -> Result<Vec<String>> {
    let output = Command::new("brew")
        .args(["outdated", "--formula", "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

pub async fn get_outdated_casks() -> Result<Vec<String>> {
    let output = Command::new("brew")
        .args(["outdated", "--cask", "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

pub async fn get_package_info(name: &str) -> Result<PackageInfo> {
    let output = Command::new("brew")
        .args(["info", "--json=v2", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("Failed to get package info: {}", stderr));
    }

    let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;

    // Try formulae first
    let formula = json
        .get("formulae")
        .and_then(|f| f.as_array())
        .and_then(|arr| arr.first());

    if let Some(pkg) = formula {
        let deps = get_dependencies(name).await.unwrap_or_default();
        let dependents = get_dependents(name).await.unwrap_or_default();
        let pinned = is_pinned(name).await.unwrap_or(false);

        return Ok(PackageInfo {
            name: pkg["name"].as_str().unwrap_or(name).to_string(),
            version: pkg["versions"]["stable"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            description: pkg["desc"].as_str().unwrap_or("").to_string(),
            homepage: pkg["homepage"].as_str().unwrap_or("").to_string(),
            dependencies: deps,
            dependents,
            pinned,
            outdated: pkg["outdated"].as_bool().unwrap_or(false),
        });
    }

    // Try casks
    let cask = json
        .get("casks")
        .and_then(|c| c.as_array())
        .and_then(|arr| arr.first());

    if let Some(pkg) = cask {
        return Ok(PackageInfo {
            name: pkg["token"].as_str().unwrap_or(name).to_string(),
            version: pkg["version"].as_str().unwrap_or("unknown").to_string(),
            description: pkg["desc"].as_str().unwrap_or("").to_string(),
            homepage: pkg["homepage"].as_str().unwrap_or("").to_string(),
            dependencies: vec![], // Casks don't have deps in the same way
            dependents: vec![],
            pinned: false,
            outdated: pkg["outdated"].as_bool().unwrap_or(false),
        });
    }

    Err(eyre!("Package not found: {}", name))
}

pub async fn get_dependencies(name: &str) -> Result<Vec<String>> {
    let output = Command::new("brew")
        .args(["deps", "--installed", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

pub async fn get_dependents(name: &str) -> Result<Vec<String>> {
    let output = Command::new("brew")
        .args(["uses", "--installed", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

pub async fn is_pinned(name: &str) -> Result<bool> {
    let output = Command::new("brew")
        .args(["list", "--pinned"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Ok(false);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().any(|line| line == name))
}

pub async fn update_package(name: &str, is_cask: bool) -> Result<String> {
    validate_package_name(name)?;

    let args = if is_cask {
        vec!["upgrade", "--cask", name]
    } else {
        vec!["upgrade", name]
    };

    let output = Command::new("brew")
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(eyre!("Failed to update {}: {}", name, stderr.trim()));
    }

    let pkg_type = if is_cask { "cask" } else { "formula" };
    Ok(format!("Updated {} ({})", name, pkg_type))
}

pub async fn update_all() -> Result<String> {
    let output = Command::new("brew")
        .args(["upgrade"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(eyre!("Failed to update all packages: {}", stderr));
    }

    Ok(format!("Updated all packages\n{}{}", stdout, stderr))
}

pub async fn uninstall_package(name: &str) -> Result<String> {
    validate_package_name(name)?;

    let output = Command::new("brew")
        .args(["uninstall", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(eyre!("Failed to uninstall {}: {}", name, stderr.trim()));
    }

    Ok(format!("Uninstalled {}", name))
}

pub async fn pin_package(name: &str) -> Result<String> {
    validate_package_name(name)?;

    let output = Command::new("brew")
        .args(["pin", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("Failed to pin {}: {}", name, stderr));
    }

    Ok(format!("Pinned {}", name))
}

pub async fn unpin_package(name: &str) -> Result<String> {
    validate_package_name(name)?;

    let output = Command::new("brew")
        .args(["unpin", name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("Failed to unpin {}: {}", name, stderr));
    }

    Ok(format!("Unpinned {}", name))
}

pub async fn cleanup() -> Result<String> {
    let output = Command::new("brew")
        .args(["cleanup", "--prune=all"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(eyre!("Failed to cleanup: {}", stderr));
    }

    Ok(format!("Cleanup completed\n{}{}", stdout, stderr))
}

