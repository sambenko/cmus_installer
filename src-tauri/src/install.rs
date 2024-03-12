use log::info;

use std::path::Path;
use std::process::Stdio;
use std::sync::Mutex;
use tauri::Window;
use crate::app_state::{AppState, BuilderState};
use tauri::Manager;

use tokio::io::AsyncBufReadExt;
use tokio::process::Command;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

const PROGRESS_EVENT: &str = "progress";

fn is_abort_state(app: tauri::AppHandle) -> bool {
    let state_mutex = app.state::<Mutex<AppState>>();
    let state = state_mutex.lock().unwrap();
    matches!(state.builder, BuilderState::Abort)
}

pub async fn install(
    window: Window,
    app: tauri::AppHandle,
    target_path: String,
) -> Result<String, ()> {
    let folder_path = Path::new(&target_path);
    let configure_script_path = folder_path.join("configure");
    let install_script_path = folder_path.join("scripts/install");

    // Set executable permissions for the scripts
    if let Err(err) = set_exec_permission(&configure_script_path) {
        info!("Failed to set executable permission for configure script: {}", err);
        return Err(());
    }

    if let Err(err) = set_exec_permission(&install_script_path) {
        info!("Failed to set executable permission for install script: {}", err);
        return Err(());
    }

    #[cfg(unix)]
    {
        info!("Running configure script: {:?}", configure_script_path);
        run_external_command_with_progress(
            window.clone(),
            app.clone(),
            configure_script_path.to_str().unwrap(),
            &[],
            PROGRESS_EVENT,
        )
        .await?;

        info!("Running make install");
        run_external_command_with_progress(
            window,
            app,
            "make",
            &["install"],
            PROGRESS_EVENT,
        )
        .await?;
    }

    #[cfg(windows)]
    {
        info!("Running configure script: {:?}", configure_script_path);
        run_external_command_with_progress(
            window.clone(),
            app.clone(),
            configure_script_path.to_str().unwrap(),
            &[],
            PROGRESS_EVENT,
        )
        .await?;

        info!("Running install script: {:?}", install_script_path);
        todo!()
    }

    Ok("Success".to_string())
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn set_exec_permission(path: &std::path::Path) -> std::io::Result<()> {
    use std::fs;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    fs::set_permissions(path, perms)
}

#[cfg(windows)]
fn set_exec_permission(path: &std::path::Path) -> std::io::Result<()> {
    todo!()
}

async fn run_external_command_with_progress(
    window: Window,
    app: tauri::AppHandle,
    cmd_name: &str,
    cmd_args: &[&str],
    progress_event: &str,
) -> Result<String, ()> {
    let cmd_name_owned = cmd_name.to_string();
    let cmd_args_owned: Vec<String> = cmd_args.iter().map(|&s| s.to_string()).collect();

    info!("Command: {} {}", cmd_name_owned, cmd_args_owned.join(" "));

    let child_result = Command::new(&cmd_name_owned)
        .args(&cmd_args_owned)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(e) => {
            info!("Failed to launch command: {:?}", e);
            return Err(());
        }
    };

    let mut stdout = tokio::io::BufReader::new(child.stdout.take().unwrap());
    let mut stderr = tokio::io::BufReader::new(child.stderr.take().unwrap());

    let mut stdout_buf = String::new();
    let mut stderr_buf = String::new();

    let poll_interval = tokio::time::Duration::from_millis(100);

    loop {
        tokio::select! {
            _ = stdout.read_line(&mut stdout_buf) => {
                if !stdout_buf.is_empty() {
                    info!("{}", stdout_buf);
                    emit_progress_event(&window, progress_event, stdout_buf.trim_end().to_string());
                    stdout_buf.clear();
                }
            },
            _ = stderr.read_line(&mut stderr_buf) => {
                if !stderr_buf.is_empty() {
                    info!("{}", stderr_buf);
                    emit_progress_event(&window, progress_event, stderr_buf.trim_end().to_string());
                    stderr_buf.clear();
                }
            },
            status = child.wait() => {
                match status {
                    Ok(status) if status.success() => {
                        info!("Done");
                        return Ok("Child process completed successfully".to_string());
                    },
                    Ok(_) => {
                        info!("Child process exited with an error");
                        return Err(());
                    },
                    Err(err) => {
                        info!("Child process encountered an error: {:?}", err);
                        return Err(());
                    },
                }
            },
            _ = tokio::time::sleep(poll_interval) => {
                if is_abort_state(app.clone()) {
                    info!("Aborting command due to external signal.");
                    let _ = child.kill();
                    return Err(());
                }
            }
        }
    }
}


fn emit_progress_event(window: &Window, event: &str, message: String) {
    let payload = Payload { message };
    window.emit(event, payload).unwrap();
}
