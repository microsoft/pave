use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[cfg(windows)]
pub fn effective_registry_path() -> Result<String> {
    platform::effective_registry_path()
}

pub fn list_managed() -> Result<Vec<PathBuf>> {
    platform::list_managed()
}

pub fn add(path: &Path) -> Result<bool> {
    let path = dunce::canonicalize(path)
        .with_context(|| format!("Cannot resolve path: {}", path.display()))?;
    let mut paths = list_managed()?;
    if paths.iter().any(|p| p == &path) {
        return Ok(false);
    }

    let mut blocked = list_blocked().unwrap_or_default();
    if blocked.iter().any(|p| p == &path) {
        blocked.retain(|p| p != &path);
        platform::write_blocked(&blocked)?;
    }

    paths.push(path);
    platform::write_managed(&paths)?;
    Ok(true)
}

pub fn remove(path: &Path) -> Result<bool> {
    let path = safe_canonicalize(path);
    let mut paths = list_managed()?;
    let before = paths.len();
    paths.retain(|p| safe_canonicalize(p) != path);
    if paths.len() == before {
        return Ok(false);
    }
    platform::write_managed(&paths)?;
    Ok(true)
}

pub fn block(path: &Path) -> Result<bool> {
    let path = safe_canonicalize(path);
    let mut blocked = list_blocked().unwrap_or_default();
    if blocked.iter().any(|p| safe_canonicalize(p) == path) {
        return Ok(false);
    }
    blocked.push(path);
    platform::write_blocked(&blocked)?;
    Ok(true)
}

pub fn list_blocked() -> Result<Vec<PathBuf>> {
    platform::list_blocked()
}

pub fn is_in_env_path(path: &Path) -> bool {
    let path = safe_canonicalize(path);
    let path_var = std::env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ';' } else { ':' };
    path_var
        .split(separator)
        .any(|d| safe_canonicalize(Path::new(d)) == path)
}

fn safe_canonicalize(path: &Path) -> PathBuf {
    dunce::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(windows)]
