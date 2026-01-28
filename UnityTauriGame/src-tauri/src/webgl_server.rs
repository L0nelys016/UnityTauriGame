use std::process::{Child, Command};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static WEBGL_SERVER: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

fn find_python_command() -> Result<String, String> {
    let commands = ["python3", "python", "py"];
    
    for cmd in &commands {
        let result = Command::new(cmd)
            .arg("--version")
            .output();
        
        if result.is_ok() {
            return Ok(cmd.to_string());
        }
    }
    
    Err("No Python interpreter found (tried: python3, python, py)".to_string())
}

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

    //ресурсы относительно исполняемого файла
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .ok_or("Failed to get executable parent directory")?
        .to_path_buf();

    //несколько возможных расположений ресурсов
    let possible_paths = [
        exe_dir.join("resources").join("unity"),
        exe_dir.join("../resources").join("unity"), // для режима разработки
        exe_dir.join("../../resources").join("unity"), // альтернативный путь для разработки
        std::env::current_dir().unwrap().join("src-tauri").join("resources").join("unity"), // путь для разработки
    ];

    let unity_dir = possible_paths
        .iter()
        .find(|path| path.exists())
        .ok_or("Unity resources directory not found")?;

    let script_path = unity_dir.join("serve_webgl.py");

    if !script_path.exists() {
        return Err(format!("Script file not found: {:?}", script_path));
    }

    let python_cmd = find_python_command()?;
    let child = Command::new(python_cmd)
        .arg(&script_path)
        .current_dir(unity_dir)
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