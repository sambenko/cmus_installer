use futures_util::stream::StreamExt;
use reqwest::{
    header::{ACCEPT, USER_AGENT},
    Client,
};
use std::{fs, io::Write, time::Instant};
use tauri::Window;

use crate::progress::Progress;

const UPDATE_SPEED: u128 = 50;

#[tauri::command]
pub async fn download(window: Window, version: String, target_path: String) -> Result<(), String> {
    let url = format!(
        "https://github.com/cmus/cmus/archive/refs/tags/{}.zip",
        version
    );
    let path = format!("{}/cmus-{}.zip", target_path, version);
    let start_time = Instant::now();

    let client = Client::new();
    let res = client
        .get(&url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0",
        )
        .header(ACCEPT, "application/octet-stream")
        .send()
        .await
        .map_err(|e| format!("Parsing failed for {}: {}", &url, e))?;

    let mut progress = Progress {
        filesize: res.content_length(),
        transfered: 0,
        transfer_rate: 0.0,
        percentage: 0.0,
    };

    let mut file = fs::File::create(&path).or(Err(format!("File creation failed on {}", &path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut last_update = std::time::Instant::now();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Failed to download file `{}`", &target_path)))?;
        file.write_all(&chunk)
            .or(Err(format!("Failed to write file `{}`", &target_path)))?;
        downloaded += chunk.len() as u64;
        progress.transfered = downloaded;
        if let Some(filesize) = progress.filesize {
            progress.percentage = (progress.transfered * 100 / filesize) as f64;
        } else {
            progress.percentage = 99.9;
        }
        progress.transfer_rate = (downloaded as f64)
            / (start_time.elapsed().as_secs() as f64)
            + (start_time.elapsed().subsec_nanos() as f64 / 1_000_000_000.0).trunc();

        if last_update.elapsed().as_millis() >= UPDATE_SPEED {
            progress.emit_progress(&window);
            last_update = std::time::Instant::now();
        }
    }

    progress.emit_finished(&window);

    return Ok(());
}