mod platform {
    use super::*;
    use anyhow::bail;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};
    use winreg::RegKey;

    const ENV_SUBKEY: &str = "Environment";
    const PATH_VALUE: &str = "Path";
    const SYS_ENV_SUBKEY: &str = r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment";

    pub fn effective_registry_path() -> Result<String> {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let sys_env = hklm.open_subkey_with_flags(SYS_ENV_SUBKEY, KEY_READ)?;
        let sys_path: String = sys_env.get_value(PATH_VALUE).unwrap_or_default();

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let user_env = hkcu.open_subkey_with_flags(ENV_SUBKEY, KEY_READ)?;
        let user_path: String = user_env.get_value(PATH_VALUE).unwrap_or_default();

        let combined = match (sys_path.is_empty(), user_path.is_empty()) {
            (true, true) => String::new(),
            (true, false) => user_path,
            (false, true) => sys_path,
            (false, false) => format!("{sys_path};{user_path}"),
        };

        Ok(expand_environment_strings(&combined))
    }

    fn expand_environment_strings(s: &str) -> String {
        use windows_sys::Win32::System::Environment::ExpandEnvironmentStringsW;

        let wide: Vec<u16> = OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        let len = unsafe { ExpandEnvironmentStringsW(wide.as_ptr(), std::ptr::null_mut(), 0) };
        if len == 0 {
            return s.to_string();
        }

        let mut buf: Vec<u16> = vec![0u16; len as usize];
        let written = unsafe { ExpandEnvironmentStringsW(wide.as_ptr(), buf.as_mut_ptr(), len) };
        if written == 0 || written > len {
            return s.to_string();
        }

        let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        String::from_utf16_lossy(&buf[..end])
    }

    pub fn list_managed() -> Result<Vec<PathBuf>> {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let env = hkcu.open_subkey_with_flags(ENV_SUBKEY, KEY_READ)?;
        let raw: String = env.get_value(PATH_VALUE).unwrap_or_default();
        Ok(raw
            .split(';')
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect())
    }

    pub fn write_managed(paths: &[PathBuf]) -> Result<()> {
        let joined = paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join(";");

        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let (env, _) = hkcu.create_subkey_with_flags(ENV_SUBKEY, KEY_WRITE)?;

        let reg_value = winreg::RegValue {
            vtype: winreg::enums::RegType::REG_EXPAND_SZ,
            bytes: to_reg_expand_sz(&joined),
        };
        env.set_raw_value(PATH_VALUE, &reg_value)?;

        broadcast_environment_change()?;
        Ok(())
    }

    pub fn list_blocked() -> Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }

    pub fn write_blocked(_paths: &[PathBuf]) -> Result<()> {
        Ok(())
    }

    fn to_reg_expand_sz(s: &str) -> Vec<u8> {
        let wide: Vec<u16> = OsStr::new(s)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        wide.iter().flat_map(|&w| w.to_le_bytes()).collect()
    }

    fn broadcast_environment_change() -> Result<()> {
        use windows_sys::Win32::Foundation::HWND;
        use windows_sys::Win32::UI::WindowsAndMessaging::{
            HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE,
        };

        extern "system" {
            fn SendMessageTimeoutW(
                hwnd: HWND,
                msg: u32,
                wparam: usize,
                lparam: *const u16,
                flags: u32,
                timeout: u32,
                result: *mut usize,
            ) -> isize;
        }

        let env_wide: Vec<u16> = OsStr::new("Environment")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let mut result: usize = 0;
        let ret = unsafe {
            SendMessageTimeoutW(
                HWND_BROADCAST as HWND,
                WM_SETTINGCHANGE,
                0,
                env_wide.as_ptr(),
                SMTO_ABORTIFHUNG,
                5000,
                &mut result,
            )
        };
        if ret == 0 {
            bail!("Failed to broadcast environment change (SendMessageTimeoutW returned 0)");
        }
        Ok(())
    }
}

#[cfg(unix)]
mod platform {
    use super::*;
    use std::io::Write;

    fn paths_file() -> Result<PathBuf> {
        let config = dirs::config_dir().context("Could not determine config directory")?;
        Ok(config.join("pave").join("paths"))
    }

    pub fn list_managed() -> Result<Vec<PathBuf>> {
        let file = paths_file()?;
        if !file.exists() {
            return Ok(Vec::new());
        }
        let contents = std::fs::read_to_string(&file)
            .with_context(|| format!("Failed to read {}", file.display()))?;
        Ok(contents
            .lines()
            .filter(|l| !l.is_empty())
            .map(PathBuf::from)
            .collect())
    }

    pub fn write_managed(paths: &[PathBuf]) -> Result<()> {
        let file = paths_file()?;
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut f = std::fs::File::create(&file)?;
        for p in paths {
            writeln!(f, "{}", p.display())?;
        }
        Ok(())
    }

    fn blocked_file() -> Result<PathBuf> {
        let config = dirs::config_dir().context("Could not determine config directory")?;
        Ok(config.join("pave").join("blocked"))
    }

    pub fn list_blocked() -> Result<Vec<PathBuf>> {
        let file = blocked_file()?;
        if !file.exists() {
            return Ok(Vec::new());
        }
        let contents = std::fs::read_to_string(&file)
            .with_context(|| format!("Failed to read {}", file.display()))?;
        Ok(contents
            .lines()
            .filter(|l| !l.is_empty())
            .map(PathBuf::from)
            .collect())
    }

    pub fn write_blocked(paths: &[PathBuf]) -> Result<()> {
        let file = blocked_file()?;
        if let Some(parent) = file.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut f = std::fs::File::create(&file)?;
        for p in paths {
            writeln!(f, "{}", p.display())?;
        }
        Ok(())
    }
}
