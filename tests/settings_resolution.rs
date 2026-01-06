use std::ffi::OsString;
use std::path::PathBuf;
use std::sync::Mutex;

use pe_vm::{Architecture, Os, VmConfig};

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn auto_settings_resolves_local_override() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let base = make_temp_dir("settings_resolution");
    std::fs::create_dir_all(&base).expect("create settings dir");

    // settings.local.yml should override settings.yml when both are present.
    std::fs::write(
        base.join("settings.yml"),
        "vm:\n  properties:\n    os: unix\n",
    )
    .expect("write settings.yml");
    std::fs::write(
        base.join("settings.local.yml"),
        "vm:\n  properties:\n    os: windows\n    architecture: x86_64\n",
    )
    .expect("write settings.local.yml");

    let prev_dir = std::env::var_os("PE_VM_SETTINGS_DIR");
    std::env::set_var("PE_VM_SETTINGS_DIR", &base);

    let config = VmConfig::from_default_settings().expect("load settings");
    assert!(matches!(config.os_value(), Os::Windows));
    assert_eq!(config.architecture_value(), Architecture::X86_64);

    restore_env("PE_VM_SETTINGS_DIR", prev_dir);
    let _ = std::fs::remove_dir_all(&base);
}

#[test]
fn auto_settings_resolves_dir_priority_order() {
    let _guard = ENV_LOCK.lock().expect("env lock");
    let first = make_temp_dir("settings_dir_first");
    let second = make_temp_dir("settings_dir_second");
    std::fs::create_dir_all(&first).expect("create first dir");
    std::fs::create_dir_all(&second).expect("create second dir");

    std::fs::write(
        first.join("settings.yml"),
        "vm:\n  properties:\n    os: unix\n",
    )
    .expect("write first settings.yml");
    std::fs::write(
        second.join("settings.yml"),
        "vm:\n  properties:\n    os: windows\n",
    )
    .expect("write second settings.yml");

    let prev_dir = std::env::var_os("PE_VM_SETTINGS_DIR");
    let joined = std::env::join_paths([&first, &second]).expect("join paths");
    std::env::set_var("PE_VM_SETTINGS_DIR", joined);

    let config = VmConfig::from_default_settings().expect("load settings");
    assert!(matches!(config.os_value(), Os::Windows));

    restore_env("PE_VM_SETTINGS_DIR", prev_dir);
    let _ = std::fs::remove_dir_all(&first);
    let _ = std::fs::remove_dir_all(&second);
}

fn make_temp_dir(prefix: &str) -> PathBuf {
    let mut dir = std::env::temp_dir();
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    dir.push(format!("pe_vm_{prefix}_{pid}_{nanos}"));
    dir
}

fn restore_env(key: &str, value: Option<OsString>) {
    if let Some(value) = value {
        std::env::set_var(key, value);
    } else {
        std::env::remove_var(key);
    }
}
