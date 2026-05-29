# Monitor Splitter

A focused, open-source Windows app that registers virtual IddCx display adapters to split one physical monitor into N independent virtual ones. Simple UI, hotkeys, no bloat.

Built in Rust (driver + app) for safety, performance, and a single-language codebase.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Tauri UI (app/)                    │
│  Monitor layout canvas · Presets · Hotkeys           │
└──────────────────────┬──────────────────────────────┘
                       │ Named Pipe (\\.\pipe\MonitorSplitter)
                       │ JSON protocol
┌──────────────────────▼──────────────────────────────┐
│              IddCx Virtual Display Driver (driver/)   │
│  Creates/destroys virtual monitors                   │
│  Synthetic EDIDs · Framebuffer region mapping         │
└─────────────────────────────────────────────────────┘
```

## Components

| Crate | Purpose |
|-------|---------|
| `driver/` | IddCx KMDF virtual display driver (Rust, wdk crate) |
| `app/` | Tauri UI + hotkey daemon |
| `common/` | Shared types & protocol definitions |
| `ui/` | Lightweight web frontend (Vite + vanilla JS) |
| `installer/` | NSIS packaging scripts |

## How It Works

1. The IddCx driver registers virtual display adapters with Windows
2. Each virtual monitor gets a synthetic EDID derived from the physical monitor's resolution
3. For example: a 3840×1080 ultrawide → 2× 1920×1080 virtual monitors
4. Virtual monitors share the physical framebuffer (DisplayFusion-style)
5. Each sub-monitor renders independently to its region of the physical display
6. Apps see independent monitors and can snap/maximize to them

---

## 🖥️ Windows Setup Guide (Step-by-Step)

This guide walks you through setting up and running Monitor Splitter from scratch on a Windows machine. No prior Rust or development experience needed.

### Prerequisites

| Tool | Version | Download |
|------|---------|----------|
| **Git** | Any recent | https://git-scm.com/download/win |
| **Rust** | Nightly | https://rustup.rs |
| **Node.js** | 18+ | https://nodejs.org (LTS recommended) |
| **Visual Studio Build Tools** | 2022 | https://aka.ms/vs/17/release/vs_BuildTools.exe |

### Step 1: Install Visual Studio Build Tools

Rust on Windows requires the MSVC C++ toolchain.

1. Download [VS Build Tools 2022](https://aka.ms/vs/17/release/vs_BuildTools.exe)
2. Run the installer
3. Select **"Desktop development with C++"**
4. Make sure these are checked:
   - MSVC v143 build tools
   - Windows 10/11 SDK
5. Click **Install** (this takes ~5-10 minutes)

### Step 2: Install Rust

1. Download and run [rustup-init.exe](https://rustup.rs)
2. Choose **"1) Proceed with installation (default)"**
3. After installation, open a **new** terminal (PowerShell or CMD) and verify:

```powershell
rustup --version
cargo --version
```

4. Install the nightly toolchain (required for the WDK driver):

```powershell
rustup install nightly
rustup default nightly
```

### Step 3: Install Node.js

1. Download [Node.js LTS](https://nodejs.org) and run the installer
2. Accept all defaults
3. Verify in a new terminal:

```powershell
node --version
npm --version
```

### Step 4: Clone the Repository

```powershell
cd C:\Users\%USERNAME%\Documents
git clone https://github.com/LauritzMeier/monitor-splitter.git
cd monitor-splitter
```

### Step 5: Install Frontend Dependencies

```powershell
cd ui
npm install
cd ..
```

### Step 6: Build & Run

#### Option A: Run in Development Mode (recommended for first time)

```powershell
# Terminal 1 — Start the UI dev server
cd ui
npm run dev

# Terminal 2 — Run the Tauri app
cd app
cargo tauri dev
```

#### Option B: Build a Release Binary

```powershell
cd app
cargo tauri build
```

The built `.msi` installer will be in `app/target/release/bundle/msi/`.

### Step 7: Install the Driver (requires Admin)

> ⚠️ The driver currently requires **test-signing mode**. This is normal for development.

Open **PowerShell as Administrator** and run:

```powershell
# Enable test signing (one-time, requires reboot)
bcdedit /set testsigning on

# Install the driver
pnputil /add-driver "C:\path\to\monitor-splitter\driver\MonitorSplitter.inf" /install

# Reboot to activate
Restart-Computer
```

After reboot, you'll see a "Test Mode" watermark in the bottom-right corner of your desktop. This is expected and confirms test-signing is active.

### Step 8: Use the App

1. Launch **Monitor Splitter** from the Start menu or run `monitor-splitter.exe`
2. Select your physical monitor from the dropdown
3. Choose a **Quick Split** preset or define a **Custom Split** (columns × rows)
4. The preview canvas shows your layout
5. Save configurations as presets and assign hotkeys

---

## 🔧 Troubleshooting

| Problem | Solution |
|---------|----------|
| `cargo` not found | Close and reopen your terminal after installing Rust |
| `node` not found | Close and reopen your terminal after installing Node.js |
| Linker errors | Make sure VS Build Tools are installed with C++ workload |
| Driver won't load | Verify test-signing is on: `bcdedit` → look for `testsigning Yes` |
| "Test Mode" watermark | Normal! This is required for unsigned drivers |
| App can't connect to driver | Make sure the driver is installed and you've rebooted |
| `cargo tauri dev` fails | Make sure you ran `npm install` in the `ui/` directory first |

## 🔑 Hotkeys

You can assign global hotkeys to presets via the app UI. Examples:
- `Ctrl+Alt+1` — 2× Horizontal split
- `Ctrl+Alt+2` — 3× Horizontal split
- `Ctrl+Alt+0` — Remove all splits

## Communication Protocol

The app communicates with the driver via a named pipe (`\\.\pipe\MonitorSplitter`) using JSON messages:

```json
{"type": "split", "monitor_index": 0, "splits": 2, "orientation": "horizontal"}
{"type": "remove", "virtual_monitor_id": "vm-1"}
{"type": "query_monitors"}
```

## Driver Signing

- **Development**: Test-signing with self-signed certificate (requires `bcdedit /set testsigning on`)
- **Production/Steam**: EV code-signing certificate or WHCP submission

## Building on macOS/Linux (UI only)

The `common/` and `app/` crates compile on any platform for UI development:

```bash
cd ui && npm install && npm run dev   # Start frontend dev server
cd app && cargo build                 # Build Rust backend (no driver)
```

The `driver/` crate only compiles on Windows with WDK installed. CI handles this automatically.

## License

MIT
