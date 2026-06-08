# YTDLP-SHIM
A thin helper program to sit between VRChat and YT-DLP.exe in order to prevent
the latter from returning media URLs that cause VRChat to segfault in Linux.

## Usage
* Rename `$VRCHAT_ROOT/Tools/yt-dlp.exe` to `real-yt-dlp.exe`
* Copy the compiled `ytdlp-shim.exe` executable to `$VRCHAT_ROOT/Tools/` and rename it to `yt-dlp.exe`
* Force the shim executable to be read-only (`chmod u-w yt-dlp.exe`) to prevent VRChat from overwriting it
* Hopefully enjoy media players without worrying about segfault crashes

## Building
Prerequisites:
* Rust >1.90
* Windows target for Rust (`rustup target add x86_64-pc-windows-gnu`)
* The MinGW toolchain (`mingw-w64` on Debian-based distros, `mingw-w64-gcc` on Arch-based distros)

To actually compile it:
* `cargo build --target x86_64-pc-windows-gnu --release`
* Resulting binary should be in `target/x86_64-pc-windows-gnu/release/ytdlp-shim.exe`

## Notes
* `yt-dlp` will return "No URLs Found" under two circumstances:
    * 1: The URL is pointing at a stream
    * 2: There is no 720p or lower version available
    * It's possible this is also related to ProTV3 just being a terrible prefab (stop using it)
* `$VRCHAT_ROOT` can differ based on your distro and whether or not you installed Steam from a package manager or via Flatpak, but in general it should look like this:
    * `/home/[your user]/.local/share/Steam/steamapps/compatdata/438100/pfx/drive_c/users/steamuser/AppData/LocalLow/VRChat/VRChat`
    * Everything past `steamapps` should be common to all distros and Flatpak.
    * When it doubt just do a `find . -type d -name 438100` from your home directory, as that's VRChat's Steam game ID.

## Exit codes
* 0 - Successfully called the real yt-dlp and passed its output back to VRC
* 1 - Error while executing the real yt-dlp
* 2 - Receivced non-UTF8 output from the real yt-dlp
* Any error messages will appear in the VRChat output logs

#### Misc
(c) 2026 Melondog Software - Released under GPLv3
NO AI TOOLS WERE USED IN THE MAKING OF THIS SOFTWARE
