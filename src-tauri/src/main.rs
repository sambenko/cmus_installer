// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod download;
mod unzipper;
mod app_state;
mod install;
mod progress;
mod utils;

use app_state::{AppState, BuilderState};

use tauri::{AppHandle, Window, State};
use std::sync::Mutex;

#[tauri::command]
async fn download_cmus(
    window: Window,
    _app: AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    version: String,
    target_path: String,
) -> Result<String, String> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let download_handle = tokio::spawn(download::download(window, version, target_path.clone()));
    let result = download_handle.await;

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(result) => match result {
            Ok(_) => Ok("Download finished successfully".to_string()),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(format!("Download task panicked: {}", err)),
    }
}

#[tauri::command]
fn decompress(
    window: Window,
    _app: tauri::AppHandle,
    state_mutex: State<'_, Mutex<AppState>>,
    source_path: String,
    target_path: String,
) -> Result<String, String> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let result = unzipper::unzip(window, source_path, target_path);

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Decompression successful".to_string()),
        Err(e) => Err(format!("Decompression failed: {:?}", e)),
    }
}


#[tauri::command]
async fn install_cmus(
    window: Window,
    app: AppHandle,
    state_mutex: tauri::State<'_, Mutex<AppState>>,
    target_path: String,
) -> Result<String, ()> {
    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Running;
    }

    let install_handle = tokio::spawn(install::install(window, app, target_path));
    let result = install_handle.await;

    // let result: Result<String, _> = Err(());

    {
        let mut state = state_mutex.lock().unwrap();
        state.builder = BuilderState::Idle;
    }

    match result {
        Ok(_) => Ok("Installation successful".to_string()),
        Err(_) => Err(()),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![download_cmus, decompress, install_cmus, utils::cleanup, utils::copy_dir ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
