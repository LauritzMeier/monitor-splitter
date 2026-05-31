# Monitor Splitter

Split one physical monitor into multiple independent virtual monitors. Open-source, no bloat.

## Install (end user)

1. Download `MonitorSplitter-Setup.exe` from [Releases](https://github.com/LauritzMeier/monitor-splitter/releases)
2. Run it — the installer handles everything (driver, app, test-signing)
3. Reboot when prompted
4. Launch **Monitor Splitter** from the Start menu

That's it. No Rust, no Node.js, no command line needed.

## How it works

The app installs an IddCx virtual display driver that creates virtual monitors mapped to regions of your physical display. Windows treats them as real monitors — apps snap to them, taskbars appear, etc.

```
Physical monitor (3840×1080)
┌──────────────────┬──────────────────┐
│  VM1: 1920×1080  │  VM2: 1920×1080  │
│       50%        │       50%        │
└──────────────────┴──────────────────┘
```

## Build from source (developer)

**Requirements:** Windows, PowerShell.

```powershell
git clone https://github.com/LauritzMeier/monitor-splitter.git
cd monitor-splitter
powershell -ExecutionPolicy Bypass -File setup-dev.ps1
```

The setup script installs Rust, VS Build Tools, and tauri-cli automatically.

Then:

```powershell
cd app
cargo tauri dev      # run in dev mode
cargo tauri build    # build release installer
```

No Node.js required — the UI is a single static HTML file.

## Project structure

```
app/        Tauri desktop app (Rust backend)
common/     Shared types & protocol
driver/     IddCx virtual display driver (Windows only)
ui/         Single-file frontend (index.html)
installer/  NSIS installer script
```

## Driver signing

- **Now:** Test-signing (installer enables it automatically, shows "Test Mode" watermark)
- **Later:** EV code-signing certificate for production releases

## License

MIT
