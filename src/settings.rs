use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_yaml::Value as YamlValue;

use crate::vm::windows;
use crate::vm::{Architecture, Os, PathMapping, SandboxConfig, VmConfig, VmError};

const SETTINGS_FILES: [SettingsFileSpec; 2] = [
    SettingsFileSpec {
        name: "settings.yml",
        priority: 1,
    },
    SettingsFileSpec {
        name: "settings.local.yml",
        priority: 2,
    },
];
const DEFAULT_SETTINGS_DIRS: [SettingsDirSpec; 3] = [
    SettingsDirSpec {
        kind: SettingsDirKind::Root,
        priority: 0,
    },
    SettingsDirSpec {
        kind: SettingsDirKind::HomePeVm,
        priority: 1,
    },
    SettingsDirSpec {
        kind: SettingsDirKind::Cwd,
        priority: 2,
    },
];
const SETTINGS_ENV_DIR: &str = "PE_VM_SETTINGS_DIR";
const SETTINGS_ENV_PRIORITY: &str = "PE_VM_SETTINGS_PRIORITY";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SettingsDirKind {
    Root,
    HomePeVm,
    Cwd,
}

#[derive(Clone, Copy)]
struct SettingsFileSpec {
    name: &'static str,
    priority: u32,
}

#[derive(Clone, Copy)]
struct SettingsDirSpec {
    kind: SettingsDirKind,
    priority: u32,
}

struct SettingsDir {
    path: PathBuf,
    priority: u32,
}

struct SettingsCandidate {
    path: PathBuf,
    dir_priority: u32,
    file_priority: u32,
}

#[derive(Clone, Default)]
pub(crate) struct Settings {
    vm: VmSettings,
    pe: PeSettings,
    sandbox: Option<SandboxConfig>,
    bypass: BypassSettings,
}

/// Bypass configuration for unimplemented or stub features.
#[derive(Clone, Default, Debug)]
pub struct BypassSettings {
    /// If true, do not panic when calling an unimplemented module function.
    pub not_implemented_module: bool,
}

impl BypassSettings {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Default)]
struct VmSettings {
    os: Option<Os>,
    architecture: Option<Architecture>,
    registry_path: Option<String>,
    paths: PathMapping,
}

#[derive(Clone, Default)]
struct PeSettings {
    paths: PathMapping,
}

#[derive(Default, Deserialize)]
struct SettingsRaw {
    vm: Option<VmSectionRaw>,
    pe: Option<PeSectionRaw>,
    sandbox: Option<YamlValue>,
    bypass: Option<BypassSectionRaw>,
}

#[derive(Default, Deserialize)]
struct BypassSectionRaw {
    not_implemented_module: Option<bool>,
}

#[derive(Default, Deserialize)]
struct VmSectionRaw {
    properties: Option<VmPropertiesRaw>,
    paths: Option<YamlValue>,
}

#[derive(Default, Deserialize)]
struct VmPropertiesRaw {
    os: Option<String>,
    architecture: Option<String>,
    path: Option<String>,
}

#[derive(Default, Deserialize)]
struct PeSectionRaw {
    paths: Option<YamlValue>,
}

pub(crate) fn load_settings(path: impl AsRef<Path>) -> Result<Settings, VmError> {
    let path = path.as_ref();
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Settings::default()),
        Err(err) => return Err(VmError::Io(err)),
    };
    parse_settings_contents(&contents)
}

pub(crate) fn load_auto_settings() -> Result<Settings, VmError> {
    let mut merged = Settings::default();
    let mut found = false;
    for candidate in settings_search_candidates() {
        if let Some(settings) = load_settings_file(&candidate.path)? {
            merged.merge_from(settings);
            found = true;
        }
    }
    if !found {
        return Ok(Settings::default());
    }
    Ok(merged)
}

pub(crate) fn load_default_settings() -> Option<Settings> {
    match load_auto_settings() {
        Ok(settings) if settings.is_empty() => None,
        Ok(settings) => Some(settings),
        Err(err) => {
            if std::env::var("PE_VM_TRACE").is_ok() {
                eprintln!("[pe_vm] settings load failed: {err}");
            }
            None
        }
    }
}

