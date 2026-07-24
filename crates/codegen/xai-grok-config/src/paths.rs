//! Filesystem locations for grok config files and binaries.

use std::path::PathBuf;
use std::sync::OnceLock;

static GROK_HOME: OnceLock<PathBuf> = OnceLock::new();
static ARCH_HOME: OnceLock<PathBuf> = OnceLock::new();

#[cfg(target_os = "macos")]
const CLAUDE_MANAGED_SETTINGS_PATH: &str =
    "/Library/Application Support/ClaudeCode/managed-settings.json";
#[cfg(target_os = "linux")]
const CLAUDE_MANAGED_SETTINGS_PATH: &str = "/etc/claude-code/managed-settings.json";

/// The default user grok directory (`~/.grok`, canonicalized) used when
/// `GROK_HOME` is unset. Exposed so callers (e.g. display helpers) can detect
/// whether [`grok_home()`] is the default without duplicating the computation.
///
/// Uses [`dunce::canonicalize`] instead of [`std::fs::canonicalize`]: on
/// Windows, std returns a verbatim path (`\\?\C:\Users\...`) which external
/// tools choke on — e.g. `git clone` rejects `\\?\` destinations with
/// "Invalid argument", breaking marketplace cache clones under
/// `~/.grok/marketplace-cache`. `dunce` strips the prefix whenever the path
/// is safely representable in legacy form; on non-Windows it is identical to
/// `std::fs::canonicalize`.
///
/// Keep the dunce canonicalization in sync with the hand-rolled duplicate in
/// `xai_fast_worktree::db::resolve_grok_home` (deliberately standalone crate).
pub fn default_grok_home() -> PathBuf {
    #[allow(deprecated)]
    let home = std::env::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dunce::canonicalize(&home).unwrap_or(home).join(".grok")
}

/// Per-user config directory: `$GROK_HOME` or `~/.grok`. Created if needed.
pub fn grok_home() -> PathBuf {
    GROK_HOME
        .get_or_init(|| {
            let grok_home = if let Ok(v) = std::env::var("GROK_HOME") {
                PathBuf::from(v)
            } else {
                default_grok_home()
            };
            let _ = std::fs::create_dir_all(&grok_home);
            grok_home
        })
        .clone()
}

/// The user-global grok home, but only when one genuinely resolves: `Some` when
/// `$GROK_HOME` is set or a home directory is found, `None` otherwise. Unlike
/// [`grok_home()`], this never falls back to a cwd-relative `.grok`, so callers
/// that *scan* user-global grok resources (hooks, marketplace sources, ...) don't
/// mistake a project's `.grok` tree for the user-global one when no home resolves.
pub fn user_grok_home() -> Option<PathBuf> {
    #[allow(deprecated)]
    let resolvable = std::env::var_os("GROK_HOME").is_some() || std::env::home_dir().is_some();
    resolvable.then(grok_home)
}

/// Default Arch user directory (`~/.arch`, canonicalized) when `ARCH_HOME` is unset.
pub fn default_arch_home() -> PathBuf {
    #[allow(deprecated)]
    let home = std::env::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dunce::canonicalize(&home).unwrap_or(home).join(".arch")
}

/// Per-user Arch config directory: `$ARCH_HOME` or `~/.arch`. Created if needed.
///
/// Runtime data still may live under [`grok_home`] during the thin-fork migration
/// (`~/.grok` compatibility). Prefer this for new Arch-native surfaces (souls,
/// agent memory, project overlays under `.arch`).
pub fn arch_home() -> PathBuf {
    ARCH_HOME
        .get_or_init(|| {
            let arch_home = if let Ok(v) = std::env::var("ARCH_HOME") {
                PathBuf::from(v)
            } else {
                default_arch_home()
            };
            let _ = std::fs::create_dir_all(&arch_home);
            arch_home
        })
        .clone()
}

/// User-global Arch home when one genuinely resolves (`Some` when `$ARCH_HOME`
/// is set or a home directory is found). Never falls back to a cwd-relative
/// `.arch`.
pub fn user_arch_home() -> Option<PathBuf> {
    #[allow(deprecated)]
    let resolvable = std::env::var_os("ARCH_HOME").is_some() || std::env::home_dir().is_some();
    resolvable.then(arch_home)
}

/// Product primary home for Arch-owned secrets and new local state.
///
/// Currently `arch_home()` — auth credentials write here by default.
pub fn product_home() -> PathBuf {
    arch_home()
}

