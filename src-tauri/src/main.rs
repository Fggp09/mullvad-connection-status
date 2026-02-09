//! Main Rust application entry point with VPN status monitoring
//!
//! @file main.rs
//! @created 2025-02-01
//! @author Christian Blank <christianblank91@gmail.com>
//! @copyright 2025 Christian Blank

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod vpn_checker;

use std::sync::{Arc, Mutex};
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Emitter, Manager, State,
};
use tokio::time::{interval, Duration};
use vpn_checker::{check_vpn_status, VpnStatus};

/// Application state shared across threads
struct AppState {
    last_status: Arc<Mutex<Option<VpnStatus>>>,
}

/// Returns the current VPN status from the application state
/// Called from the frontend via Tauri command
#[tauri::command]
async fn get_vpn_status(state: State<'_, AppState>) -> Result<VpnStatus, String> {
    let status = state.last_status.lock().unwrap();
    status.clone().ok_or_else(|| "Status not available yet".to_string())
}

/// Updates the system tray icon based on connection status
/// Green icon for connected, red for disconnected
fn update_tray_icon(app: &AppHandle, connected: bool) {
    if let Some(tray) = app.tray_by_id("main-tray") {
        // Choose color based on connection status
        let color = if connected {
            (0x44, 0xad, 0x4d) // Mullvad green for connected
        } else {
            (0xe3, 0x40, 0x39) // Mullvad red for disconnected
        };

        // Create new icon with appropriate color
        let icon = create_tray_icon(color);
        let _ = tray.set_icon(Some(icon));

        // Update tooltip
        let tooltip = if connected {
            "Mullvad VPN: Connected"
        } else {
            "Mullvad VPN: Disconnected"
        };
        let _ = tray.set_tooltip(Some(tooltip));
    }
}

/// Starts the background task that polls the Mullvad API
/// Checks status every 15 seconds and emits events on changes
fn start_status_monitor(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut poll_interval = interval(Duration::from_secs(15));

        loop {
            poll_interval.tick().await;

            match check_vpn_status().await {
                Ok(status) => {
                    // Get previous status to detect changes
                    let state = app.state::<AppState>();
                    let mut last_status = state.last_status.lock().unwrap();

                    let status_changed = match &*last_status {
                        Some(prev) => prev.connected != status.connected,
                        None => true,
                    };

                    // Update stored status
                    *last_status = Some(status.clone());
                    drop(last_status);

                    // Update UI and tray
                    update_tray_icon(&app, status.connected);
                    let _ = app.emit("vpn-status-changed", status.clone());

                    // Show notification on status change
                    if status_changed {
                        let message = if status.connected {
                            format!(
                                "Connected to {} server",
                                status.country.as_deref().unwrap_or("Mullvad")
                            )
                        } else {
                            "VPN connection lost".to_string()
                        };

                        #[cfg(desktop)]
                        {
                            use tauri_plugin_notification::NotificationExt;
                            let _ = app
                                .notification()
                                .builder()
                                .title("Mullvad Connection Status")
                                .body(&message)
                                .show();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to check VPN status: {}", e);
                }
            }
        }
    });
}

/// Creates a simple tray icon with a shield design
/// Uses raw RGBA data to create a 32x32 icon
fn create_tray_icon(color: (u8, u8, u8)) -> Image<'static> {
    let size = 32u32;
    let mut rgba = vec![0u8; (size * size * 4) as usize];

    // Create a simple shield shape
    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;

            // Simple shield outline shape
            let cx = size as f32 / 2.0;
            let cy = size as f32 / 2.0;
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;

            // Shield shape: rounded top, pointed bottom
            let in_shield = if y < size / 2 {
                // Top half: circle
                (dx * dx + dy * dy) < (size as f32 / 2.5).powi(2)
            } else {
                // Bottom half: triangle pointing down
                dx.abs() < (size as f32 / 2.5) * (1.0 - (y as f32 - size as f32 / 2.0) / (size as f32 / 2.0))
            };

            if in_shield {
                rgba[idx] = color.0;     // R
                rgba[idx + 1] = color.1; // G
                rgba[idx + 2] = color.2; // B
                rgba[idx + 3] = 255;     // A
            } else {
                rgba[idx + 3] = 0; // Transparent
            }
        }
    }

    Image::new_owned(rgba, size, size)
}

/// Sets up the system tray icon and menu
/// Creates menu items for showing the window and quitting
fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let show_item = MenuItemBuilder::with_id("show", "Show Status").build(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&show_item)
        .separator()
        .item(&quit_item)
        .build()?;

    // Create embedded tray icon with Mullvad blue color
    let icon = create_tray_icon((0x29, 0x4d, 0x73));

    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .tooltip("Mullvad Connection Status")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize app state
            app.manage(AppState {
                last_status: Arc::new(Mutex::new(None)),
            });

            // Setup system tray
            setup_tray(app)?;

            // Start background status monitoring
            start_status_monitor(app.handle().clone());

            // Handle window close event to hide instead of quit
            let window = app.get_webview_window("main").unwrap();
            let window_clone = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // Don't close, just hide
                    window_clone.hide().unwrap();
                    api.prevent_close();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_vpn_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run();
}
