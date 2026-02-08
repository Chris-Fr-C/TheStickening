use crate::setupinterface::SetupInterface;

/// Platform-specific app setup
pub fn setup_app() -> Result<AppSetup, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        crate::setupwin::WindowsSetup::setup()
    }
    #[cfg(target_os = "linux")]
    {
        crate::setuplinux::LinuxSetup::setup()
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        Err("Unsupported platform".into())
    }
}

pub use crate::setupinterface::SetupComponents as AppSetup;