/// Primary `auth.json` path for Arch.
///
/// Resolution order:
/// 1. `$ARCH_AUTH_PATH` or `$GROK_AUTH_PATH` (explicit file override)
/// 2. `$ARCH_HOME/auth.json` / `~/.arch/auth.json` (product default)
///
/// Callers that pass an explicit non-default home (tests) should join
/// `auth.json` themselves; see [`resolve_auth_json_path`].
pub fn auth_json_primary_path() -> PathBuf {
    if let Ok(p) = std::env::var("ARCH_AUTH_PATH") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("GROK_AUTH_PATH") {
        return PathBuf::from(p);
    }
    product_home().join("auth.json")
}

/// Legacy Grok auth path (`~/.grok/auth.json`) for **read-only** fallback.
///
/// `$ARCH_AUTH_LEGACY_PATH` overrides (tests / emergency read source).
pub fn auth_json_legacy_path() -> PathBuf {
    if let Ok(p) = std::env::var("ARCH_AUTH_LEGACY_PATH") {
        return PathBuf::from(p);
    }
    default_grok_home().join("auth.json")
}

/// Resolve the auth.json path for an AuthManager home directory.
///
/// - Explicit `$ARCH_AUTH_PATH` / `$GROK_AUTH_PATH` always wins.
/// - When `home` is the default user grok home, Arch remaps writes to
///   [`auth_json_primary_path`] (`~/.arch/auth.json`).
/// - Otherwise (tests / custom `GROK_HOME`) keep `{home}/auth.json`.
pub fn resolve_auth_json_path(home: &std::path::Path) -> PathBuf {
    if let Ok(p) = std::env::var("ARCH_AUTH_PATH") {
        return PathBuf::from(p);
    }
    if let Ok(p) = std::env::var("GROK_AUTH_PATH") {
        return PathBuf::from(p);
    }
    let default_g = default_grok_home();
    if paths_equal_lex(home, &default_g) {
        auth_json_primary_path()
    } else {
        home.join("auth.json")
    }
}

fn paths_equal_lex(a: &std::path::Path, b: &std::path::Path) -> bool {
    if a == b {
        return true;
    }
    // Compare canonical when possible so `/Users/x/.grok` vs symlink match.
    match (dunce::canonicalize(a), dunce::canonicalize(b)) {
        (Ok(ca), Ok(cb)) => ca == cb,
        _ => false,
    }
}

/// Canonical grok application path: `$GROK_HOME/bin/grok` (Unix) or `grok.exe` (Windows).
pub fn grok_application() -> PathBuf {
    grok_application_in(&grok_home())
}

/// [`grok_application`] under an explicit home instead of `$GROK_HOME`.
pub fn grok_application_in(home: &std::path::Path) -> PathBuf {
    let name = if cfg!(windows) { "grok.exe" } else { "grok" };
    home.join("bin").join(name)
}

/// System-wide config directory: `/etc/grok/` on Unix, `None` on Windows.
pub fn system_config_dir() -> Option<PathBuf> {
    if cfg!(unix) {
        Some(PathBuf::from("/etc/grok"))
    } else {
        None
    }
}

/// System path for the managed-settings.json used for settings compat, if it exists.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn claude_managed_settings_path() -> Option<PathBuf> {
    let path = PathBuf::from(CLAUDE_MANAGED_SETTINGS_PATH);
    path.exists().then_some(path)
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn claude_managed_settings_path() -> Option<PathBuf> {
    None
}