pub(crate) fn apply_vm_settings(
    mut config: VmConfig,
    settings: &Settings,
) -> Result<VmConfig, VmError> {
    if let Some(os) = settings.vm.os {
        config = config.os(os);
    }
    if let Some(architecture) = settings.vm.architecture {
        config = config.architecture(architecture);
    }
    if let Some(path) = settings.vm.registry_path.as_ref() {
        let registry = load_registry(path)?;
        config = config.properties(registry);
    }
    let mut paths = if config.paths_ref().is_empty() {
        default_path_mapping()
    } else {
        config.paths_ref().clone()
    };
    apply_settings_paths(&mut paths, settings);
    if !paths.is_empty() {
        config = config.paths(paths);
    }
    if let Some(sandbox) = settings.sandbox.as_ref() {
        config = config.sandbox(sandbox.clone());
    }
    config = config.bypass(settings.bypass.clone());
    Ok(config)
}

pub(crate) fn apply_settings_paths(paths: &mut PathMapping, settings: &Settings) {
    merge_paths(paths, &settings.pe.paths);
    merge_paths(paths, &settings.vm.paths);
}

pub(crate) fn default_path_mapping() -> PathMapping {
    let mut paths = PathMapping::new();
    if let Some(home) = home_dir() {
        let root = home.join(".pe-vm");
        let _ = std::fs::create_dir_all(&root);
        let data_dir = root.join("data");
        let cache_dir = root.join("cache");
        let _ = std::fs::create_dir_all(&data_dir);
        let _ = std::fs::create_dir_all(&cache_dir);
        let lock_path = data_dir.join("lock.dat");
        if !lock_path.exists() {
            let _ = std::fs::write(&lock_path, b"");
        }
        let host_path = root.to_string_lossy().to_string();
        if !host_path.is_empty() {
            paths.insert("C:\\".to_string(), host_path);
        }
    }
    paths
}

impl Settings {
    fn from_raw(raw: SettingsRaw) -> Result<Self, VmError> {
        let mut settings = Settings::default();
        if let Some(vm) = raw.vm {
            if let Some(properties) = vm.properties {
                if let Some(os) = properties.os {
                    settings.vm.os = Some(parse_os(&os)?);
                }
                if let Some(architecture) = properties.architecture {
                    settings.vm.architecture = Some(parse_architecture(&architecture)?);
                }
                settings.vm.registry_path = properties.path;
            }
            if let Some(paths) = vm.paths {
                settings.vm.paths = parse_paths(&paths);
            }
        }
        if let Some(pe) = raw.pe {
            if let Some(paths) = pe.paths {
                settings.pe.paths = parse_paths(&paths);
            }
        }
        if let Some(sandbox) = raw.sandbox {
            settings.sandbox = parse_sandbox(&sandbox);
        }
        if let Some(bypass) = raw.bypass {
            if let Some(v) = bypass.not_implemented_module {
                settings.bypass.not_implemented_module = v;
            }
        }
        Ok(settings)
    }

    #[allow(dead_code)]
    pub(crate) fn bypass(&self) -> &BypassSettings {
        &self.bypass
    }

    fn merge_from(&mut self, other: Settings) {
        if other.vm.os.is_some() {
            self.vm.os = other.vm.os;
        }
        if other.vm.architecture.is_some() {
            self.vm.architecture = other.vm.architecture;
        }
        if other.vm.registry_path.is_some() {
            self.vm.registry_path = other.vm.registry_path;
        }
        merge_paths(&mut self.vm.paths, &other.vm.paths);
        merge_paths(&mut self.pe.paths, &other.pe.paths);
        if other.sandbox.is_some() {
            self.sandbox = other.sandbox;
        }
        if other.bypass.not_implemented_module {
            self.bypass.not_implemented_module = true;
        }
    }

    fn is_empty(&self) -> bool {
        self.vm.os.is_none()
            && self.vm.architecture.is_none()
            && self.vm.registry_path.is_none()
            && self.vm.paths.is_empty()
            && self.pe.paths.is_empty()
            && self.sandbox.is_none()
    }
}

