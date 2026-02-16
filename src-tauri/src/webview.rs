use std::path::PathBuf;
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub struct PlatformWebview {
    window: WebviewWindow,
}

impl PlatformWebview {
    pub fn new(
        app_handle: &AppHandle,
        label: &str,
        title: &str,
        url: &str,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        data_dir: PathBuf,
        resizable: bool,
    ) -> Result<Self, String> {
        let webview = WebviewWindowBuilder::new(app_handle, label, WebviewUrl::App(url.into()))
            .title(title)
            .inner_size(width, height)
            .position(x, y)
            .data_directory(data_dir)
            .resizable(resizable)
            .decorations(true)
            .always_on_top(false)
            .focused(true)
            .build()
            .map_err(|e| format!("Failed to build webview: {}", e))?;

        Ok(PlatformWebview { window: webview })
    }

    #[cfg(target_os = "macos")]
    pub fn configure_macos(&self) {
        let window = &self.window;

        window
            .set_title_bar_style(tauri::TitleBarStyle::Overlay)
            .ok();
    }

    #[cfg(target_os = "windows")]
    pub fn configure_windows(&self) {
        let window = &self.window;

        window
            .set_effects(tauri::window::EffectsConfig::default())
            .ok();
    }

    pub fn close(&self) {
        let _ = self.window.close();
    }

    pub fn set_url(&self, url: &str) -> Result<(), String> {
        self.window
            .eval(&format!("window.location.href = '{}'", url))
            .map_err(|e| format!("Failed to change URL: {}", e))
    }

    pub fn label(&self) -> &str {
        self.window.label()
    }

    pub fn inner(&self) -> &WebviewWindow {
        &self.window
    }

    pub fn inner_mut(&mut self) -> &mut WebviewWindow {
        &mut self.window
    }
}

#[cfg(target_os = "macos")]
impl PlatformWebview {
    pub fn with_macos_options(
        app_handle: &AppHandle,
        label: &str,
        title: &str,
        url: &str,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        data_dir: PathBuf,
    ) -> Result<Self, String> {
        let webview = Self::new(
            app_handle, label, title, url, width, height, x, y, data_dir, true,
        )?;

        webview.configure_macos();

        Ok(webview)
    }
}

#[cfg(target_os = "windows")]
impl PlatformWebview {
    pub fn with_windows_options(
        app_handle: &AppHandle,
        label: &str,
        title: &str,
        url: &str,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        data_dir: PathBuf,
    ) -> Result<Self, String> {
        let webview = Self::new(
            app_handle, label, title, url, width, height, x, y, data_dir, true,
        )?;

        webview.configure_windows();

        Ok(webview)
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
impl PlatformWebview {
    pub fn with_platform_options(
        app_handle: &AppHandle,
        label: &str,
        title: &str,
        url: &str,
        width: f64,
        height: f64,
        x: f64,
        y: f64,
        data_dir: PathBuf,
    ) -> Result<Self, String> {
        Self::new(
            app_handle, label, title, url, width, height, x, y, data_dir, true,
        )
    }
}
