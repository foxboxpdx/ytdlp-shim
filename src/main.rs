//! YTDLP-SHIM
//! 2026 FoxBox @ Melondog Software
//! Receives calls bound for yt-dlp.exe from VRChat and passes them along to
//! the 'real' yt-dlp but with the max resolution limited to 720p.

use std::process::Command;
use std::env;

/// This was taken from an iwaSync3 player with the max resoltion set to
/// 720p.  Forces yt-dlp to a 720p (or lower) video stream if available
const FORMAT: &'static str =
    "(mp4/best)[height<=?720][height>=?64][width>=?64][protocol^=http]";

/// Define the name of the 'real' yt-dlp executable
const YT_DLP: &'static str = "real-yt-dlp.exe";

fn main() {
    // Parse arguments
    let args: Vec<_> = env::args().collect();

    // Iterate through args to build a Vec to send to the 'real' yt-dlp.
    let mut outgoing_args = Vec::new();
    // Keep track of whether we've seen the '-f' flag
    let mut state = 0;
    for a in args.iter() {
        if state == 1 { state = 2; continue; } // Skips the format string arg
        if a.as_str() == "-f" {
            outgoing_args.push(a.to_string());
            outgoing_args.push(FORMAT.to_string());
            state = 1; // We've replaced the format string, skip next arg
        } else {
            // Pass thru any other args
            outgoing_args.push(a.to_string());
        }
    }

    // Sanity check: If state is still 0, it means we never saw the '-f' flag
    // We should add ours in there just to make sure we don't get a bad url
    if state == 0 {
        outgoing_args.push("-f".to_string());
        outgoing_args.push(FORMAT.to_string());
    }

    // Call the 'real' yt-dlp and pass its output back up to VRChat
    match Command::new(YT_DLP).args(outgoing_args).output() {
        Ok(cmd) => {
            // Make sure the output is in utf8, exit if it isn't
            // If everything looks good, return the output to VRChat
            match str::from_utf8(&cmd.stdout) {
                Ok(output) => { print!("{output}"); },
                Err(_) => {
                    eprint!("Error: Non-UTF8 output received from yt-dlp");
                    std::process::exit(2);
                }
            }
        },
        // If the 'real' yt-dlp errors out, print to STDERR and exit
        Err(e) => {
            eprint!("Error executing yt-dlp: {e}");
            std::process::exit(1);
        }
    };
}