fn parse_os(value: &str) -> Result<Os, VmError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "windows" | "win" => Ok(Os::Windows),
        "unix" | "linux" => Ok(Os::Unix),
        "mac" | "macos" | "osx" => Ok(Os::Mac),
        _ => Err(VmError::InvalidConfig("unknown vm.os value")),
    }
}

fn parse_architecture(value: &str) -> Result<Architecture, VmError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "x86" | "i386" | "ia32" => Ok(Architecture::X86),
        "x86_64" | "amd64" | "x64" => Ok(Architecture::X86_64),
        _ => Err(VmError::InvalidConfig("unknown vm.architecture value")),
    }
}

fn parse_paths(value: &YamlValue) -> PathMapping {
    let mut out = PathMapping::new();
    match value {
        YamlValue::Mapping(map) => {
            merge_yaml_map(&mut out, map);
        }
        YamlValue::Sequence(seq) => {
            for item in seq {
                if let YamlValue::Mapping(map) = item {
                    merge_yaml_map(&mut out, map);
                }
            }
        }
        _ => {}
    }
    out
}

fn merge_yaml_map(out: &mut PathMapping, map: &serde_yaml::Mapping) {
    for (key, value) in map {
        let Some(key) = key.as_str() else { continue };
        let Some(value) = value.as_str() else {
            continue;
        };
        out.insert(key.to_string(), value.to_string());
    }
}

fn parse_sandbox(value: &YamlValue) -> Option<SandboxConfig> {
    match value {
        YamlValue::Mapping(map) => parse_sandbox_map(map),
        YamlValue::Sequence(seq) => {
            for item in seq {
                match item {
                    YamlValue::String(name) if name.eq_ignore_ascii_case("network") => {
                        return Some(SandboxConfig::new().enable_network(""));
                    }
                    YamlValue::Mapping(map) => {
                        if let Some(config) = parse_sandbox_map(map) {
                            return Some(config);
                        }
                    }
                    _ => {}
                }
            }
            None
        }
        _ => None,
    }
}

fn parse_sandbox_map(map: &serde_yaml::Mapping) -> Option<SandboxConfig> {
    for (key, value) in map {
        let Some(name) = key.as_str() else { continue };
        if !name.eq_ignore_ascii_case("network") {
            continue;
        }
        return Some(parse_network(value));
    }
    None
}

fn parse_network(value: &YamlValue) -> SandboxConfig {
    match value {
        YamlValue::Mapping(map) => {
            let fallback = map
                .get(YamlValue::String("fallback_host".to_string()))
                .and_then(|value| value.as_str())
                .unwrap_or("");
            SandboxConfig::new().enable_network(fallback)
        }
        YamlValue::String(host) => SandboxConfig::new().enable_network(host),
        _ => SandboxConfig::new().enable_network(""),
    }
}

fn load_registry(path: &str) -> Result<windows::registry::Registry, VmError> {
    let ext = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    if ext.eq_ignore_ascii_case("reg") {
        return windows::registry::load_from_registry(path)
            .map_err(|_| VmError::InvalidConfig("registry load failed"));
    }
    if ext.eq_ignore_ascii_case("yml") || ext.eq_ignore_ascii_case("yaml") {
        return windows::registry::load_from_yml(path)
            .map_err(|_| VmError::InvalidConfig("registry load failed"));
    }
    Err(VmError::InvalidConfig("unsupported registry format"))
}

fn merge_paths(base: &mut PathMapping, extra: &PathMapping) {
    for (key, value) in extra {
        base.insert(key.clone(), value.clone());
    }
}

fn parse_settings_contents(contents: &str) -> Result<Settings, VmError> {
    if contents.trim().is_empty() {
        return Ok(Settings::default());
    }
    let raw: SettingsRaw = serde_yaml::from_str(contents)
        .map_err(|_| VmError::InvalidConfig("settings parse failed"))?;
    Settings::from_raw(raw)
}

fn load_settings_file(path: &Path) -> Result<Option<Settings>, VmError> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(VmError::Io(err)),
    };
    let settings = parse_settings_contents(&contents)?;
    Ok(Some(settings))
}

