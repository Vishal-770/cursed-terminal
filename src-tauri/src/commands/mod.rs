pub mod enclave;
pub mod linux;
pub mod macos;
pub mod powershell;
pub mod trolls;
pub mod types;
pub mod windows;

use types::CommandResponse;

pub fn dispatch_command(state: &mut enclave::EnclaveState, cmd: String) -> CommandResponse {
    let raw = cmd.trim();
    if raw.is_empty() {
        return CommandResponse::text("");
    }

    // 0. Check Hash-Gated Secure Vault Interactions (Derive-the-Key Model)
    if let Some(resp) = enclave::check_vault_interaction(state, raw) {
        return resp;
    }

    let parts: Vec<&str> = raw.split_whitespace().collect();
    let root_cmd = parts[0];
    let args = if parts.len() > 1 {
        parts[1..].join(" ")
    } else {
        String::new()
    };

    // 1. Check Rickroll troll triggers & clear screen denial spam
    if let Some(resp) = trolls::handle_troll_command(root_cmd, raw) {
        return resp;
    }

    // 2. Check PowerShell cmdlets & $env: queries
    if let Some(resp) = powershell::handle_powershell_command(root_cmd, &args, raw) {
        return resp;
    }

    // 3. Check Windows CMD commands (dir, type, reg, tasklist, systeminfo, etc.)
    if let Some(resp) = windows::handle_windows_command(root_cmd, &args, raw) {
        return resp;
    }

    // 4. Check Linux commands (ls, cat, ps, strings, grep, find, top, df, etc.)
    if let Some(resp) = linux::handle_linux_command(root_cmd, &args, raw) {
        return resp;
    }

    // 5. Check macOS commands (sw_vers, xattr, diskutil, defaults, launchctl, say, etc.)
    if let Some(resp) = macos::handle_macos_command(root_cmd, &args, raw) {
        return resp;
    }

    // 6. Generic cross-OS gaslighting fallback
    let fallbacks = [
        format!("'{}' is not recognized as an internal or external command, operable program or batch file.", raw),
        format!("zsh: command not found: {}", raw),
        format!("bash: {}: command not found", raw),
        format!("{}: The term '{}' is not recognized as the name of a cmdlet, function, script file, or operable program.", raw, root_cmd),
    ];

    let chosen = &fallbacks[raw.len() % fallbacks.len()];
    CommandResponse::with_sound(chosen.to_string(), "tungtung")
}
