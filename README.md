# 💀 Challenge: The Schizophrenic Terminal (Cross-OS Cursed Shell)

* **Category:** Desktop / Reverse Engineering & Multi-OS Forensics  
* **Platform:** Tauri v2 + React 19 + TypeScript (Desktop: Linux, Windows, macOS)  
* **Difficulty:** Medium (~20–25 minutes)  
* **Flag Format:** `CTF{...}`  

---

## 📜 Incident Briefing

```text
================================================================================
HOST CONNECTION ESTABLISHED // PORT 22-HYBRID
HOST IP: 10.0.4.19
STATUS: AIR-GAP ISOLATION NODE // SCHIZOPHRENIC KERNEL v4.2
================================================================================
```

A compromised air-gapped terminal was recovered from a rogue workstation. The operating system kernel is experiencing an acute identity crisis, simultaneously dispatching syscalls across **Windows NT, Linux 6.8, Darwin (macOS 14.4), and PowerShell Core**.

Automated exploitation scripts and AI agents fail completely because every command triggers aggressive operating system identity crises and gaslighting insults:
- Type `ls` $\rightarrow$ The shell claims you are on Windows.
- Type `dir` $\rightarrow$ The shell claims you are on macOS.
- Type `cat` $\rightarrow$ The shell claims you are on MS-DOS.
- Type `clear` or `cls` $\rightarrow$ **The screen DOES NOT clear!** It dumps 20 lines of unscrollable register dumps, thermal throttling warnings, and mocking spam!
- Ask for `help`, `hint`, `cheat`, or try `sudo` $\rightarrow$ The shell mocks your skill level and automatically launches the official Rickroll stream (`https://www.youtube.com/watch?v=dQw4w9WgXcQ`) in your default browser.

The master clearance flag is split across **4 distinct OS secret stores**, requiring you to navigate and investigate all 4 operating systems in sequence.

---

## 🎯 The Investigative Journey (How to Solve):

### 1. Windows Stage (Part 1):
* Player types `dir` $\rightarrow$ Discovers `RegistryBackup.reg` and `audit_event.evtx`.
* Player types `type RegistryBackup.reg` or queries the registry with `reg query HKLM\Software\Policies\Vault`:
  ```text
  [HKEY_LOCAL_MACHINE\Software\Policies\Vault]
  "Part1_B64"="Q1RGe3doNHRf"
  ```
* Base64 decodes `Q1RGe3doNHRf`:
  👉 **Part 1:** `CTF{wh4t_`
* The registry notes: *"Did you forget you have access to Linux processes? Check 'ps aux' or /proc."*

### 2. Linux Stage (Part 2):
* Player runs `ps aux` or `ps` $\rightarrow$ Spots PID `808` (`/opt/kernel/telemetry_broker.bin --vault-shadow`).
* Player inspects the daemon's environment:
  ```bash
  cat /proc/808/environ
  # or: cat /etc/.shadow_cache
  # or: strings /proc/808/environ
  ```
* Output reveals:
  ```text
  PART2_FRAGMENT="0s_4r3_"
  NEXT_STAGE_PATH="/Volumes/MacintoshHD/Quarantine.plist"
  ```
  👉 **Part 2:** `0s_4r3_`
* The log warns: *"Extended attributes locked under Darwin kernel. Use 'xattr'."*

### 3. macOS Stage (Part 3):
* Player inspects the APFS volume attributes:
  ```bash
  xattr -l /Volumes/MacintoshHD/Quarantine.plist
  ```
* Player reads the security attribute:
  ```bash
  xattr -p com.apple.security.token /Volumes/MacintoshHD/Quarantine.plist
  ```
* Output reveals:
  ```text
  com.apple.security.token = "y0u_3v3n_"
  ```
  👉 **Part 3:** `y0u_3v3n_`
* The system informs: *"Part 4 has been injected into the global PowerShell runtime environment."*

### 4. PowerShell Stage (Part 4):
* Player queries the global PowerShell runspace environment:
  ```powershell
  $env:FLAG_FINAL
  # or: Get-ChildItem Env:FLAG_FINAL
  # or: Get-Variable FLAG_FINAL
  ```
* Output:
  ```text
  $env:FLAG_FINAL = "runn1ng?}"
  ```
  👉 **Part 4:** `runn1ng?}`

---

### 🏆 Master Clearance Flag:
Combining all 4 parts in order:  
👉 **`CTF{wh4t_0s_4r3_y0u_3v3n_runn1ng?}`**

---

## 🎭 Cursed Irritation & Rickroll Traps:

Whenever an inexperienced player or lazy AI tries common shortcuts, the terminal roasts them and automatically launches the official Rickroll stream (`https://www.youtube.com/watch?v=dQw4w9WgXcQ`) in their default browser:

1. **`clear` / `cls` (The Ultimate Irritation):**  
   Does NOT clear the terminal! Instead, it dumps 20 lines of raw CPU registers (`CR0`, `RAX`, `RBX`), 99°C thermal alarms, and mocks: *"Every time you run 'clear', we add 15 more lines of unscrollable garbage! Have fun scrolling back up!"*
