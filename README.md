# Gakumas Decryption Tools and Launcher

This repository provides an offline `GameAssembly.dll` decryption tool based on published version profiles, the profiles for each supported version, and a dedicated Rust launcher.

## Platform support

This project supports Windows and Linux, with a focus on running Gakuen Idolmaster (Gakumas) on **macOS with CrossOver**. The Windows CLI runs inside CrossOver, and a native macOS CLI build can also launch the Windows game through Wine.

For Steam with [NotProton](https://github.com/NotProtonNot/NotProton), use the Windows CLI executable as the launch executable. It runs inside the selected CrossOver prefix and launches the game in the same Wine session, without an external Wine wrapper. The launcher accepts NotProton's working directory and macOS host paths. See the [Steam setup instructions](launcher/README.md#steam-and-notproton-on-macos).

The native Apple Silicon CLI build and game launch have also been tested locally with CrossOver; standalone native launches still need a Wine wrapper.

## Releases

Pushing a tag runs `.github/workflows/build.yml` and publishes a GitHub Release after all three platform builds succeed:

- `gakumas-launcher-x86_64-pc-windows-msvc.zip`: Windows CLI and host, built from the tagged commit. The CLI supports CrossOver/NotProton and retains the native Windows launch path.
- `gakumas-launcher-x86_64-unknown-linux-gnu.tar.gz`: Linux CLI and Windows host, built from the same tagged commit.
- `gakumas-launcher-aarch64-apple-darwin.tar.gz`: Native Apple Silicon macOS CLI and Windows host. Requires a prepared Wine/CrossOver runtime and prefix; see [native macOS setup](launcher/README.md#standalone-native-cli).
- `gakumas.exe`: Windows host as a standalone download.

All three archives include `gakumas.exe`. If a local host is unavailable, the launcher can download and cache it automatically. The host is also available as a standalone release asset. The workflow builds all platforms from this repository. A manual workflow run on a branch builds artifacts without publishing a release.

## Repository layout

| Path | Description |
| --- | --- |
| `status.json` | Current game version, update time (`updated_at`), decryption status (`decrypted`: `true`/`false`), profile path, and original `packed_sha256` |
| `profile/<version>/` | Decryption materials for each version (`manifest.json`, `key.bin`, `sbox.bin`, `pass3.bin`, `records.bin`) |
| `decrypt.py` | Command-line script for decrypting `GameAssembly.dll` with a published profile |
| `decryptor/` | Profile decoder and loadable PE reconstruction module (`ga_static_decrypt.py`), plus the stable import-table list (`decoded-imports.json`) |
| `launcher/` | Dedicated Rust launcher for login, downloads, unpacking, optional BepInEx installation, and game launch; see [`launcher/README.md`](launcher/README.md) |
| `.github/workflows/build.yml` | GitHub Actions workflow that builds the x64 Linux and Windows launchers, the Apple Silicon macOS launcher, and the Windows host (`gakumas.exe`) |

## Using the Python decryption tool

Install the dependencies:

```sh
python3 -m pip install -r requirements.txt
```

Use the matching version profile to decrypt a packed `GameAssembly.dll` into a loadable DLL:

```sh
python3 decrypt.py /path/to/packed/GameAssembly.dll /path/to/output/GameAssembly.dll \
  --profile profile/<version>
```

The script verifies the input `GameAssembly.dll` SHA-256 against `packed_sha256` in `manifest.json` and checks the hashes of all four materials (`key.bin`, `sbox.bin`, `pass3.bin`, `records.bin`). It then decodes the body stream, injects Stock LibCpp2IL registration structures, and rebuilds the `.idata` import table to produce a directly loadable `GameAssembly.dll`.
