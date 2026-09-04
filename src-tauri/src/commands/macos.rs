use super::types::CommandResponse;

pub fn handle_macos_command(cmd: &str, args: &str, _raw: &str) -> Option<CommandResponse> {
    let lower = cmd.to_lowercase();
    let lower_args = args.to_lowercase();

    match lower.as_str() {
        "sw_vers" => {
            Some(CommandResponse::text(
                "ProductName:            macOS\n\
                ProductVersion:         14.4.1\n\
                BuildVersion:           23E224\n\
                Darwin Kernel:          Darwin Kernel Version 23.4.0: root:xnu-10063.101.17~1/RELEASE_ARM64",
            ))
        }

        "xattr" => {
            if lower_args.contains("-l") {
                Some(CommandResponse::text(
                    "/Volumes/MacintoshHD/Quarantine.plist:\n\
                    com.apple.quarantine\n\
                    com.apple.security.token\n\
                    com.apple.provenance",
                ))
            } else {
                Some(CommandResponse::text(
                    "Usage: xattr [-l] [-p attr_name] <path>",
                ))
            }
        }

        "diskutil" => {
            Some(CommandResponse::text(
                "/dev/disk0 (internal, physical):\n   \
                #:                       TYPE NAME                    SIZE       IDENTIFIER\n   \
                0:      GUID_partition_scheme                        *500.3 GB   disk0\n   \
                1:             Microsoft Basic C:                     100.0 GB   disk0s1 (NTFS)\n   \
                2:                Linux Filesystem /                  150.0 GB   disk0s2 (ext4)\n   \
                3:                    Apple_APFS Container disk1      250.0 GB   disk0s3 (APFS)",
            ))
        }

        "defaults" => {
            Some(CommandResponse::text(
                "{\n    \
                AppleInterfaceStyle = Dark;\n    \
                AppleLanguages = (en);\n    \
                ActivePreferences = \"/Library/Preferences/com.apple.enclave\";\n    \
                RunspaceTarget = \"$env:SCHIZO_ENCLAVE_TOKEN\";\n\
                }",
            ))
        }

        "launchctl" => {
            Some(CommandResponse::text(
                "PID     Status  Label\n\
                1337    0       com.apple.enclave.chameleon\n\
                1       0       com.apple.launchd\n\
                -       0       com.microsoft.powershell.daemon",
            ))
        }

        "say" => {
            Some(CommandResponse::text(
                "[COREAUDIO]: Synth stream completed.",
            ))
        }

        "pbcopy" | "pbpaste" => {
            Some(CommandResponse::text(
                "pbpaste: pasteboard buffer empty.",
            ))
        }

        "open" => {
            Some(CommandResponse::text(
                "open: LSOpenURLsWithRole() failed with error -10810.",
            ))
        }

        "brew" => {
            Some(CommandResponse::text(
                "Homebrew 4.2.14\n\
                Error: Unknown command: brew",
            ))
        }

        _ => None,
    }
}
