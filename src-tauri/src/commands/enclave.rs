use sha2::{Sha256, Digest};
use super::types::CommandResponse;

const SALT: &[u8] = b"CURSED_OS_KERNEL_SALT_2026_v4";

// Target Hashes for the 4 OS interactions (SHA-256 of normalized commands)
// Pre-images DO NOT exist anywhere in the binary:
// Vault 1: "reg query hklm\\software\\policies\\vault"
const HASH_VAULT_1: [u8; 32] = [
    61, 146, 224, 130, 58, 237, 5, 161, 144, 102, 107, 40, 96, 254, 220, 27,
    53, 208, 230, 95, 157, 212, 41, 145, 191, 17, 4, 246, 163, 243, 35, 161
];

// Vault 2: "cat /proc/808/environ"
const HASH_VAULT_2: [u8; 32] = [
    113, 205, 30, 62, 132, 234, 187, 113, 220, 161, 145, 114, 235, 180, 244, 107,
    131, 52, 63, 108, 114, 27, 8, 31, 70, 25, 69, 36, 107, 49, 123, 185
];

// Vault 3: "xattr -p com.apple.security.token /volumes/macintoshhd/quarantine.plist"
const HASH_VAULT_3: [u8; 32] = [
    99, 73, 30, 169, 43, 103, 3, 128, 32, 13, 169, 204, 75, 28, 141, 43,
    9, 238, 251, 213, 59, 252, 35, 220, 181, 170, 179, 53, 128, 53, 27, 105
];

// Vault 4: "$env:flag_final"
const HASH_VAULT_4: [u8; 32] = [
    194, 171, 3, 63, 53, 222, 187, 130, 88, 240, 75, 154, 238, 205, 32, 196,
    111, 38, 200, 0, 175, 152, 93, 34, 33, 188, 113, 13, 236, 64, 138, 22
];

// Hardware-enclave encrypted ciphertext
const CIPHERTEXT: [u8; 34] = [
    215, 53, 56, 42, 178, 216, 31, 110, 72, 31, 51, 241, 236, 87, 113, 235,
    96, 76, 33, 150, 199, 214, 181, 113, 216, 59, 96, 183, 57, 6, 16, 112, 171, 28
];

#[derive(Default)]
pub struct EnclaveState {
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
}

impl EnclaveState {
    pub fn count_unlocked(&self) -> usize {
        let mut count = 0;
        if self.v1.is_some() { count += 1; }
        if self.v2.is_some() { count += 1; }
        if self.v3.is_some() { count += 1; }
        if self.v4.is_some() { count += 1; }
        count
    }

    pub fn is_fully_unlocked(&self) -> bool {
        self.v1.is_some() && self.v2.is_some() && self.v3.is_some() && self.v4.is_some()
    }

    pub fn try_decrypt_flag(&self) -> Option<String> {
        if let (Some(c1), Some(c2), Some(c3), Some(c4)) = (&self.v1, &self.v2, &self.v3, &self.v4) {
            let mut hasher = Sha256::new();
            hasher.update(SALT);
            hasher.update(c1.as_bytes());
            hasher.update(c2.as_bytes());
            hasher.update(c3.as_bytes());
            hasher.update(c4.as_bytes());
            let key = hasher.finalize();

            let mut decrypted = vec![0u8; CIPHERTEXT.len()];
            for i in 0..CIPHERTEXT.len() {
                decrypted[i] = CIPHERTEXT[i] ^ key[i % key.len()];
            }

            String::from_utf8(decrypted).ok()
        } else {
            None
        }
    }
}

