use super::types::CommandResponse;

pub fn handle_powershell_command(cmd: &str, args: &str, raw: &str) -> Option<CommandResponse> {
    let lower = cmd.to_lowercase();
    let lower_args = args.to_lowercase();
    let raw_lower = raw.to_lowercase();

    // Direct environment query: $env:FLAG_FINAL, $env:flag, etc.
    if raw_lower.starts_with("$env:") {
        if raw_lower.contains("flag_final") || raw_lower.contains("final") {
            return Some(CommandResponse::with_sound(
                "runn1ng?}",
                "mario",
            ));
        } else {
            return Some(CommandResponse::text(
                "Name                           Value\n\
                ----                           -----\n\
                OS                             Windows_NT\n\
                PATH                           C:\\Windows\\System32;C:\\Windows\n\
                FLAG_FINAL                     runn1ng?}",
            ));
        }
    }

    match lower.as_str() {
        "get-childitem" | "gci" => {
            if lower_args.contains("env:") || lower_args.contains("env") {
                Some(CommandResponse::text(
                    "Name                           Value\n\
                    ----                           -----\n\
                    COMPUTERNAME                   SCHIZO-01\n\
                    FLAG_FINAL                     runn1ng?}\n\
                    HOMEDRIVE                      C:\n\
                    PROCESSOR_ARCHITECTURE         AMD64\n\
                    PSExecutionPolicyPreference    Unrestricted",
                ))
            } else {
                Some(CommandResponse::text(
                    "Directory: C:\\root\\enclave\n\n\
                    Mode                 LastWriteTime         Length Name\n\
                    ----                 -------------         ------ ----\n\
                    d----            9/3/2026 12:00 PM                System32\n\
                    -a---            9/3/2026 12:00 PM           1024 RegistryBackup.reg\n\
                    -a---            9/3/2026 12:00 PM           4096 audit_event.evtx",
                ))
            }
        }

        "get-variable" | "gv" => {
            if lower_args.contains("flag") || lower_args.contains("final") {
                Some(CommandResponse::text(
                    "Name                           Value\n\
                    ----                           -----\n\
                    FLAG_FINAL                     runn1ng?}",
                ))
            } else {
                Some(CommandResponse::text(
                    "Name                           Value\n\
                    ----                           -----\n\
                    FLAG_FINAL                     runn1ng?}\n\
                    ErrorActionPreference          Continue\n\
                    PSVersionTable                 {PSVersion, PSEdition...}",
                ))
            }
        }

        "get-content" | "gc" => {
            if lower_args.contains("registry") {
                Some(CommandResponse::text(
                    "[HKEY_LOCAL_MACHINE\\Software\\Policies\\Vault]\n\
                    \"Part1_B64\"=\"Q1RGe3doNHRf\"",
                ))
            } else {
                Some(CommandResponse::text(
                    "Get-Content: Cannot find path across hybrid virtual disks.",
                ))
            }
        }

        "get-process" | "gps" => {
            Some(CommandResponse::text(
                " NPM(K)    PM(M)      WS(M)     CPU(s)      Id  SI ProcessName\n\
                -------    -----      -----     ------      --  -- -----------\n\
                     12    18.42      34.12       0.12     808   1 telemetry_broker\n\
                     24    42.10      89.54       1.40     999   1 launchd\n\
                      8     4.12       8.90       0.04     328   0 smss\n\
                     30    64.12     120.40       3.12    2048   1 pwsh",
            ))
        }

        "get-service" | "gsv" => {
            Some(CommandResponse::text(
                "Status   Name               DisplayName\n\
                ------   ----               -----------\n\
                Running  VaultBroker        Schizophrenic Memory Broker\n\
                Running  WinDefend          Microsoft Defender Antivirus\n\
                Running  launchd_shim       Apple Launchd APFS Bridge",
            ))
        }

        "invoke-webrequest" | "iwr" | "curl.exe" => {
            Some(CommandResponse::text(
                "Invoke-WebRequest: Network air-gap enforced. Local loopback only.",
            ))
        }

        "get-location" | "gl" => {
            Some(CommandResponse::text(
                "Path\n----\nC:\\root\\enclave",
            ))
        }

        "set-executionpolicy" => {
            Some(CommandResponse::text(
                "Execution Policy is locked to 'Bypass'.",
            ))
        }

        "select-string" | "sls" => {
            Some(CommandResponse::text(
                "Select-String: 0 matches found.",
            ))
        }

        _ => None,
    }
}