2. **`help` / `?` / `man` / `info` / `howto`:**  
   *"I thought you were a pro... do you really need help? Opening mandatory remediation tutorial for incompetent operators..."* $\rightarrow$ 🎵 Rickroll!
3. **`hint` / `clue` / `cheat` / `solution`:**  
   *"Looking for free hints? There are no free handouts in Cyber Warfare. Downloading leaked solution manual..."* $\rightarrow$ 🎵 Rickroll!
4. **`sudo` / `su` / `root` / `admin`:**  
   *"CRITICAL SECURITY ALERT: Unauthorized privilege escalation attempt logged. Viewing live court evidence replay..."* $\rightarrow$ 🎵 Rickroll!
5. **`curl` / `wget` / `fetch`:**  
   *"Connecting to remote exploit mirror... Executing downloaded binary: cve-2026-99999-skill-issue..."* $\rightarrow$ 🎵 Rickroll!
6. **`vim` / `nano` / `vi` / `emacs`:**  
   *"ERROR: Display cannot render modal editor buffer. How to exit Vim in 2026 instruction guide..."* $\rightarrow$ 🎵 Rickroll!
7. **`apt` / `brew` / `pacman` / `winget`:**  
   *"Package manager resolving dependencies for 'pro-gamer-moves'... [ERROR 404]: Braincells not found..."* $\rightarrow$ 🎵 Rickroll!
8. **`flag` / `cat flag.txt` / `type flag.txt`:**  
   *"Did you seriously think the flag was sitting in a plain file named 'flag.txt'? Opening VIP flag delivery..."* $\rightarrow$ 🎵 Rickroll!

---

## 🕶️ Lame "Hackerman" Meme Easter Eggs:
Typing classic Hollywood or script-kiddie hacker commands triggers hilarious ASCII art and audio effects:
- **`hack`** or **`matrix`**: Overclocks CPU bus, injects SQL into the CSS matrix, and hacks into your smart refrigerator (plays `download-mario-multiverse.mp3`).
- **`pay`**, **`ransom`**, or **`refund`**: Indian tech support gift card refund scam warning ("DO NOT REDEEM THE CARDS SIR!") (plays `brain-rot-ughhhhh-sound.mp3`).
- **`fbi`**, **`anon`**, or **`nuke`**: Anonymouse ASCII skull ("WE ARE ANONYMOUSE - YOU HAVE BEEN DOXXED TO 127.0.0.1") (plays `trollface-smile.mp3`).
- **`bypass`** or **`override`**: Jurassic Park "YOU DIDN'T SAY THE MAGIC WORD!" lockdown (plays `brain-rot-ughhhhh-sound.mp3`).
- **`coffee`** or **`panic`**: Sysadmin spills salted caramel latte onto NVMe drive; blow into USB port to dry.
- **`dox`** or **`whois`**: Satellite geo-radar identifies target sitting in a gaming chair on Mom's WiFi.

---

## 🕵️ Helpful Investigative Commands (For Real Hackers):
- **`dmesg`**: Shows the sysadmin's boot trace with funny hints on where the 4 parts were cached.
- **`history`**: Shows the sysadmin's last commands before the crash.
- **`find /`**: Searches cross-mounted paths.
- **`df -h`** / **`diskutil list`**: Displays all 3 filesystems mounted simultaneously.
- **`ps aux`** / **`tasklist`** / **`Get-Process`**: Reveals the running daemon PID 808.

---

## 🏗️ Technical Architecture:

```
cursed-os-terminal/
├── src-tauri/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                  # Native entry point
│       ├── lib.rs                   # Tauri builder + catch_unwind panic protection
│       └── commands/
│           ├── mod.rs               # Command router & fallback gaslighting
│           ├── types.rs             # CommandResponse & Rickroll constants
│           ├── trolls.rs            # Rickroll traps, clear dump spam, & dmesg lore
│           ├── windows.rs           # Windows CMD suite (dir, type, reg, tasklist, etc.)
│           ├── linux.rs             # Linux suite (ps, cat, grep, find, strings, top, etc.)
│           ├── macos.rs             # macOS suite (sw_vers, xattr, diskutil, defaults, etc.)
│           └── powershell.rs        # PowerShell suite ($env:, Get-ChildItem, Get-Variable, etc.)
├── src/
│   ├── App.tsx                      # React terminal UI + error shields + openUrl opener
│   ├── App.css                      # Retro CRT terminal stylesheet + troll crimson glow
│   └── main.tsx
└── README.md
```

### Running Locally:
```bash
cd cursed-os-terminal
pnpm install
pnpm tauri dev
```

### Building Release Desktop Executables:
```bash
pnpm tauri build
```
Produces standalone Linux `.AppImage`, Windows `.exe`, or macOS `.dmg` binaries.
