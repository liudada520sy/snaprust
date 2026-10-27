// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arboard::{Clipboard, ImageData};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use image::ImageOutputFormat;
use screenshots::Screen;
use serde::Serialize;
use std::borrow::Cow;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use tauri::Window;

#[derive(Serialize)]
pub struct MonitorInfo {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

#[derive(Serialize)]
pub struct CaptureResult {
    pub width: u32,
    pub height: u32,
    pub base64_png: String,
    pub monitor_id: u32,
}

/// 获取所有连接的显示器列表（跨平台：Windows / macOS / Linux X11 & Wayland）
#[tauri::command]
fn get_monitors() -> Result<Vec<MonitorInfo>, String> {
    let screens = Screen::all().map_err(|e| format!("无法枚举显示器: {}", e))?;
    let mut list = Vec::new();
    for s in screens {
        let info = s.display_info;
        list.push(MonitorInfo {
            id: info.id,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            scale_factor: info.scale_factor,
            is_primary: info.is_primary,
        });
    }
    Ok(list)
}

/// 捕获指定显示器全屏图像并编码为 Base64 Data URL 返回给前端 Canvas
#[tauri::command]
async fn capture_screen(
    window: Window,
    monitor_id: Option<u32>,
    hide_window: bool,
) -> Result<CaptureResult, String> {
    if hide_window {
        let _ = window.hide();
        thread::sleep(Duration::from_millis(220));
    }

    let screens = Screen::all().map_err(|e| format!("获取屏幕列表失败: {}", e))?;
    let target = match monitor_id {
        Some(id) => screens
            .into_iter()
            .find(|s| s.display_info.id == id)
            .or_else(|| Screen::all().ok()?.into_iter().next()),
        None => screens
            .into_iter()
            .find(|s| s.display_info.is_primary)
            .or_else(|| Screen::all().ok()?.into_iter().next()),
    }
    .ok_or_else(|| "未找到可用显示器".to_string())?;

    let captured = target
        .capture()
        .map_err(|e| format!("屏幕捕获失败: {}", e))?;

    let width = captured.width();
    let height = captured.height();

    let mut png_bytes: Vec<u8> = Vec::new();
    captured
        .write_to(&mut Cursor::new(&mut png_bytes), ImageOutputFormat::Png)
        .map_err(|e| format!("PNG 编码失败: {}", e))?;

    if hide_window {
        let _ = window.show();
        let _ = window.set_focus();
    }

    let b64 = BASE64.encode(&png_bytes);
    Ok(CaptureResult {
        width,
        height,
        base64_png: format!("data:image/png;base64,{}", b64),
        monitor_id: target.display_info.id,
    })
}

/// 将前端 Canvas 导出的 Base64 PNG 直接写入系统原生剪贴板
#[tauri::command]
fn copy_image_to_clipboard(base64_png: String) -> Result<(), String> {
    let raw_b64 = base64_png
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_png);

    let bytes = BASE64
        .decode(raw_b64)
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    let img = image::load_from_memory(&bytes)
        .map_err(|e| format!("图像解析失败: {}", e))?
        .into_rgba8();

    let (width, height) = img.dimensions();
    let raw_rgba = img.into_raw();

    let mut clipboard = Clipboard::new().map_err(|e| format!("打开系统剪贴板失败: {}", e))?;
    let img_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::from(raw_rgba),
    };

    clipboard
        .set_image(img_data)
        .map_err(|e| format!("写入剪贴板失败: {}", e))?;

    Ok(())
}

/// 将标注后的 PNG 保存到本地磁盘
#[tauri::command]
fn save_image_to_disk(base64_png: String, file_path: String) -> Result<String, String> {
    let raw_b64 = base64_png
        .strip_prefix("data:image/png;base64,")
        .unwrap_or(&base64_png);

    let bytes = BASE64
        .decode(raw_b64)
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    let path = PathBuf::from(&file_path);
    fs::write(&path, bytes).map_err(|e| format!("文件保存失败: {}", e))?;

    Ok(path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_monitors,
            capture_screen,
            copy_image_to_clipboard,
            save_image_to_disk
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 截图工具时发生错误");
}
