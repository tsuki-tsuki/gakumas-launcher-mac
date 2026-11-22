# gakumas-launcher

A dedicated Rust command-line launcher for Gakuen Idolmaster (Gakumas). Upstream supports x64 Windows and Linux (through Wine). **This fork focuses on native macOS launcher support with CrossOver running the Windows game.**

## macOS and CrossOver

The CLI has been built natively for Apple Silicon and used to launch the game locally through CrossOver. Integration is still being developed. Configure `wine` to point to a wrapper that selects CrossOver's underlying Wine loader and matching wineserver, and `wine_prefix` to the game prefix. CrossOver's `bin/wine` management script expects a named bottle; setting `WINEPREFIX` alone does not select a Steam-managed prefix through that script. Use the same runtime for all processes sharing a prefix to avoid Wine client/server version mismatches.

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
  "profile_base_url": "https://raw.githubusercontent.com/huochai67/gakumas-launcher/main",
  "bepinex": "/path/to/BepInEx-Unity.IL2CPP-win-x64.zip",
  "host": "/path/to/gakumas.exe",
  "host_url": "https://github.com/huochai67/gakumas-launcher/releases/latest/download/gakumas.exe",
  "wine": "wine",
  "wine_prefix": "/path/to/wineprefix"
}
```

`dir` also accepts the alias `game_dir`. `profile_base_url` also accepts `profile_repo` and `profile_url`. The example uses upstream's profile and host download URLs. For CrossOver, replace `wine` with the absolute path to your Wine wrapper and `wine_prefix` with your prefix path.

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
