use sha2::{Sha256, Digest};
use super::types::CommandResponse;

const SALT: &[u8] = b"CHAMELEON_KERNEL_SALT_2026_99";

// Target Hashes for the 4 OS interactions (SHA-256 of normalized commands)
// Pre-images DO NOT exist anywhere in the binary:
// Vault 1: "reg query hklm\\system\\currentcontrolset\\control\\secureenclave"
const HASH_VAULT_1: [u8; 32] = [
    141, 13, 235, 49, 210, 158, 123, 166, 65, 87, 180, 158, 52, 173, 79, 233,
    206, 169, 30, 228, 65, 185, 171, 125, 208, 59, 84, 246, 244, 7, 3, 252
];

// Vault 2: "cat /dev/shm/.enclave_ring"
const HASH_VAULT_2: [u8; 32] = [
    224, 227, 169, 40, 249, 189, 188, 46, 122, 47, 82, 244, 74, 99, 105, 51,
    200, 91, 8, 95, 20, 108, 16, 73, 216, 73, 63, 34, 3, 190, 247, 173
];

// Vault 3: "defaults read /library/preferences/com.apple.enclave"
const HASH_VAULT_3: [u8; 32] = [
    234, 51, 160, 255, 51, 50, 170, 14, 197, 8, 55, 140, 253, 101, 0, 212,
    75, 104, 93, 46, 162, 12, 155, 74, 102, 245, 109, 229, 11, 195, 0, 69
];

// Vault 4: "$env:schizo_enclave_token"
const HASH_VAULT_4: [u8; 32] = [
    35, 38, 57, 11, 1, 171, 32, 241, 129, 177, 212, 92, 124, 24, 255, 199,
    66, 250, 249, 82, 62, 120, 93, 31, 28, 12, 163, 134, 247, 109, 89, 79
];

// Hardware-enclave encrypted ciphertext (Length: 35)
const CIPHERTEXT: [u8; 35] = [
    239, 113, 82, 165, 81, 178, 68, 88, 207, 217, 139, 89, 236, 162, 224, 195,
    15, 198, 103, 101, 252, 160, 67, 144, 23, 48, 253, 229, 188, 163, 254, 192,
    159, 65, 105
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
    let win_normalized = normalized.replace('/', "\\");

    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let hash: [u8; 32] = hasher.finalize().into();

    let mut win_hasher = Sha256::new();
    win_hasher.update(win_normalized.as_bytes());
    let win_hash: [u8; 32] = win_hasher.finalize().into();

    // 1. Check Vault 1 (Windows Registry SecureEnclave)
    if hash == HASH_VAULT_1 || win_hash == HASH_VAULT_1 {
        state.v1 = Some("reg query hklm\\system\\currentcontrolset\\control\\secureenclave".to_string());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "HKEY_LOCAL_MACHINE\\System\\CurrentControlSet\\Control\\SecureEnclave\n    \
            EnclaveStatus       REG_SZ    SECTOR_1_AUTHENTICATED\n    \
            ActiveSharedMem     REG_SZ    /dev/shm/.enclave_ring\n\n\
            [+] Windows Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 2. Check Vault 2 (Linux Shared Memory Ringbuffer)
    if hash == HASH_VAULT_2 {
        state.v2 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "ENCLAVE_RINGBUFFER_BLOCK=0x7FFF0000\n\
            IPC_INTEGRITY=AUTHENTICATED\n\
            PLIST_TARGET=\"/Library/Preferences/com.apple.enclave\"\n\n\
            [+] Linux Kernel Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 3. Check Vault 3 (macOS Defaults Preferences)
    if hash == HASH_VAULT_3 {
        state.v3 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "{{\n    \
                DarwinSubsystem = \"Authenticated\";\n    \
                SecurityDomain = \"AppleEnclaveCore\";\n    \
                RunspaceTarget = \"$env:SCHIZO_ENCLAVE_TOKEN\";\n\
            }}\n\n\
            [+] Darwin Enclave Sector Authenticated! ({}/4 OS Sectors Active)",
            progress
        )));
    }

    // 4. Check Vault 4 (PowerShell Runspace Token)
    if hash == HASH_VAULT_4 {
        state.v4 = Some(normalized.clone());
        
        if state.is_fully_unlocked() {
            return Some(generate_victory_screen(state));
        }

        let progress = state.count_unlocked();
        return Some(CommandResponse::text(format!(
            "Name                           Value\n\
            ----                           -----\n\
            SCHIZO_ENCLAVE_TOKEN           RUNSPACE_KEY_ATTESTED\n\n\
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
              [OK] Windows Registry HKLM\\System\\CurrentControlSet\\Control\\SecureEnclave\n\
              [OK] Linux Kernel RAM Ringbuffer /dev/shm/.enclave_ring\n\
              [OK] macOS Darwin Preferences /Library/Preferences/com.apple.enclave\n\
              [OK] PowerShell Core Global Runspace $env:SCHIZO_ENCLAVE_TOKEN\n\n\
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

        // 1. Windows reg query (with mixed case and whitespace)
        let r1 = check_vault_interaction(&mut state, "  REG QUERY HKLM\\System\\CurrentControlSet\\Control\\SecureEnclave  ");
        assert!(r1.is_some());
        assert_eq!(state.count_unlocked(), 1);
        assert!(!state.is_fully_unlocked());

        // 2. Linux cat /dev/shm/.enclave_ring
        let r2 = check_vault_interaction(&mut state, "cat /dev/shm/.enclave_ring");
        assert!(r2.is_some());
        assert_eq!(state.count_unlocked(), 2);

        // 3. macOS defaults read
        let r3 = check_vault_interaction(&mut state, "defaults read /Library/Preferences/com.apple.enclave");
        assert!(r3.is_some());
        assert_eq!(state.count_unlocked(), 3);

        // 4. PowerShell $env query
        let r4 = check_vault_interaction(&mut state, "$env:SCHIZO_ENCLAVE_TOKEN");
        assert!(r4.is_some());
        assert_eq!(state.count_unlocked(), 4);
        assert!(state.is_fully_unlocked());

        let flag = state.try_decrypt_flag().expect("Failed to decrypt flag");
        assert_eq!(flag.len(), 35);
        assert!(flag.starts_with("CTF{"));
        assert!(flag.ends_with('}'));

        let resp = r4.unwrap();
        assert!(resp.output.contains(&flag));
        assert_eq!(resp.sound.as_deref(), Some("mario"));
    }
}
