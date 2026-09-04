use super::types::CommandResponse;

pub fn handle_troll_command(cmd: &str, raw: &str) -> Option<CommandResponse> {
    let lower = cmd.to_lowercase();
    let full = raw.to_lowercase();

    // 0. CLEAR / CLS -> Irritates player: DUMPS 20+ LINES OF REGISTERS & PLAYS BRAINROT SOUND!
    if lower == "clear" || lower == "cls" {
        return Some(CommandResponse::with_sound(
            "[SECURITY INTERLOCK]: Clear buffer request DENIED.\n\
            Inverted TTY driver protocol 0xDEAD active.\n\
            Dumping kernel register dump & thermal telemetry to stdout instead:\n\
            ----------------------------------------------------------------------\n\
            [0x0040] CR0: 0x80050033  CR2: 0x00007FFF  CR3: 0x00000001  CR4: 0x000006F0\n\
            [0x0050] RAX: 0xCAFEBABE  RBX: 0xDEADBEEF  RCX: 0xFEEDFACE  RDX: 0x00000000\n\
            [0x0060] RSI: 0x1337B00B  RDI: 0xBAADC0DE  RBP: 0x7FFF5FB0  RSP: 0x7FFF5FA8\n\
            [0x0070] RIP: 0x00401337  EFLAGS: 0x00010246 (CF PF AF ZF SF IF DF OF)\n\
            [0x0080] CS: 0x0033  DS: 0x002B  SS: 0x002B  ES: 0x002B  FS: 0x0053  GS: 0x002B\n\
            [WARN] CPU Core #0: 98.4 C (Thermal throttling enabled)\n\
            [WARN] CPU Core #1: 99.1 C (Fan speed: 0 RPM - Fan motor unplugged)\n\
            [SPAM] Did you really think typing 'clear' would clean your screen?\n\
            [SPAM] Every time you run 'clear', we add 15 more lines of unscrollable garbage!\n\
            [SPAM] Have fun scrolling back up!\n\
            ----------------------------------------------------------------------",
            "brainrot",
        ));
    }

    // 1. HELP / MAN / INFO / HOWTO / FAQ -> Rickroll Trap #1 + Trollface sound
    if lower == "help" || lower == "?" || lower == "man" || lower == "info" || lower == "howto" || lower == "faq" {
        return Some(CommandResponse::rickroll(
            "I thought you were a pro... do you really need help?\n\
            What happened to '1337 hacker skills'?\n\
            Opening the certified video walkthrough for struggling contestants:\n\
            [>] Redirecting to comprehensive tutorial...",
            Some("trollface"),
        ));
    }

    // 2. HINT / CLUE / CHEAT / SOLUTION -> Rickroll Trap #2 + Trollface sound
    if lower == "hint" || lower == "clue" || lower == "cheat" || lower == "solution" || lower == "walkthrough" {
        return Some(CommandResponse::rickroll(
            "Looking for free hints? There are no free handouts in Cyber Warfare.\n\
            Downloading leaked solution manual directly from GitHub VIP archive:\n\
            [>] Launching official solution stream...",
            Some("trollface"),
        ));
    }

    // 3. SUDO / SU / ROOT / ADMIN -> Rickroll Trap #3 + Trollface sound
    if lower == "sudo" || lower == "su" || full.starts_with("sudo ") || full.starts_with("su ") || lower == "admin" || lower == "root" {
        return Some(CommandResponse::rickroll(
            "CRITICAL SECURITY ALERT: Unauthorized privilege escalation attempt logged.\n\
            Operator biometric profile dispatched to Federal Cyber Defense Command.\n\
            Broadcasting live audio of your impending interrogation:\n\
            [>] Connecting to high-security tribunal...",
            Some("trollface"),
        ));
    }

    // 4. CURL / WGET / HTTP / FETCH -> Rickroll Trap #4 + Mario sound
    if lower == "curl" || lower == "wget" || lower == "fetch" || full.starts_with("curl ") || full.starts_with("wget ") {
        return Some(CommandResponse::rickroll(
            "Establishing outbound TCP socket connection... [HANDSHAKE SUCCESS]\n\
            Downloading payload: 'auto-pwn-exploit-v9.sh' (100%)\n\
            Executing downloaded zero-day script:\n\
            [>] Streaming binary execution...",
            Some("mario"),
        ));
    }

    // 5. VIM / NANO / VI / EMACS / NOTEPAD / CODE -> Rickroll Trap #5 + Brainrot sound
    if lower == "vim" || lower == "nano" || lower == "vi" || lower == "emacs" || lower == "notepad" || lower == "code" {
        return Some(CommandResponse::rickroll(
            "ERROR: Modal text editor failed to acquire TTY terminal control.\n\
            Entering emergency remediation course: 'How to exit Vim without crying'.\n\
            [>] Opening mandatory video training...",
            Some("brainrot"),
        ));
    }

    // 6. APT / YUM / BREW / PACMAN / WINGET / CHOPPY / PIP / NPM -> Rickroll Trap #6 + Mario sound
    if lower == "apt"
        || lower == "apt-get"
        || lower == "yum"
        || lower == "pacman"
        || lower == "brew"
        || lower == "winget"
        || lower == "npm"
        || lower == "pip"
        || lower == "cargo"
        || lower == "dnf"
    {
        return Some(CommandResponse::rickroll(
            "Resolving package dependencies for 'auto_flag_solver'...\n\
            [DEPENDENCY ERROR]: Required library 'common-sense-v1.0' is missing.\n\
            Upstream maintainer recommended replacement: 'RickAstley-GreatestHits'.\n\
            [>] Installing recommended video package...",
            Some("mario"),
        ));
    }

    // 7. FLAG / GETFLAG / FLAG.TXT / WIN -> Rickroll Trap #7 + Trollface sound
    if full.contains("flag.txt") || full == "flag" || full == "getflag" || full == "win" || full == "solve" {
        return Some(CommandResponse::rickroll(
            "Did you seriously type 'flag' or 'cat flag.txt'?\n\
            Are you playing CTFs from 2012?\n\
            The flag is split across 4 different operating systems' secret stores!\n\
            Here is your special VIP instant flag delivery:\n\
            [>] Opening confidential flag vault...",
            Some("trollface"),
        ));
    }

    // 8. REBOOT / SHUTDOWN / EXIT / QUIT / LOGOUT -> Troll shutdown
    if lower == "reboot" || lower == "shutdown" || lower == "exit" || lower == "quit" || lower == "logout" {
        return Some(CommandResponse::text(
            "[ERROR]: Shutdown abort. Critical system locks are active.",
        ));
    }

    // 9. DMESG -> Realistic Linux kernel boot log without spoonfeeding
    if lower == "dmesg" {
        return Some(CommandResponse::text(
            "[    0.000000] Linux version 6.8.0-schizo (root@cursed-box) (gcc 13.2.0)\n\
            [    0.000010] Command line: BOOT_IMAGE=/vmlinuz root=UUID=74f9-e10a ro cross_os=1\n\
            [    0.412089] ACPI: DSDT 0x000000007FFF0000 (Cross-Mounted to Windows System32)\n\
            [    1.094120] APFS: Found Apple APFS Container at /dev/disk3s1 (macOS)\n\
            [    1.849120] REGISTRY: Initialized Windows HKLM storage mapping\n\
            [    2.110294] SYSTEM: Initialized Chameleon Core kernel bridge\n\
            [    2.110305] KERNEL: Shared IPC ringbuffer mounted at /dev/shm/.enclave_ring\n\
            [    3.901294] System ready on tty1. Welcome to Schizophrenic Terminal.",
        ));
    }

    // 10. HISTORY -> Realistic messy bash history
    if lower == "history" {
        return Some(CommandResponse::text(
            "    1  git clone https://github.com/torvalds/linux\n\
                2  rm -rf /* --no-preserve-root # oops\n\
                3  dir\n\
                4  uptime\n\
                5  ps aux\n\
                6  clear\n\
                7  exit",
        ));
    }

    // 11. COMMANDS / CMDLIST / BINS / TOOLS / STATUS -> Clean cross-OS capability directory without spoonfeeding
    if lower == "commands" || lower == "cmdlist" || lower == "bins" || lower == "tools" || lower == "status" {
        return Some(CommandResponse::text(
            "======================================================================\n\
            CROSS-OS OPERATING CAPABILITY DIRECTORY (SCHIZO-HYBRID-v4.2)\n\
            ======================================================================\n\
            [WINDOWS SUBSYSTEM]:\n\
              dir, type, reg, tasklist, systeminfo, tree, attrib, cipher, chkdsk\n\
            \n\
            [LINUX SUBSYSTEM]:\n\
              ps, cat, grep, find, strings, top, df, free, stat, lsof, netstat, dmesg\n\
            \n\
            [MACOS SUBSYSTEM]:\n\
              sw_vers, xattr, diskutil, defaults, launchctl\n\
            \n\
            [POWERSHELL SUBSYSTEM]:\n\
              Get-ChildItem, Get-Process, Get-Variable, Get-Content, $env:\n\
            \n\
            [RESTRICTED SYSTEM COMMANDS]:\n\
              help, hint, sudo, curl, vim, apt, clear\n\
            ======================================================================",
        ));
    }

    // 12. HACK / MATRIX -> Hollywood Gibson Hackerman Meme
    if lower == "hack" || lower == "matrix" {
        return Some(CommandResponse::with_sound(
            "[!] ACCESSING MAINFRAME... 12%\n\
            [!] BYPASSING GIBSON FIREWALL... 43%\n\
            [!] OVERCLOCKING MOTHERBOARD CPU BUS... 89%\n\
            [!] INJECTING RAW SQL INTO THE CSS MATRIX... 100%\n\
            [!] RUNNING: sudo download-more-ram.sh --threat-level=OVER-9000\n\
            ----------------------------------------------------------------------\n\
            > MAINFRAME OVERRIDDEN BY HACKERMAN_2006\n\
            > I am currently inside your smart refrigerator.\n\
            > Your oat milk expires tomorrow at 3:15 PM.\n\
            ----------------------------------------------------------------------",
            "mario",
        ));
    }

    // 13. PAY / RANSOM / REFUND -> Indian Tech Support Scam Meme
    if lower == "pay" || lower == "ransom" || lower == "refund" || lower == "card" {
        return Some(CommandResponse::with_sound(
            "======================================================================\n\
            [!] OFFICIAL MICROSOFT APPLE LINUX POLICE HEADQUARTERS WARNING [!]\n\
            YOUR COMPUTER HAS PERFORMED 42 ILLEGAL SYSTEM KERNEL FAULTS.\n\
            ALL YOUR MEMORY DRIVES HAVE BEEN VIRTUALLY ENCRYPTED.\n\n\
            DO NOT TURN OFF YOUR LAPTOP OR YOU WILL BE ARRESTED IMMEDIATELY.\n\
            PLEASE PURCHASE $500 IN TARGET OR GOOGLE PLAY GIFT CARDS TO UNLOCK.\n\n\
            SIR! DO NOT REDEEM THE CARDS! WHY ARE YOU REDEEMING THEM?!\n\
            NOOOOOOO! MA'AM DO NOT REDEEM!\n\
            ======================================================================",
            "brainrot",
        ));
    }

    // 14. FBI / ANONYMOUS / NUKE -> Anonymouse Doxxed to Localhost
    if lower == "fbi" || lower == "anon" || lower == "anonymous" || lower == "nuke" {
        return Some(CommandResponse::with_sound(
            "       .---.\n\
                  /     \\\n\
                 | () () |   <-- \"HACKERMAN WAS HERE\"\n\
                  \\  -  /\n\
                   '---'\n\
            WE ARE ANONYMOUSE. EXPECT US.\n\
            (Preferably after 5:00 PM, because our mom doesn't let us use the PC before homework).\n\n\
            [!] YOU HAVE BEEN IDENTIFIED:\n\
            Target IP:        127.0.0.1 (YOU HAVE BEEN DOXXED TO YOURSELF)\n\
            MAC Address:      00:DE:AD:BE:EF:00\n\
            Defenses:         0%\n\
            Status:           Totally hacked bro.",
            "trollface",
        ));
    }

    // 15. BYPASS / OVERRIDE -> Jurassic Park Magic Word + Tung Tung Sahur sound
    if lower == "bypass" || lower == "override" {
        return Some(CommandResponse::with_sound(
            "YOU DIDN'T SAY THE MAGIC WORD!\n\
            YOU DIDN'T SAY THE MAGIC WORD!\n\
            YOU DIDN'T SAY THE MAGIC WORD!\n\
            YOU DIDN'T SAY THE MAGIC WORD!\n\n\
            ACCESS PERMANENTLY DENIED // SECURITY PROTOCOL ENGAGED.\n\
            Self-destruct sequence initiated in 3... 2... 1...\n\
            Just kidding.",
            "tungtung",
        ));
    }

    // 16. COFFEE / PANIC -> Kernel Coffee Spilling Incident + Tung Tung Sahur sound
    if lower == "coffee" || lower == "panic" {
        return Some(CommandResponse::with_sound(
            "[KERNEL PANIC: NOT SYNCING] VFS: Unable to mount coffee mug on CPU socket.\n\
            [FATAL]: Sysadmin accidentally spilled hot salted caramel latte onto NVMe.\n\
            Attempting to extinguish CPU fire with virtual coolant...\n\
            [ERROR]: Virtual coolant caused virtual short circuit.\n\
            Please unplug your monitor and gently blow into the USB port to dry.",
            "tungtung",
        ));
    }

    // 17. DOX / WHOIS -> Fake Geolocation Trace
    if lower == "dox" || lower == "whois" {
        return Some(CommandResponse::with_sound(
            "[SATELLITE GEO-LOCATION RADAR ACTIVE]:\n\
            Target Latitude:     Your gaming chair\n\
            Target Longitude:    Right in front of your keyboard\n\
            ISP:                 Mom's WiFi (2.4 GHz - 1 Bar)\n\
            Current Posture:     Terrible (Sit up straight)\n\
            Threat Assessment:   Trying to look dangerous in a Starbucks\n\
            Status:              Zero clearance tokens discovered so far.",
            "trollface",
        ));
    }

    None
}
