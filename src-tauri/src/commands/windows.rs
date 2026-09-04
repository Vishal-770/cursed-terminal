use super::types::CommandResponse;

pub fn handle_windows_command(cmd: &str, args: &str, raw: &str) -> Option<CommandResponse> {
    let lower = cmd.to_lowercase();
    let lower_args = args.to_lowercase();

    match lower.as_str() {
        "dir" => {
            Some(CommandResponse::text(
                " Volume in drive C has no label.\n \
                Volume Serial Number is 74F9-E10A\n\n \
                Directory of C:\\root/enclave\n\n\
                09/03/2026  12:00 PM    <DIR>          .\n\
                09/03/2026  12:00 PM    <DIR>          ..\n\
                09/03/2026  12:00 PM    <DIR>          System32\n\
                09/03/2026  12:00 PM             1,024 RegistryBackup.reg\n\
                09/03/2026  12:00 PM             4,096 audit_event.evtx\n\
                09/03/2026  12:00 PM               512 .shadow_link.lnk\n               \
                3 File(s)          5,632 bytes\n               \
                3 Dir(s)   412,892,160 bytes free",
            ))
        }

        "type" => {
            if lower_args.contains("registrybackup.reg") {
                Some(CommandResponse::text(
                    "Windows Registry Editor Version 5.00\n\n\
                    [HKEY_LOCAL_MACHINE\\Software\\Policies\\Vault]\n\
                    \"EnclaveStatus\"=\"SECTOR_UNINITIALIZED\"\n\
                    \"AuditNote\"=\"Query active hive with: reg query HKLM\\Software\\Policies\\Vault\"\n\
                    \"DaemonRef\"=\"/opt/kernel/telemetry_broker.bin\"",
                ))
            } else if lower_args.contains("audit_event.evtx") {
                Some(CommandResponse::text(
                    "[SECURITY AUDIT LOG: EVENT ID 4657]\n\
                    EventID: 4657 (Registry Value Modified)\n\
                    Account: NT AUTHORITY\\SYSTEM\n\
                    KeyName: \\REGISTRY\\MACHINE\\Software\\Policies\\Vault\n\
                    Process: C:\\Windows\\System32\\reg.exe\n\
                    Image:   /opt/kernel/telemetry_broker.bin",
                ))
            } else if lower_args.contains(".shadow_link.lnk") {
                Some(CommandResponse::text(
                    "[LNK HEADER]: Target -> /Volumes/MacintoshHD/Quarantine.plist",
                ))
            } else {
                Some(CommandResponse::text(format!(
                    "The system cannot find the file specified: '{}'.",
                    args
                )))
            }
        }

        "reg" => {
            if lower_args.contains("query") && !lower_args.contains("vault") {
                Some(CommandResponse::text(
                    "HKEY_LOCAL_MACHINE\\Software\\Policies\n    \
                    HKEY_LOCAL_MACHINE\\Software\\Policies\\Microsoft\n    \
                    HKEY_LOCAL_MACHINE\\Software\\Policies\\Vault",
                ))
            } else {
                Some(CommandResponse::text(
                    "ERROR: Invalid registry command syntax. Try 'reg query HKLM\\Software\\Policies'.",
                ))
            }
        }

        "tasklist" => {
            Some(CommandResponse::text(
                "Image Name                     PID Session Name        Session#    Mem Usage\n\
                ========================= ======== ================ =========== ============\n\
                System                           4 Services                   0        24 K\n\
                smss.exe                       328 Services                   0     1,120 K\n\
                csrss.exe                      440 Services                   0     4,892 K\n\
                telemetry_broker.bin           808 Console                    1    18,400 K\n\
                launchd                          1 Console                    1    32,100 K",
            ))
        }

        "ipconfig" => {
            Some(CommandResponse::text(
                "Windows IP Configuration\n\n\
                Ethernet adapter vEthernet (WSL):\n   \
                Connection-specific DNS Suffix  . : enclave.local\n   \
                Link-local IPv6 Address . . . . . : fe80::99a1:44bc%12\n   \
                IPv4 Address. . . . . . . . . . . : 10.0.4.19\n   \
                Subnet Mask . . . . . . . . . . . : 255.255.255.0\n   \
                Default Gateway . . . . . . . . . : 10.0.4.1",
            ))
        }

        "systeminfo" => {
            Some(CommandResponse::text(
                "Host Name:                 SCHIZO-ENCLAVE-01\n\
                OS Name:                   Microsoft Windows Server 2025 / Unified Hybrid Subsystem\n\
                OS Version:                10.0.26100 N/A Build 26100\n\
                OS Manufacturer:           Schizophrenic Hybrid Systems Inc.\n\
                Kernel Type:               Multiprocessor Free\n\
                System Type:               x64-based PC",
            ))
        }

        "tree" => {
            Some(CommandResponse::text(
                "Folder PATH listing for volume OS_ENCLAVE\n\
                C:.\n\
                +---System32\n\
                |   +---drivers\n\
                |   \\---config\n\
                +---etc\n\
                |   \\---.shadow_cache\n\
                \\---Volumes\n\
                    \\---MacintoshHD\n\
                        \\---Quarantine.plist",
            ))
        }

        "attrib" => {
            Some(CommandResponse::text(
                "A   SHR       C:\\root/enclave\\RegistryBackup.reg\n\
                A             C:\\root/enclave\\audit_event.evtx\n\
                A   H         C:\\root/enclave\\.shadow_link.lnk",
            ))
        }

        "cipher" => {
            Some(CommandResponse::text(
                "Listing C:\\root/enclave\\\n\
                U RegistryBackup.reg\n\
                E audit_event.evtx\n\
                U .shadow_link.lnk",
            ))
        }

        "chkdsk" => {
            Some(CommandResponse::text(
                "The type of the file system is NTFS.\n\
                Volume label is OS_ENCLAVE.\n\
                0 bad file records processed.",
            ))
        }

        "ver" => {
            Some(CommandResponse::text(
                "Microsoft Windows [Version 10.0.26100.1742]\n\
                (c) Microsoft Corporation. All rights reserved.",
            ))
        }

        "ping" => {
            Some(CommandResponse::text(
                "Pinging 127.0.0.1 with 32 bytes of data:\n\
                Reply from 127.0.0.1: bytes=32 time<1ms TTL=128\n\
                Reply from 127.0.0.1: bytes=32 time<1ms TTL=128\n\
                Ping statistics for 127.0.0.1:\n    \
                Packets: Sent = 2, Received = 2, Lost = 0 (0% loss)",
            ))
        }

        "echo" if raw.contains("%cd%") || raw.contains("%CD%") => {
            Some(CommandResponse::text("C:\\root/enclave"))
        }

        _ => None,
    }
}
