use crate::paths;
use std::process::Command;

const BRAVE_ARGS: [&str; 2] = [
    // the windows storage apis needed for this are currently stubbed in wine and brave crashes
    "--disable-features=HardwareMediaKeyHandling",
    // removes the "enable gpu acceleration" error on the video player
    "--use-gl=desktop",
];

fn brave_command() -> Result<Command, String> {
    let log_file = std::fs::File::create(paths::brave_log_file())
        .map_err(|e| format!("could not create log file: {}", e))?;
    let log_file_err = log_file
        .try_clone()
        .map_err(|e| format!("could not clone log file: {}", e))?;

    let current_url = std::fs::read_to_string(paths::current_url_file())
        .map_err(|_| "Failed to read current URL".to_string())?;

    let mut cmd = Command::new(paths::wine_bin());
    cmd.arg(paths::brave_exe());
    cmd.arg(format!("--app={}", current_url));
    cmd.args(BRAVE_ARGS);
    cmd.env("WINEPREFIX", paths::prefix_dir());
    cmd.env("WINEDEBUG", "-all");
    // no start menu or desktop entries from wine
    cmd.env("WINEDLLOVERRIDES", "winemenubuilder.exe=d");
    cmd.stdout(log_file);
    cmd.stderr(log_file_err);
    Ok(cmd)
}

// used when the desktop icon is clicked, stays alive while brave runs
pub fn launch_prime_and_wait() -> Result<(), String> {
    let started = std::time::Instant::now();
    let status = brave_command()?
        .status()
        .map_err(|e| format!("could not start wine: {}", e))?;

    // dying this fast means something is broken
    if started.elapsed().as_secs() < 15 && !status.success() {
        return Err(format!(
            "brave closed right away, check the log at {}",
            paths::brave_log_file().display()
        ));
    }
    Ok(())
}

// used by the play button in the gui
pub fn launch_prime_detached() -> Result<(), String> {
    brave_command()?
        .spawn()
        .map_err(|e| format!("could not start wine: {}", e))?;
    Ok(())
}

// stops everything running in our prefix
pub fn kill_wine() -> Result<(), String> {
    let status = Command::new(paths::wineserver_bin())
        .arg("-k")
        .env("WINEPREFIX", paths::prefix_dir())
        .status()
        .map_err(|e| format!("could not run wineserver: {}", e))?;
    if !status.success() {
        return Err("wineserver -k failed, maybe nothing was running".to_string());
    }
    Ok(())
}
