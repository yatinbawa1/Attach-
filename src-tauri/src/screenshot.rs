use base64::{engine::general_purpose::STANDARD, Engine as _};
use image::ImageFormat;
use std::io::Cursor;
use tokio::process::Command;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ScreenshotData {
    pub id: Uuid,
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: String,
}

impl ScreenshotData {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            data,
            width,
            height,
            format: "png".to_string(),
        }
    }

    pub fn to_base64(&self) -> String {
        STANDARD.encode(&self.data)
    }

    pub fn to_data_url(&self) -> String {
        format!("data:image/png;base64,{}", self.to_base64())
    }
}

#[cfg(target_os = "macos")]
pub async fn capture_webview_window() -> Result<ScreenshotData, String> {
    let output = Command::new("screencapture")
        .arg("-o")
        .arg("-x")
        .arg("/tmp/webview_capture.png")
        .output()
        .await
        .map_err(|e| format!("Failed to execute screencapture: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let data = std::fs::read("/tmp/webview_capture.png")
        .map_err(|e| format!("Failed to read screenshot: {}", e))?;

    let img = image::load_from_memory(&data).map_err(|e| format!("Failed to load image: {}", e))?;

    let mut cursor = Cursor::new(Vec::new());
    img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(ScreenshotData::new(
        cursor.into_inner(),
        img.width(),
        img.height(),
    ))
}

#[cfg(target_os = "windows")]
pub async fn capture_webview_window() -> Result<ScreenshotData, String> {
    use std::mem::zeroed;
    use std::os::windows::process::CommandExt;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::{HWND, RECT};
    use windows::Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, DeleteObject, GetDIBits, GetDesktopWindow, GetWindowDC,
        ReleaseDC, SelectObject, HDC, SRCCOPY,
    };
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowRect};

    let window_name: Vec<u16> = "Webview\0".encode_utf16().collect();
    let window_name_ptr = PCWSTR::from_raw(window_name.as_ptr());
    let hwnd = unsafe { FindWindowW(window_name_ptr, None) };

    let mut rect = RECT::default();
    unsafe { GetWindowRect(hwnd, &mut rect) };

    let width = (rect.right - rect.left) as u32;
    let height = (rect.bottom - rect.top) as u32;

    if width == 0 || height == 0 {
        return Err("Window has zero size".to_string());
    }

    let hwnd_desktop = unsafe { GetDesktopWindow() };
    let hdc = unsafe { GetWindowDC(hwnd_desktop) };

    let hdc_mem = unsafe { CreateCompatibleDC(hdc) };
    let hbitmap = unsafe { CreateCompatibleBitmap(hdc, width as i32, height as i32) };

    unsafe { SelectObject(hdc_mem, hbitmap) };

    let result = unsafe {
        BitBlt(
            hdc_mem,
            0,
            0,
            width as i32,
            height as i32,
            hdc,
            rect.left,
            rect.top,
            SRCCOPY,
        )
    };

    unsafe { ReleaseDC(hwnd_desktop, hdc) };

    if !result.as_bool() {
        unsafe { DeleteObject(hbitmap) };
        unsafe { DeleteObject(hdc_mem) };
        return Err("BitBlt failed".to_string());
    }

    let mut bitmap_data = vec![0u8; (width * 4 * height) as usize];
    let mut bmi: BITMAPINFO = unsafe { zeroed() };
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = width as i32;
    bmi.bmiHeader.biHeight = -(height as i32);
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi.bmiHeader.biCompression = BI_RGB;

    let success = unsafe {
        GetDIBits(
            hdc_mem,
            hbitmap,
            0,
            height,
            Some(bitmap_data.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        )
    }
    .as_bool();

    unsafe { DeleteObject(hbitmap) };
    unsafe { DeleteObject(hdc_mem) };

    if !success {
        return Err("GetDIBits failed".to_string());
    }

    let mut rgba_data = Vec::with_capacity((width * 4 * height) as usize);
    for y in 0..height {
        let row_start = (y * width * 4) as usize;
        for x in 0..width {
            let offset = row_start + (x * 4) as usize;
            rgba_data.push(bitmap_data[offset + 2]);
            rgba_data.push(bitmap_data[offset + 1]);
            rgba_data.push(bitmap_data[offset]);
            rgba_data.push(255);
        }
    }

    let img = image::DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(width, height, rgba_data).ok_or("Failed to create image")?,
    );

    let mut cursor = Cursor::new(Vec::new());
    img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(ScreenshotData::new(cursor.into_inner(), width, height))
}

#[cfg(target_os = "linux")]
pub async fn capture_webview_window() -> Result<ScreenshotData, String> {
    let output = Command::new("scrot")
        .arg("--select")
        .arg("--freeze")
        .arg("/tmp/webview_capture.png")
        .output()
        .await
        .map_err(|e| format!("Failed to execute scrot: {}", e))?;

    if !output.status.success() {
        let output = Command::new("import")
            .arg("root")
            .arg("/tmp/webview_capture.png")
            .output()
            .await
            .map_err(|e| format!("Failed to execute import: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    }

    let data = std::fs::read("/tmp/webview_capture.png")
        .map_err(|e| format!("Failed to read screenshot: {}", e))?;

    let img = image::load_from_memory(&data).map_err(|e| format!("Failed to load image: {}", e))?;

    let mut cursor = Cursor::new(Vec::new());
    img.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(ScreenshotData::new(
        cursor.into_inner(),
        img.width(),
        img.height(),
    ))
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub async fn capture_webview_window() -> Result<ScreenshotData, String> {
    Err("Screenshot not supported on this platform".to_string())
}

#[cfg(target_os = "windows")]
mod windows_screenshot_helpers {
    use std::mem::zeroed;
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleBitmap, CreateCompatibleDC, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HBITMAP, HDC,
    };

    pub const BI_RGB: u32 = 0;
    pub const DIB_RGB_COLORS: u32 = 0;
}

#[cfg(target_os = "windows")]
use windows_screenshot_helpers::{BI_RGB, DIB_RGB_COLORS};
