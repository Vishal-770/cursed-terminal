import React, { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import trollfaceAudio from "./assets/trollface-smile.mp3";
import brainrotAudio from "./assets/brain-rot-ughhhhh-sound.mp3";
import marioAudio from "./assets/download-mario-multiverse.mp3";
import tungTungAudio from "./assets/tung-tung-sahur.mp3";
import "./App.css";

interface TerminalLine {
  id: number;
  type: "input" | "output" | "system" | "troll";
  text: string;
}

interface CommandResponse {
  output: string;
  open_url?: string | null;
  sound?: string | null;
}

export default function App() {
  const [history, setHistory] = useState<TerminalLine[]>([
    {
      id: 1,
      type: "system",
      text: "Linux cursed-box 6.8.0-schizo (tty1)\nlogin: root (automatic login)\nLast login: Thu Sep  3 12:00:01 2026 on tty1",
    },
    {
      id: 2,
      type: "system",
      text: "==================================================================",
    },
    {
      id: 3,
      type: "system",
      text: "SCHIZOPHRENIC OS KERNEL BRIDGE v4.2 [HYBRID MULTI-OS ARCHITECTURE]",
    },
    {
      id: 4,
      type: "system",
      text: "Active Kernels: Windows NT 10.0 // Darwin 23.4 (macOS) // Linux 6.8 // PowerShell",
    },
    {
      id: 5,
      type: "system",
      text: "Status: 4 OS fragments active. Clearance token quarantined across 4 vaults.",
    },
    {
      id: 6,
      type: "system",
      text: "==================================================================",
    },
  ]);

  const [inputVal, setInputVal] = useState("");
  const [commandHistory, setCommandHistory] = useState<string[]>([]);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const bottomRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [history]);

  useEffect(() => {
    const handleGlobalError = (event: ErrorEvent) => {
      console.warn("Recovered from UI event error:", event.error);
      event.preventDefault();
    };

    const handleRejection = (event: PromiseRejectionEvent) => {
      console.warn("Recovered from unhandled promise rejection:", event.reason);
      event.preventDefault();
    };

    window.addEventListener("error", handleGlobalError);
    window.addEventListener("unhandledrejection", handleRejection);

    return () => {
      window.removeEventListener("error", handleGlobalError);
      window.removeEventListener("unhandledrejection", handleRejection);
    };
  }, []);

  const playSound = (soundName?: string | null) => {
    if (!soundName) return;
    let audioSrc = "";
    if (soundName === "trollface") audioSrc = trollfaceAudio;
    else if (soundName === "brainrot") audioSrc = brainrotAudio;
    else if (soundName === "mario") audioSrc = marioAudio;
    else if (soundName === "tungtung") audioSrc = tungTungAudio;

    if (audioSrc) {
      try {
        const sound = new Audio(audioSrc);
        sound.volume = 0.85;
        sound.play().catch((e) => console.warn("Audio play blocked by policy:", e));
      } catch (err) {
        console.warn("Audio initialization error:", err);
      }
    }
  };

  const handleKeyDown = async (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      e.preventDefault();
      const trimmed = inputVal.trim();
      if (!trimmed) return;

      const userLine: TerminalLine = {
        id: Date.now(),
        type: "input",
        text: trimmed,
      };

      setCommandHistory((prev) => [...prev, trimmed]);
      setHistoryIndex(-1);
      setInputVal("");

      setHistory((prev) => [...prev, userLine]);

      try {
        const response = await invoke<CommandResponse>("execute_command", {
          cmd: trimmed,
        });

        if (response.output === "CLEAR_TERMINAL_EVENT") {
          setHistory([]);
          return;
        }

        const isTroll = !!response.open_url;
        const outLine: TerminalLine = {
          id: Date.now() + 1,
          type: isTroll ? "troll" : "output",
          text: response.output,
        };
        setHistory((prev) => [...prev, outLine]);

        // Play appropriate troll audio effect
        if (response.sound) {
          playSound(response.sound);
        }

        // Trigger Rickroll external URL if requested
        if (response.open_url) {
          try {
            await openUrl(response.open_url);
          } catch {
            window.open(response.open_url, "_blank");
          }
        }
      } catch (err) {
        setHistory((prev) => [
          ...prev,
          { id: Date.now() + 1, type: "output", text: `[PANIC]: ${String(err)}` },
        ]);
      }
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      if (commandHistory.length > 0) {
        const nextIdx =
          historyIndex === -1
            ? commandHistory.length - 1
            : Math.max(0, historyIndex - 1);
        setHistoryIndex(nextIdx);
        setInputVal(commandHistory[nextIdx]);
      }
    } else if (e.key === "ArrowDown") {
      e.preventDefault();
      if (historyIndex !== -1) {
        const nextIdx = historyIndex + 1;
        if (nextIdx < commandHistory.length) {
          setHistoryIndex(nextIdx);
          setInputVal(commandHistory[nextIdx]);
        } else {
          setHistoryIndex(-1);
          setInputVal("");
        }
      }
    }
  };

  return (
    <div
      className="terminal-screen"
      onClick={() => inputRef.current?.focus()}
      onPaste={(e) => {
        e.preventDefault();
        alert(
          "[SECURITY VIOLATION]: Direct clipboard pasting disabled by kernel security policy. Type your commands manually."
        );
      }}
      onContextMenu={(e) => e.preventDefault()}
    >
      <div className="terminal-logs">
        {history.map((line) => (
          <div key={line.id} className={`log-line ${line.type}`}>
            {line.type === "input" && (
              <span className="prompt-prefix">root@cursed-box:~$ </span>
            )}
            <span className="log-text">{line.text}</span>
          </div>
        ))}

        <div className="input-row">
          <span className="prompt-prefix">root@cursed-box:~$ </span>
          <input
            ref={inputRef}
            type="text"
            className="cli-input"
            value={inputVal}
            autoFocus
            spellCheck={false}
            autoComplete="off"
            onChange={(e) => setInputVal(e.target.value)}
            onKeyDown={handleKeyDown}
          />
        </div>
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
