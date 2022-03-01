mod download;
mod target;
use std::{io::ErrorKind, process::Command};

pub use download::*;
pub use target::*;

pub fn run_command(cmd: &mut Command, program: &str) {
    println!(
        "current_dir: {:?}\nrunning: {:?}",
        cmd.get_current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or("".to_string()),
        cmd
    );
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            panic!(
                "{}",
                &format!(
                    "failed to execute command: {}\nis `{}` not installed?",
                    e, program
                )
            );
        }
        Err(e) => panic!("{}", &format!("failed to execute command: {:?}", e)),
    };
    if !status.success() {
        panic!(
            "{}",
            &format!("command did not execute successfully, got: {}", status)
        );
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn static_lib_filename(lib_name: &str) -> String {
    format!("lib{}.a", lib_name)
}

#[cfg(target_os = "windows")]
pub fn static_lib_filename(lib_name: &str) -> String {
    format!("{}.lib", lib_name)
}
