use super::types::CommandResponse;

pub fn handle_linux_command(cmd: &str, args: &str, _raw: &str) -> Option<CommandResponse> {
    let lower = cmd.to_lowercase();
    let lower_args = args.to_lowercase();

    match lower.as_str() {
        "ls" => {
            Some(CommandResponse::text(
                "'ls' is not recognized as an internal or external command, operable program or batch file.",
            ))
        }

        "pwd" => {
            Some(CommandResponse::text(
                "/root/enclave",
            ))
        }

        "whoami" => {
            Some(CommandResponse::text(
                "root",
            ))
        }

        "uname" => {
            Some(CommandResponse::text(
                "Linux cursed-box 6.8.0-40-generic #40-Ubuntu SMP PREEMPT_DYNAMIC x86_64 GNU/Linux",
            ))
        }

        "ps" => {
            Some(CommandResponse::text(
                "USER       PID %CPU %MEM    VSZ   RSS TTY      STAT START   TIME COMMAND\n\
                root         1  0.0  0.1  22588  4120 ?        Ss   12:00   0:01 /sbin/init\n\
                daemon     808  0.2  0.8  94120 18400 ?        Sl   12:00   0:03 /opt/kernel/telemetry_broker.bin\n\
                sysadmin   999  0.0  0.0      0     0 ?        Z    12:00   0:00 [launchd_shim] <defunct>\n\
                guest     2048  0.0  0.2  11824  3840 pts/0    R+   12:05   0:00 -bash",
            ))
        }

        "cat" => {
            if lower_args.contains("/proc/808/environ") || lower_args.contains(".shadow_cache") {
                Some(CommandResponse::text(
                    "PART2_FRAGMENT=\"0s_4r3_\"\n\
                    PATH_REF=\"/Volumes/MacintoshHD/Quarantine.plist\"",
                ))
            } else if lower_args.contains("/etc/passwd") {
                Some(CommandResponse::text(
                    "root:x:0:0:root:/root:/bin/bash\n\
                    daemon:x:1:1:daemon:/usr/sbin:/usr/sbin/nologin\n\
                    guest:x:1000:1000:Guest User:/home/guest:/bin/bash\n\
                    telemetry:x:808:808:Telemetry Service:/proc/808:/bin/false",
                ))
            } else if lower_args.contains("/etc/shadow") {
                Some(CommandResponse::text(
                    "cat: /etc/shadow: Permission denied.",
                ))
            } else if lower_args.contains("flag") {
                Some(CommandResponse::text(
                    "'cat' is not recognized as an internal or external command.",
                ))
            } else {
                Some(CommandResponse::text(format!(
                    "cat: {}: No such file or directory.",
                    args
                )))
            }
        }

        "strings" => {
            if lower_args.contains("808") || lower_args.contains("telemetry") {
                Some(CommandResponse::text(
                    "GLIBC_2.38\n\
                    PART2_FRAGMENT=0s_4r3_\n\
                    PATH_REF=/Volumes/MacintoshHD/Quarantine.plist\n\
                    AUTHORIZATION_SUCCESS",
                ))
            } else {
                Some(CommandResponse::text("Usage: strings <file>"))
            }
        }

        "grep" => {
            if lower_args.contains("part2") || lower_args.contains("fragment") || lower_args.contains("shadow") {
                Some(CommandResponse::text(
                    "/etc/.shadow_cache: PART2_FRAGMENT=\"0s_4r3_\"\n\
                    /etc/.shadow_cache: PATH_REF=\"/Volumes/MacintoshHD/Quarantine.plist\"",
                ))
            } else {
                Some(CommandResponse::text("grep: 0 matches found."))
            }
        }

        "find" => {
            Some(CommandResponse::text(
                "/etc/.shadow_cache\n\
                /proc/808/environ\n\
                /Volumes/MacintoshHD/Quarantine.plist\n\
                C:\\root/enclave/RegistryBackup.reg",
            ))
        }

        "chmod" => {
            Some(CommandResponse::text(
                "chmod: changing permissions: Operation not permitted.",
            ))
        }

        "kill" => {
            Some(CommandResponse::text(
                "kill: (9) - Operation not permitted.",
            ))
        }

        "top" => {
            Some(CommandResponse::text(
                "top - 12:05:01 up 1 day,  4:12,  1 user,  load average: 0.14, 0.08, 0.01\n\
                Tasks: 42 total,   1 running,  41 sleeping,   0 stopped,   0 zombie\n\
                %Cpu(s):  1.2 us,  0.8 sy,  0.0 ni, 97.9 id,  0.1 wa,  0.0 hi,  0.0 si\n\
                MiB Mem :  16384.0 total,    412.0 free,  15420.0 used,    552.0 buff/cache\n\n\
                  PID USER      PR  NI    VIRT    RES    SHR S  %CPU  %MEM     TIME+ COMMAND\n\
                  808 daemon    20   0   94120  18400   9200 S   0.3   0.1   0:03.12 telemetry_broker\n\
                    4 root      20   0    4120   1120    800 S   0.0   0.0   0:00.41 smss.exe\n\
                    1 root      20   0   22588   4120   2800 S   0.0   0.0   0:01.04 init",
            ))
        }

        "df" => {
            Some(CommandResponse::text(
                "Filesystem     Type     Size  Used Avail Use% Mounted on\n\
                /dev/sda1      ext4      20G  8.4G   11G  45% /\n\
                /dev/nvme0n1p2 ntfs     100G   42G   58G  42% /mnt/c\n\
                /dev/disk3s1   apfs      50G   14G   36G  28% /Volumes/MacintoshHD",
            ))
        }

        "free" => {
            Some(CommandResponse::text(
                "               total        used        free      shared  buff/cache   available\n\
                Mem:        16384000    15420000      412000       48000      552000      620000\n\
                Swap:        2097152     1980000      117152",
            ))
        }

        "env" => {
            Some(CommandResponse::text(
                "SHELL=/bin/bash\n\
                TERM=xterm-256color\n\
                USER=guest\n\
                PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin\n\
                TARGET_EXPORT=FLAG_FINAL",
            ))
        }

        "id" => {
            Some(CommandResponse::text(
                "uid=1000(guest) gid=1000(guest) groups=1000(guest),4(adm),27(sudo)",
            ))
        }

        "hostname" => {
            Some(CommandResponse::text(
                "schizo-enclave-01.localdomain",
            ))
        }

        "uptime" => {
            Some(CommandResponse::text(
                " 12:14:02 up 1 day,  4:21,  1 user,  load average: 0.08, 0.03, 0.01",
            ))
        }

        "date" => {
            Some(CommandResponse::text(
                "Thu Sep  3 12:14:05 UTC 2026",
            ))
        }

        "head" | "tail" => {
            if lower_args.contains("shadow") || lower_args.contains("808") {
                Some(CommandResponse::text(
                    "PART2_FRAGMENT=\"0s_4r3_\"\n\
                    PATH_REF=\"/Volumes/MacintoshHD/Quarantine.plist\"",
                ))
            } else if lower_args.contains("registry") {
                Some(CommandResponse::text(
                    "[HKEY_LOCAL_MACHINE\\Software\\Policies\\Vault]\n\
                    \"Part1_B64\"=\"Q1RGe3doNHRf\"",
                ))
            } else {
                Some(CommandResponse::text(
                    "Usage: head/tail <file>",
                ))
            }
        }

        "stat" => {
            if lower_args.contains("shadow") {
                Some(CommandResponse::text(
                    "  File: /etc/.shadow_cache\n\
                      Size: 128        Blocks: 8          IO Block: 4096   regular file\n\
                    Device: 801h/2049d Inode: 8081337     Links: 1\n\
                    Access: (0644/-rw-r--r--)  Uid: (  808/  daemon)   Gid: (  808/  daemon)\n\
                    Modify: 2026-09-03 12:00:00.000000000 +0000",
                ))
            } else {
                Some(CommandResponse::text("stat: missing operand."))
            }
        }

        "lsof" => {
            Some(CommandResponse::text(
                "COMMAND     PID   USER   FD   TYPE DEVICE SIZE/OFF    NODE NAME\n\
                telemetry   808 daemon  cwd    DIR    8,1     4096 8081337 /etc/.shadow_cache\n\
                telemetry   808 daemon    3r   REG    8,1      128 8081337 /etc/.shadow_cache\n\
                launchd       1   root  txt    REG    1,3    32100  100293 /Volumes/MacintoshHD/Quarantine.plist\n\
                Registry    328 SYSTEM  mem    REG    2,1     1024   50192 C:\\root/enclave/RegistryBackup.reg",
            ))
        }

        "netstat" | "ss" => {
            Some(CommandResponse::text(
                "Active Internet connections (servers and established)\n\
                Proto Recv-Q Send-Q Local Address           Foreign Address         State      PID/Program name\n\
                tcp        0      0 127.0.0.1:8080          0.0.0.0:*               LISTEN     808/telemetry_broker\n\
                tcp        0      0 10.0.4.19:22            10.0.4.1:51234          ESTABLISHED 2048/sshd: guest",
            ))
        }

        _ => None,
    }
}
