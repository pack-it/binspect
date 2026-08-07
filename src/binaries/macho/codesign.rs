use std::path::Path;

#[cfg(target_os = "macos")]
use std::process::{Command, Stdio};

use crate::macros::error;

/// Signs a binary using the `codesign` command.
#[cfg(target_os = "macos")]
pub fn sign_binary(path: &Path) -> bool {
    let Some(path) = path.to_str() else {
        error!("Cannot pass path to codesign command: path is not valid unicode.");
        return false;
    };

    // Run the codesign utility on macOS
    let mut command = Command::new("/usr/bin/codesign");
    let command = command.args(["--sign", "-", "--force", path]).stdout(Stdio::null()).stderr(Stdio::null());

    match command.status() {
        Ok(code) if !code.success() => {
            error!("Cannot sign binary, codesign exited with code {code}");
            false
        },
        Ok(_) => true,
        Err(e) => {
            error!("Cannot sign binary, codesign exited with error: {e}");
            false
        },
    }
}

/// Shows a message that signing is currently not possible and should be done on macOS.
#[cfg(not(target_os = "macos"))]
pub fn sign_binary(path: &Path) -> bool {
    error!("Cannot sign MachO binary because the current OS is not macOS.");
    error!("Please complete the signing on a macOS system.");
    false
}
