# gakumas-launcher

A dedicated Rust command-line launcher for Gakuen Idolmaster (Gakumas). Supports x64 Windows and Linux (through Wine), with **macOS support through CrossOver/NotProton** and a native macOS CLI build.

## macOS and CrossOver

### Steam and NotProton on macOS

Use the **Windows CLI** (`gakumas-launcher.exe`) as NotProton's **Launch Executable**, and select its working folder with **Start Executable From**. Choose the CrossOver compatibility tool and prefix in NotProton as usual. Keep `gakumas-launcher.exe` and the Windows host `gakumas.exe` in the working folder; the game files live in its `game/` subdirectory by default.

The Windows CLI runs inside Wine and starts the game directly as a Windows child process. It inherits the selected runtime, prefix, and graphics environment, and waits until the game exits. No Wine wrapper or `config.json` is required. `--wine` and `--wine-prefix` are for native Unix builds; the Windows CLI uses the Wine session it is already running in.

The launcher uses `NOTPROTON_GAME_CWD` when supplied, otherwise the current process directory. macOS absolute paths such as `/Volumes/Games/Gakumas` are accessed through Wine's `Z:` drive. This also lets it reuse the native launcher's `$HOME/.config/gakumas` session and identity seed. Under Wine/CrossOver, hardware identity uses that stored seed instead of requiring PowerShell hardware queries.

Explicit command-line options override `config.json`. With neither specified, the game directory is `game/` relative to the launcher working directory. For a different layout, pass `--dir` and `--host` in Steam's launch options, for example:

```text
--dir "/Volumes/Games/Gakumas/game" --host "/Volumes/Games/Gakumas/gakumas.exe"
```

Profiles are required for decryption, but this repository's profile URL is already the default; an explicit `--profile-base-url` is unnecessary. `--host-url` is only needed to override host downloads. BepInEx is optional and is not required to launch the base game.

First-time login asks for a callback URL or authorization code. Complete it once from an interactive terminal before launching through Steam; Steam does not provide terminal input. Saved login credentials can then be reused, including those from the native CLI when the same host home directory is inherited.

### Standalone native CLI

The CLI has also been built natively for Apple Silicon and used to launch the game locally through CrossOver. Configure `wine` to point to a wrapper that selects CrossOver's underlying Wine loader and matching wineserver, and `wine_prefix` to the game prefix. CrossOver's `bin/wine` management script expects a named bottle; setting `WINEPREFIX` alone does not select a Steam-managed prefix through that script. Use the same runtime for all processes sharing a prefix to avoid Wine client/server version mismatches.

The launcher downloads game files, decryption profiles, and the Windows host as needed. It does not install CrossOver or set up the prefix's runtime dependencies. The existing release workflow builds Windows and Linux binaries.

## Architecture

- **Built-in pure Rust static unpacking engine (`src/decrypt.rs`)**: No Python or `pycryptodome` installation is needed. Using a version profile (`key.bin`, `sbox.bin`, `pass3.bin`, `records.bin`), the engine decrypts `GameAssembly.dll`, aligns Il2Cpp registration structures, and reconstructs `.idata` and the CRT entry point in memory.
- **Single-binary Windows bootstrap (`src/host.rs`)**: On Windows, `gakumas-launcher.exe` includes the unpacked Unity host logic. When copied into the game directory as `gakumas.exe`, it automatically enters Unity host mode, loading `GameAssembly.dll` and calling `UnityPlayer.dll!UnityMain`. No separate host executable is required on Windows.
- **Automatic Windows host download and caching for Wine**: The launcher searches for `gakumas.exe` using `--host`, local candidates such as the launcher's directory, and `$HOME/.config/gakumas/gakumas.exe`. If no local host is available, it downloads one from a GitHub Release associated with the remote profile repository, or from `host_url`, and caches it at `$HOME/.config/gakumas/gakumas.exe`.

## Data directory and configuration (`$HOME/.config/gakumas`)

The launcher stores its data and configuration in `$HOME/.config/gakumas`:

- **`$HOME/.config/gakumas/session.json`**: Stores DMM login credentials (`access_token`, `expires_in_seconds`, `stored_at`).
- **`$HOME/.config/gakumas/config.json`**: Optional configuration for the default game directory, remote profile repository, and other options. Command-line arguments take precedence over `config.json`.

```json
{
  "dir": "/path/to/game",
  "profile_base_url": "https://raw.githubusercontent.com/tsuki-tsuki/gakumas-launcher-mac/main",
  "bepinex": "/path/to/BepInEx-Unity.IL2CPP-win-x64.zip",
  "host": "/path/to/gakumas.exe",
  "host_url": "https://github.com/tsuki-tsuki/gakumas-launcher-mac/releases/latest/download/gakumas.exe",
  "wine": "wine",
  "wine_prefix": "/path/to/wineprefix"
}
```

`dir` also accepts the alias `game_dir`. `profile_base_url` also accepts `profile_repo` and `profile_url`. The example uses this repository's public profile and host download URLs. For CrossOver, replace `wine` with the absolute path to your Wine wrapper and `wine_prefix` with your prefix path.

## Usage

### 1. Automatic launch (recommended)

After configuring `$HOME/.config/gakumas/config.json`, run `gakumas-launcher` without arguments:

```sh
# Check the login session (prompt for login if needed), download and verify game
# files, unpack using a matching profile, deploy the host, and launch the game.
gakumas-launcher
```

### 2. Individual commands (optional)

```sh
gakumas-launcher login
gakumas-launcher logout
gakumas-launcher download --jobs 8
gakumas-launcher decrypt
gakumas-launcher prepare
gakumas-launcher launch
```
