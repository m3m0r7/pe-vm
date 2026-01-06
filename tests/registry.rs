// Tests registry parsing, mutation, and YAML merge behavior.
use pe_vm::windows::registry::{
    load_from_registry, Registry, RegistryHive, RegistryKey, RegistryMergeMode, RegistryValue,
};

// Verify that default value keys parse without a value name.
#[test]
fn parse_registry_key_default_value() {
    let key = RegistryKey::parse(r"HKCR\CLSID\{GUID}\InprocServer32").expect("parse");
    assert_eq!(key.hive, RegistryHive::ClassesRoot);
    assert_eq!(
        key.path,
        vec!["CLSID", "{GUID}", "InprocServer32"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
    assert!(key.value_name.is_none());
}

// Verify that @value-name notation parses as a named value.
#[test]
fn parse_registry_key_named_value() {
    let key =
        RegistryKey::parse(r"HKCR\CLSID\{GUID}\InprocServer32@ThreadingModel").expect("parse");
    assert_eq!(key.hive, RegistryHive::ClassesRoot);
    assert_eq!(
        key.path,
        vec!["CLSID", "{GUID}", "InprocServer32"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
    assert_eq!(key.value_name.as_deref(), Some("ThreadingModel"));
}

// Ensure append does not overwrite, while set does.
#[test]
fn append_and_set_values() {
    let mut registry = Registry::with_defaults();
    registry
        .set(
            r"HKCR\CLSID\{GUID}\InprocServer32",
            RegistryValue::String("a.dll".to_string()),
        )
        .expect("set");
    let appended = registry
        .append(
            r"HKCR\CLSID\{GUID}\InprocServer32",
            RegistryValue::String("b.dll".to_string()),
        )
        .expect("append");
    assert!(!appended);

    registry
        .set(
            r"HKCR\CLSID\{GUID}\InprocServer32",
            RegistryValue::String("c.dll".to_string()),
        )
        .expect("set");
    let value = registry
        .get(r"HKCR\CLSID\{GUID}\InprocServer32")
        .expect("get")
        .expect("value");
    assert_eq!(value, &RegistryValue::String("c.dll".to_string()));
}

// Ensure YAML merges map to default and named values under a key.
#[test]
fn merge_yaml_values() {
    let yaml = r#"
HKCR:
  CLSID:
    "{TEST}":
      InprocServer32:
        "(Default)": "C:\\sample.dll"
        ThreadingModel: "Apartment"
"#;
    let mut registry = Registry::with_defaults();
    registry
        .merge_yaml_str(yaml, RegistryMergeMode::Overwrite)
        .expect("merge");

    let default_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32")
        .expect("get")
        .expect("default");
    assert_eq!(
        default_value,
        &RegistryValue::String("C:\\sample.dll".to_string())
    );

    let threading_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@ThreadingModel")
        .expect("get")
        .expect("threading");
    assert_eq!(
        threading_value,
        &RegistryValue::String("Apartment".to_string())
    );
}

// Ensure .reg files load common value types into the registry.
#[test]
fn load_reg_file_values() {
    let reg = r#"
Windows Registry Editor Version 5.00

[HKEY_CLASSES_ROOT\CLSID\{TEST}\InprocServer32]
@="C:\sample.dll"
"ThreadingModel"="Apartment"
"Flags"=dword:00000002
"Binary"=hex:01,02,0a
"Expand"=hex(2):43,00,3a,00,5c,00,46,00,6f,00,6f,00,00,00
"Multi"=hex(7):41,00,00,00,42,00,00,00,00,00
"#;
    let path = std::env::temp_dir().join("pe_vm_test_registry.reg");
    std::fs::write(&path, reg).expect("write");

    let registry = load_from_registry(&path).expect("load");
    let default_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32")
        .expect("get")
        .expect("default");
    assert_eq!(
        default_value,
        &RegistryValue::String(r"C:\sample.dll".to_string())
    );
    let threading_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@ThreadingModel")
        .expect("get")
        .expect("threading");
    assert_eq!(
        threading_value,
        &RegistryValue::String("Apartment".to_string())
    );
    let flags_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@Flags")
        .expect("get")
        .expect("flags");
    assert_eq!(flags_value, &RegistryValue::Dword(2));
    let binary_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@Binary")
        .expect("get")
        .expect("binary");
    assert_eq!(binary_value, &RegistryValue::Binary(vec![1, 2, 10]));
    let expand_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@Expand")
        .expect("get")
        .expect("expand");
    assert_eq!(
        expand_value,
        &RegistryValue::String(r"C:\Foo".to_string())
    );
    let multi_value = registry
        .get(r"HKCR\CLSID\{TEST}\InprocServer32@Multi")
        .expect("get")
        .expect("multi");
    assert_eq!(
        multi_value,
        &RegistryValue::MultiString(vec!["A".to_string(), "B".to_string()])
    );

    let _ = std::fs::remove_file(path);
}

// Ensure subkey enumeration and stats reflect stored registry data.
#[test]
fn registry_subkeys_and_stats() {
    let mut registry = Registry::with_defaults();
    registry
        .set(
            r"HKLM\Software\ExampleVendor\App\Settings@endpoint",
            RegistryValue::String("https://example.invalid/api".to_string()),
        )
        .expect("set endpoint");
    registry
        .set(
            r"HKLM\Software\ExampleVendor\App\State@retry_count",
            RegistryValue::Dword(2),
        )
        .expect("set retry_count");

    let subkeys = registry
        .subkeys(r"HKLM\Software\ExampleVendor\App")
        .expect("subkeys");
    assert!(subkeys.contains(&"SETTINGS".to_string()));
    assert!(subkeys.contains(&"STATE".to_string()));

    let root_stats = registry
        .stats(r"HKLM\Software\ExampleVendor\App", false)
        .expect("stats root");
    assert_eq!(root_stats.subkey_count, 2);
    assert_eq!(root_stats.value_count, 0);

    let info_stats = registry
        .stats(r"HKLM\Software\ExampleVendor\App\Settings", false)
        .expect("stats settings");
    assert_eq!(info_stats.value_count, 1);
    assert_eq!(info_stats.max_value_name_len, "ENDPOINT".len() as u32);
    assert!(info_stats.max_value_len >= "https://example.invalid/api".len() as u32 + 1);
}
