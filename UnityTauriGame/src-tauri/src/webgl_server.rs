use std::process::{Child, Command};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static WEBGL_SERVER: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

#[derive(Default)]
pub struct WebglServerState;

#[tauri::command]
pub fn webgl_start() -> Result<WebglStatus, String> {
    webgl_stop().ok();

    let mut lock = WEBGL_SERVER.lock().unwrap();
    if lock.is_some() {
        return Ok(WebglStatus {
            running: true,
            url: Some("http://localhost:8000/".to_string()),
            port: Some(8000),
        });
    }

    let child = Command::new("py")
    .arg("F:\\Learning\\UnityTauriGame\\UnityTauriGame\\src-tauri\\resources\\unity\\serve_webgl.py")
    .current_dir("F:\\Learning\\UnityTauriGame\\UnityTauriGame\\src-tauri\\resources\\unity")
    .spawn()
    .map_err(|e| format!("Failed to start server: {}", e))?;


    *lock = Some(child);

    Ok(WebglStatus {
        running: true,
        url: Some("http://localhost:8000/".to_string()),
        port: Some(8000),
    })
}

#[derive(serde::Serialize)]
pub struct WebglStatus {
    pub running: bool,
    pub url: Option<String>,
    pub port: Option<u16>,
}


#[tauri::command]
pub fn webgl_stop() -> Result<String, String> {
    let mut lock = WEBGL_SERVER.lock().unwrap();
    if let Some(mut child) = lock.take() {
        child.kill().map_err(|e| format!("Failed to stop server: {}", e))?;
        Ok("Server stopped".into())
    } else {
        Ok("Server not running".into())
    }
}

#[tauri::command]
pub fn webgl_status() -> Result<WebglStatus, String> {
    let lock = WEBGL_SERVER.lock().unwrap();
    Ok(WebglStatus {
        running: lock.is_some(),
        url: if lock.is_some() { Some("http://localhost:8000/".to_string()) } else { None },
        port: if lock.is_some() { Some(8000) } else { None },
    })
}