fn settings_search_candidates() -> Vec<SettingsCandidate> {
    let dirs = settings_search_dirs();
    let mut candidates = Vec::new();
    for dir in dirs {
        for file in SETTINGS_FILES {
            candidates.push(SettingsCandidate {
                path: dir.path.join(file.name),
                dir_priority: dir.priority,
                file_priority: file.priority,
            });
        }
    }
    candidates.sort_by(|left, right| {
        let order = left
            .dir_priority
            .cmp(&right.dir_priority)
            .then_with(|| left.file_priority.cmp(&right.file_priority));
        if order.is_eq() {
            left.path
                .to_string_lossy()
                .cmp(&right.path.to_string_lossy())
        } else {
            order
        }
    });
    candidates
}

fn settings_search_dirs() -> Vec<SettingsDir> {
    if let Some(dirs) = settings_env_dirs() {
        return dirs;
    }
    let overrides = settings_dir_priority_overrides();
    let mut dirs = Vec::new();
    for spec in DEFAULT_SETTINGS_DIRS {
        let priority = overrides.get(&spec.kind).copied().unwrap_or(spec.priority);
        if let Some(path) = resolve_settings_dir(spec.kind) {
            push_unique_dir(&mut dirs, path, priority);
        }
    }
    dirs
}

fn settings_env_dirs() -> Option<Vec<SettingsDir>> {
    let value = std::env::var_os(SETTINGS_ENV_DIR)?;
    let mut dirs = Vec::new();
    let mut priority = 1u32;
    for entry in std::env::split_paths(&value) {
        push_unique_dir(&mut dirs, entry, priority);
        priority = priority.saturating_add(1);
    }
    if dirs.is_empty() {
        None
    } else {
        Some(dirs)
    }
}

fn settings_dir_priority_overrides() -> HashMap<SettingsDirKind, u32> {
    let Ok(value) = std::env::var(SETTINGS_ENV_PRIORITY) else {
        return HashMap::new();
    };
    let mut overrides = HashMap::new();
    let mut order_priority = 1u32;
    for token in value.split(|ch: char| ch == ',' || ch.is_whitespace()) {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        if let Some((name, priority)) = token.split_once('=') {
            if let Some(kind) = parse_settings_dir_kind(name) {
                if let Ok(priority) = priority.trim().parse::<u32>() {
                    overrides.insert(kind, priority);
                    continue;
                }
            }
        }
        if let Some(kind) = parse_settings_dir_kind(token) {
            overrides.entry(kind).or_insert_with(|| {
                let value = order_priority;
                order_priority = order_priority.saturating_add(1);
                value
            });
        }
    }
    overrides
}

fn parse_settings_dir_kind(token: &str) -> Option<SettingsDirKind> {
    match token.trim().to_ascii_lowercase().as_str() {
        "cwd" | "current" | "current_dir" | "." => Some(SettingsDirKind::Cwd),
        "home" | "config" | "settings" | "~" | "~/.pe-vm" => Some(SettingsDirKind::HomePeVm),
        "root" | "/" => Some(SettingsDirKind::Root),
        _ => None,
    }
}

fn resolve_settings_dir(kind: SettingsDirKind) -> Option<PathBuf> {
    match kind {
        SettingsDirKind::Root => Some(PathBuf::from("/")),
        SettingsDirKind::HomePeVm => home_dir().map(|home| home.join(".pe-vm")),
        SettingsDirKind::Cwd => std::env::current_dir().ok(),
    }
}

fn push_unique_dir(dirs: &mut Vec<SettingsDir>, path: PathBuf, priority: u32) {
    if let Some(existing) = dirs.iter_mut().find(|entry| entry.path == path) {
        if priority > existing.priority {
            existing.priority = priority;
        }
        return;
    }
    dirs.push(SettingsDir { path, priority });
}

fn home_dir() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("HOME") {
        return Some(PathBuf::from(path));
    }
    if let Some(path) = std::env::var_os("USERPROFILE") {
        return Some(PathBuf::from(path));
    }
    let drive = std::env::var_os("HOMEDRIVE")?;
    let path = std::env::var_os("HOMEPATH")?;
    let mut buf = PathBuf::from(drive);
    buf.push(path);
    Some(buf)
}