pub fn normalize_cmd(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

pub fn check_vault_interaction(state: &mut EnclaveState, raw_cmd: &str) -> Option<CommandResponse> {
    let normalized = normalize_cmd(raw_cmd);
    
    // Windows normalization: also check slash conversion
    let win_normalized = normalized.replace('/', "\\");

    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let hash: [u8; 32] = hasher.finalize().into();

    let mut win_hasher = Sha256::new();
    win_hasher.update(win_normalized.as_bytes());
    let win_hash: [u8; 32] = win_hasher.finalize().into();

    // 1. Check Vault 1 (Windows Registry)
    if hash == HASH_VAULT_1 || win_hash == HASH_VAULT_1 {
        state.v1 = Some("reg query hklm\\software\\policies\\vault".to_string());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "HKEY_LOCAL_MACHINE\\Software\\Policies\\Vault\n    \
            EnclaveStatus    REG_SZ    SECTOR_1_AUTHENTICATED\n    \
            DiagnosticTrace  REG_SZ    /opt/kernel/telemetry_broker.bin (PID 808)\n\n\
            [+] Windows Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 2. Check Vault 2 (Linux Process Environment)
    if hash == HASH_VAULT_2 {
        state.v2 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "DAEMON_IDENTITY=\"telemetry_broker\"\n\
            MEMORY_HASH=\"SHA256_VERIFIED\"\n\
            NEXT_MOUNT=\"/Volumes/MacintoshHD/Quarantine.plist\"\n\n\
            [+] Linux Kernel Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 3. Check Vault 3 (macOS Extended Attribute)
    if hash == HASH_VAULT_3 {
        state.v3 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "com.apple.security.token: APFS_EXTENDED_ATTRIBUTE_ATTESTED\n\
            RUNSPACE_TARGET: $env:FLAG_FINAL\n\n\
            [+] Darwin Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 4. Check Vault 4 (PowerShell Runspace)
    if hash == HASH_VAULT_4 {
        state.v4 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "Name                           Value\n\
            ----                           -----\n\
            FLAG_FINAL                     RUNSPACE_TOKEN_VERIFIED\n\n\
            [+] PowerShell Core Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    None
}

fn generate_victory_screen(state: &EnclaveState) -> CommandResponse {
    let flag = state.try_decrypt_flag().unwrap_or_else(|| "DECRYPTION_ERROR".to_string());
    CommandResponse::with_sound(
        format!(
            "======================================================================\n\
            [ALL 4 OS SECTORS AUTHENTICATED // HARDWARE CLEARANCE GRANTED]\n\
            ======================================================================\n\
            System telemetry reconciled across all four kernel subsystems:\n\
              [OK] Windows Registry Hive HKLM\\Software\\Policies\\Vault\n\
              [OK] Linux Kernel Process /proc/808/environ\n\
              [OK] macOS APFS Extended Attribute com.apple.security.token\n\
              [OK] PowerShell Core Global Runspace $env:FLAG_FINAL\n\n\
            CONGRATULATIONS, OPERATIVE!\n\
            The master air-gap clearance flag has been decrypted in memory:\n\n\
            FLAG: {}\n\
            ======================================================================",
            flag
        ),
        "mario"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_derive_the_key_pipeline() {
        let mut state = EnclaveState::default();

        assert_eq!(state.count_unlocked(), 0);
        assert!(!state.is_fully_unlocked());
        assert!(state.try_decrypt_flag().is_none());

        // 1. Run Windows reg query (with messy capitalization)
        let r1 = check_vault_interaction(&mut state, "  REG QUERY HKLM\\Software\\Policies\\Vault  ");
        assert!(r1.is_some());
        assert_eq!(state.count_unlocked(), 1);
        assert!(!state.is_fully_unlocked());

        // 2. Run Linux cat
        let r2 = check_vault_interaction(&mut state, "cat /proc/808/environ");
        assert!(r2.is_some());
        assert_eq!(state.count_unlocked(), 2);

        // 3. Run macOS xattr
        let r3 = check_vault_interaction(&mut state, "xattr -p com.apple.security.token /Volumes/MacintoshHD/Quarantine.plist");
        assert!(r3.is_some());
        assert_eq!(state.count_unlocked(), 3);

        // 4. Run PowerShell $env query
        let r4 = check_vault_interaction(&mut state, "$env:FLAG_FINAL");
        assert!(r4.is_some());
        assert_eq!(state.count_unlocked(), 4);
        assert!(state.is_fully_unlocked());

        let flag = state.try_decrypt_flag().expect("Failed to decrypt flag");
        assert_eq!(flag.len(), 34);
        assert!(flag.starts_with("CTF{"));
        assert!(flag.ends_with('}'));

        // Victory screen should contain the decrypted flag and mario sound
        let resp = r4.unwrap();
        assert!(resp.output.contains(&flag));
        assert_eq!(resp.sound.as_deref(), Some("mario"));
    }
}