/// The platform path where managed-settings.json would live for settings
/// compat, whether or not it exists. `None` on unsupported platforms.
#[cfg(any(target_os = "macos", target_os = "linux"))]
pub fn claude_managed_settings_probe_path() -> Option<PathBuf> {
    Some(PathBuf::from(CLAUDE_MANAGED_SETTINGS_PATH))
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn claude_managed_settings_probe_path() -> Option<PathBuf> {
    None
}

/// Max bytes for a single directory name component (macOS APFS, Linux ext4,
/// NTFS all enforce 255 bytes).
const MAX_DIRNAME_BYTES: usize = 255;

/// Encode a CWD string into a filesystem-safe directory name component.
///
/// Short CWDs (URL-encoded form <= 255 bytes) use URL-encoding for backward
/// compatibility and human readability on disk.
///
/// Long CWDs (> 255 bytes encoded) use a compact `{slug}-{blake3_hex16}`
/// form that is always <= 57 bytes. Callers must write a `.cwd` metadata
/// file via [`ensure_sessions_cwd_dir`] so the original CWD can be
/// recovered by [`decode_cwd_from_dirname`].
pub fn encode_cwd_dirname(cwd: &str) -> String {
    let url_encoded = urlencoding::encode(cwd);
    if url_encoded.len() <= MAX_DIRNAME_BYTES {
        return url_encoded.into_owned();
    }
    let hash = blake3::hash(cwd.as_bytes());
    let hash16 = &hash.to_hex()[..16];
    let leaf = std::path::Path::new(cwd)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("workspace");
    let slug = slugify(leaf, 40);
    let slug = if slug.is_empty() { "workspace" } else { &slug };
    format!("{slug}-{hash16}")
}

/// Recover the original CWD from a sessions CWD directory.
///
/// Tries URL-decoding the directory name first (works for short/legacy dirs).
/// Falls back to reading a `.cwd` metadata file inside the directory (written
/// by [`ensure_sessions_cwd_dir`] for hash-based dirs).
pub fn decode_cwd_from_dirname(dir: &std::path::Path) -> Option<String> {
    let name = dir.file_name()?.to_str()?;
    if let Ok(decoded) = urlencoding::decode(name) {
        let s = decoded.into_owned();
        // URL-decoded absolute CWDs always start with `/` (Unix) or a drive
        // letter (Windows).  The slug-hash form never does, so this
        // distinguishes the two encodings unambiguously.
        if s.starts_with('/') || (cfg!(windows) && s.chars().nth(1) == Some(':')) {
            return Some(s);
        }
    }
    std::fs::read_to_string(dir.join(".cwd"))
        .ok()
        .map(|s| s.trim().to_string())
}

/// Build the CWD-level session directory path:
/// `grok_home()/sessions/{encode_cwd_dirname(cwd)}`.
///
/// Does **not** create the directory on disk — use [`ensure_sessions_cwd_dir`]
/// when the directory must exist.
pub fn sessions_cwd_dir(cwd: &str) -> PathBuf {
    grok_home().join("sessions").join(encode_cwd_dirname(cwd))
}

/// Create the CWD-level session directory and write a `.cwd` metadata file
/// when hash-based encoding is used (long paths).
///
/// For short paths the `.cwd` file is not written because the directory name
/// itself is reversible via URL-decoding.
pub fn ensure_sessions_cwd_dir(cwd: &str) -> std::io::Result<PathBuf> {
    let encoded_name = encode_cwd_dirname(cwd);
    let dir = grok_home().join("sessions").join(&encoded_name);
    std::fs::create_dir_all(&dir)?;
    // Hash-based encoding is in use when the dirname differs from the
    // plain URL-encoded form.  Write a `.cwd` file so decode can recover
    // the original path.  O_CREAT|O_EXCL via create_new avoids TOCTOU
    // races with parallel session starts.
    if encoded_name != urlencoding::encode(cwd).as_ref() {
        let cwd_file = dir.join(".cwd");
        match std::fs::File::create_new(&cwd_file) {
            Ok(mut f) => {
                std::io::Write::write_all(&mut f, cwd.as_bytes())?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Ok(dir)
}

/// Generate a URL-safe slug from a string.
///
/// Lowercases, replaces non-alphanumeric chars with `-`, collapses
/// consecutive dashes, and truncates to `max_len` characters.
fn slugify(input: &str, max_len: usize) -> String {
    let mut result = String::with_capacity(input.len());
    let mut prev_dash = false;
    for c in input.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            result.push(c);
            prev_dash = false;
        } else if !prev_dash {
            result.push('-');
            prev_dash = true;
        }
    }
    let trimmed = result.trim_matches('-');
    trimmed.chars().take(max_len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Realistic CWDs that trigger the bug (URL-encoded > 255 bytes).
    const LONG_CWDS: &[&str] = &[
        "/Users/dev/Documents/開発プロジェクト/機能追加/テスト環境/ソースコード/main-branch",
        "/Users/user/Library/Mobile Documents/com~apple~CloudDocs/项目文件/深层嵌套目录/更深层次的/工作区域/project",
        "/Users/user/Library/CloudStorage/OneDrive-대한민국회사/프로젝트/개발환경/소스코드/백엔드/서비스/my-app",
        "/Users/user/Documents/工作文件夹/二零二六年项目/子目录一/子目录二/子目录三/源代码/code",
    ];

    #[test]
    fn long_cwd_uses_hash_fallback_within_name_max() {
        let long_cwd = format!("/Users/test/{}", "中".repeat(30));
        let encoded = encode_cwd_dirname(&long_cwd);
        assert!(encoded.len() <= MAX_DIRNAME_BYTES);
        assert!(!encoded.starts_with("%2F"));
    }

    #[test]
    fn different_long_paths_produce_different_hashes() {
        let a = format!("/Users/test/{}", "中".repeat(30));
        let b = format!("/Users/test/{}", "日".repeat(30));
        assert_ne!(encode_cwd_dirname(&a), encode_cwd_dirname(&b));
    }

    #[test]
    fn decode_reads_cwd_file_for_hash_dirs() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("some-slug-abcdef0123456789");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(".cwd"), "/original/long/path").unwrap();
        assert_eq!(
            decode_cwd_from_dirname(&dir),
            Some("/original/long/path".to_string())
        );
    }

    #[test]
    fn decode_returns_none_without_cwd_file() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("some-slug-abcdef0123456789");
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(decode_cwd_from_dirname(&dir), None);
    }

    #[test]
    fn cwd_file_write_is_idempotent_via_excl() {
        let tmp = TempDir::new().unwrap();
        let long_cwd = format!("/Users/test/{}", "中".repeat(30));
        let dir = tmp.path().join(encode_cwd_dirname(&long_cwd));
        std::fs::create_dir_all(&dir).unwrap();
        let cwd_file = dir.join(".cwd");
        std::fs::write(&cwd_file, &long_cwd).unwrap();
        match std::fs::File::create_new(&cwd_file) {
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            other => panic!("expected AlreadyExists, got: {other:?}"),
        }
        assert_eq!(std::fs::read_to_string(&cwd_file).unwrap(), long_cwd);
    }

    #[test]
    fn url_encoded_long_cwd_fails_on_real_filesystem() {
        let tmp = TempDir::new().unwrap();
        let url_encoded = urlencoding::encode(LONG_CWDS[0]).into_owned();
        let result = std::fs::create_dir_all(tmp.path().join(&url_encoded));
        assert!(result.is_err());
    }

    #[test]
    fn full_roundtrip_on_real_filesystem_for_long_cwds() {
        let tmp = TempDir::new().unwrap();
        for cwd in LONG_CWDS {
            let encoded = encode_cwd_dirname(cwd);
            let dir = tmp.path().join(&encoded);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(dir.join(".cwd"), cwd).unwrap();
            assert_eq!(decode_cwd_from_dirname(&dir).as_deref(), Some(*cwd));
        }
    }

    #[test]
    fn short_cwds_use_url_encoding_and_roundtrip_on_real_filesystem() {
        let tmp = TempDir::new().unwrap();
        for cwd in [
            "/Users/foo/project",
            "/tmp",
            "/Users/user/Documents/project-名前",
        ] {
            let encoded = encode_cwd_dirname(cwd);
            assert_eq!(encoded, urlencoding::encode(cwd).into_owned());
            let dir = tmp.path().join(&encoded);
            std::fs::create_dir_all(&dir).unwrap();
            assert_eq!(decode_cwd_from_dirname(&dir).as_deref(), Some(cwd));
        }
    }

    #[test]
    fn default_grok_home_has_no_verbatim_prefix() {
        // On Windows, std::fs::canonicalize returns `\\?\C:\...` verbatim
        // paths that external tools (notably `git clone`) reject. The dunce
        // canonicalization must yield a plain path. No-op assertion on Unix.
        let home = default_grok_home();
        assert!(!home.to_string_lossy().starts_with(r"\\?\"));
        assert!(home.ends_with(".grok"));
    }

    #[test]
    fn default_arch_home_ends_with_dot_arch() {
        let home = default_arch_home();
        assert!(!home.to_string_lossy().starts_with(r"\\?\"));
        assert!(home.ends_with(".arch"));
    }

    #[test]
    fn auth_primary_under_arch_when_no_env_override() {
        // Shape only — absolute HOME / ARCH_HOME may vary in CI.
        let p = auth_json_primary_path();
        assert!(
            p.ends_with("auth.json"),
            "primary auth path must end with auth.json: {p:?}"
        );
        let legacy = auth_json_legacy_path();
        assert!(legacy.ends_with("auth.json"));
        // Product primary is never the same path as legacy when defaults apply.
        if std::env::var_os("ARCH_AUTH_PATH").is_none()
            && std::env::var_os("GROK_AUTH_PATH").is_none()
        {
            assert_ne!(p, legacy, "Arch primary and Grok legacy auth paths must differ");
        }
    }

    #[test]
    fn resolve_auth_json_custom_home_stays_local() {
        let custom = PathBuf::from("/tmp/arch-test-home-xyz");
        let p = resolve_auth_json_path(&custom);
        assert_eq!(p, custom.join("auth.json"));
    }

    #[test]
    fn auth_write_simulation_under_custom_home_not_legacy() {
        let home = tempfile::TempDir::new().expect("tempdir");
        let path = resolve_auth_json_path(home.path());
        assert_eq!(path, home.path().join("auth.json"));
        std::fs::write(&path, "{}").expect("write auth");
        assert!(path.exists());
        assert!(
            !home.path().join(".grok").exists(),
            "Arch auth must not create ~/.grok under a custom home"
        );
    }

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("Hello World!", 40), "hello-world");
    }

    #[test]
    fn slugify_cjk_produces_empty() {
        assert_eq!(slugify("深层目录", 40), "");
    }

    #[test]
    fn slugify_truncates() {
        assert_eq!(slugify(&"a".repeat(100), 10).len(), 10);
    }
}
