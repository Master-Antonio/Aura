use tauri::WebviewWindow;

#[cfg(target_os = "windows")]
use window_vibrancy::apply_acrylic;
#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};

/// Applies platform-specific window effects
pub fn setup_window_effects(window: &WebviewWindow) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        apply_acrylic(window, Some((0, 0, 25, 125)))?;
    }

    #[cfg(target_os = "macos")]
    {
        apply_vibrancy(
            window,
            NSVisualEffectMaterial::HudWindow,
            Some(NSVisualEffectState::Active),
            None,
        )?;
    }

    Ok(())
}

/// No-op for platforms without specific window effects
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub fn setup_window_effects(_window: &Window) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
