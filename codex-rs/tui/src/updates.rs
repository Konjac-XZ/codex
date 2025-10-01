#![cfg(any(not(debug_assertions), test))]

use codex_core::config::Config;

/// Updates are intentionally disabled in this build.
pub fn get_upgrade_version(_config: &Config) -> Option<String> {
    None
}
