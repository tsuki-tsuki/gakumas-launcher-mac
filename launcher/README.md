# gakumas-launcher

A dedicated Rust command-line launcher for Gakuen Idolmaster (Gakumas). Supports x64 Windows and Linux (through Wine), with **macOS support through CrossOver/NotProton** and a native macOS CLI build.

## macOS and CrossOver

### Steam and NotProton on macOS

Use the **Windows CLI** (`gakumas-launcher.exe`) as [NotProton](https://github.com/NotProtonNot/NotProton)'s **Launch Executable**, and select its working folder with **Start Executable From**. Choose the CrossOver compatibility tool and prefix in NotProton as usual. Keep `gakumas-launcher.exe` and the Windows host `gakumas.exe` in the working folder; the game files live in its `game/` subdirectory by default.

The Windows CLI runs inside Wine and starts the game directly as a Windows child process. It inherits the selected runtime, prefix, and graphics environment, and waits until the game exits. No Wine wrapper or `config.json` is required. `--wine` and `--wine-prefix` are for native Unix builds; the Windows CLI uses the Wine session it is already running in.

The launcher uses `NOTPROTON_GAME_CWD` when supplied, otherwise the current process directory. macOS absolute paths such as `/Volumes/Games/Gakumas` are accessed through Wine's `Z:` drive. This also lets it reuse the native launcher's `$HOME/.config/gakumas` session and identity seed. Under Wine/CrossOver, hardware identity uses that stored seed instead of requiring PowerShell hardware queries.

Explicit command-line options override `config.json`. With neither specified, the game directory is `game/` relative to the launcher working directory. For a different layout, pass `--dir` and `--host` in Steam's launch options, for example:

```text
--dir "/Volumes/Games/Gakumas/game" --host "/Volumes/Games/Gakumas/gakumas.exe"
```

Profiles are required for decryption, but this repository's profile URL is already the default; an explicit `--profile-base-url` is unnecessary. `--host-url` is only needed to override host downloads. BepInEx is optional and is not required to launch the base game.

First-time login asks for a callback URL or authorization code. Complete it once from an interactive terminal before launching through Steam; Steam does not provide terminal input. Saved login credentials can then be reused, including those from the native CLI when the same host home directory is inherited.

### Standalone native CLI

Download `gakumas-launcher-aarch64-apple-darwin.tar.gz` for Apple Silicon and extract it into your launcher working folder. Like the Linux archive, it contains the native `gakumas-launcher` and the Windows `gakumas.exe` host. Keep them together so the launcher can find the host automatically. If a local host is unavailable, it can be downloaded and cached automatically, or downloaded separately from the release assets. This package does not include an Intel macOS binary. Rust and Xcode are only needed to build from source, not to run the downloaded CLI.

The native launcher needs:

- A macOS Wine/CrossOver runtime capable of running the Windows game.
- A prepared 64-bit prefix with the game's Windows runtime dependencies.
- A Wine wrapper or equivalent environment that selects the actual Wine loader and matching wineserver.
- An interactive terminal for first-time DMM login. The native macOS CLI opens the login URL in your default browser using `/usr/bin/open`; paste the callback URL or authorization code into the terminal. If browser launch fails, you can open the printed URL manually.

CrossOver's `bin/wine` management script expects a named bottle; setting `WINEPREFIX` alone does not select a Steam-managed prefix through that script. For an **x86_64/Rosetta CrossOver runtime**, save a wrapper such as the following as `wine-crossover.sh`. Set `CX_ROOT` to the CrossOver installation you want to use:

```sh
#!/bin/sh
export CX_ROOT="/Applications/CrossOver.app/Contents/SharedSupport/CrossOver"
export WINELOADER="$CX_ROOT/lib/wine/x86_64-unix/wine"
export WINESERVER="$CX_ROOT/bin/wineserver"
if [ -f "$CX_ROOT/lib64/apple_gptk/external/libd3dshared.dylib" ]; then
    export CX_APPLEGPTK_LIBD3DSHARED_PATH="$CX_ROOT/lib64/apple_gptk/external/libd3dshared.dylib"
fi
exec "$WINELOADER" "$@"
```

This example is for the Rosetta runtime layout tested with CrossOver 26.3/FineWine. Other runtime layouts, including ARM/FEX, need their own matching loader and wineserver paths. Do not mix versions or switch runtimes while another process is using the prefix.

Make the wrapper executable, then run the native CLI from its extracted folder with explicit options:

```sh
chmod +x /path/to/wine-crossover.sh
./gakumas-launcher \
  --dir "/path/to/launcher/game" \
  --wine "/path/to/wine-crossover.sh" \
  --wine-prefix "/path/to/prefix"
```

For a NotProton prefix, `--wine-prefix` must point to `steamapps/compatdata/<AppID>/pfx`, not its parent directory. This standalone command does not obtain a runtime from Steam automatically; use the Windows CLI with NotProton for that integration.

Alternatively, store the options in `$HOME/.config/gakumas/config.json`:

```json
{
  "dir": "/path/to/launcher/game",
  "wine": "/path/to/wine-crossover.sh",
  "wine_prefix": "/path/to/prefix"
}
```

Then run `./gakumas-launcher`. Command-line arguments override the saved options. With no game directory configured, `game/` is relative to the working directory. Leave `--host` unset to allow automatic host discovery and download; use it only to select an existing host file. No explicit profile or host download URL is needed for the default repository. BepInEx remains optional.

The launcher downloads game files, decryption profiles, and a host if needed; it does not install CrossOver or prepare prefix dependencies. Tag releases publish Windows, Linux, and Apple Silicon macOS binaries. All three archives bundle the Windows host. The host is also published as a standalone release asset; see [Releases](../README.md#releases).

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
