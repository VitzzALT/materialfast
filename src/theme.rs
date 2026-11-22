use eframe::egui::Color32;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};

#[derive(Clone, Copy)]
pub struct Palette {
    pub base: Color32,
    pub panel: Color32,
    pub raised: Color32,
    pub floating: Color32,
    pub text: Color32,
    pub muted: Color32,
    pub faint: Color32,
    pub accent: Color32,
    pub on_accent: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

impl Palette {
    pub const ORIGINAL: Palette = Palette {
        base: rgb(0x0E0F12),
        panel: rgb(0x0A0B0D),
        raised: rgb(0x1A1B20),
        floating: rgb(0x1B1C21),
        text: rgb(0xECEDEF),
        muted: Color32::from_rgba_premultiplied(166, 166, 166, 166),
        faint: Color32::from_rgba_premultiplied(122, 122, 122, 122),
        accent: rgb(0xF4623F),
        on_accent: rgb(0x1A0A05),
    };
}

static CURRENT: RwLock<Palette> = RwLock::new(Palette::ORIGINAL);

#[derive(Deserialize)]
struct Scheme {
    surface: String,
    #[serde(rename = "surfaceContainerLowest")]
    surface_container_lowest: String,
    #[serde(rename = "surfaceContainerHigh")]
    surface_container_high: String,
    #[serde(rename = "surfaceContainerHighest")]
    surface_container_highest: String,
    #[serde(rename = "onSurface")]
    on_surface: String,
    #[serde(rename = "onSurfaceVariant")]
    on_surface_variant: String,
    outline: String,
    primary: String,
    #[serde(rename = "onPrimary")]
    on_primary: String,
}

#[derive(Deserialize)]
struct HyprYouColors {
    dark: Scheme,
    light: Scheme,
    is_dark: bool,
}

fn hex(value: &str) -> Option<Color32> {
    let digits = value.trim().trim_start_matches('#');
    if digits.len() != 6 {
        return None;
    }
    u32::from_str_radix(digits, 16).ok().map(rgb)
}

fn hex_alpha(value: &str, alpha: u8) -> Option<Color32> {
    let digits = value.trim().trim_start_matches('#');
    if digits.len() != 6 {
        return None;
    }
    let hex = u32::from_str_radix(digits, 16).ok()?;
    Some(Color32::from_rgba_unmultiplied(
        (hex >> 16) as u8,
        (hex >> 8) as u8,
        hex as u8,
        alpha,
    ))
}

fn parse(text: &str) -> Option<Palette> {
    let file: HyprYouColors = serde_json::from_str(text).ok()?;
    let scheme = if file.is_dark { &file.dark } else { &file.light };
    Some(Palette {
        base: hex(&scheme.surface)?,
        panel: hex(&scheme.surface_container_lowest)?,
        raised: hex(&scheme.surface_container_high)?,
        floating: hex(&scheme.surface_container_highest)?,
        text: hex(&scheme.on_surface)?,
        muted: hex_alpha(&scheme.on_surface_variant, 200)?,
        faint: hex_alpha(&scheme.outline, 140)?,
        accent: hex(&scheme.primary)?,
        on_accent: hex(&scheme.on_primary)?,
    })
}

fn path() -> Option<PathBuf> {
    if let Some(custom) = std::env::var_os("MATERIALFAST_PALETTE") {
        return Some(PathBuf::from(custom));
    }
    let root = match std::env::var_os("XDG_CACHE_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME")?).join(".cache"),
    };
    Some(root.join("hypryou").join("colors").join("colors.json"))
}

pub fn load() {
    let palette = path()
        .and_then(|file| std::fs::read_to_string(file).ok())
        .and_then(|text| parse(&text));
    if let Some(palette) = palette
        && let Ok(mut current) = CURRENT.write()
    {
        *current = palette;
    }
}

pub fn get() -> Palette {
    CURRENT.read().map(|palette| *palette).unwrap_or(Palette::ORIGINAL)
}

struct PollState {
    next_check: Instant,
    last_modified: Option<SystemTime>,
}

static POLL: LazyLock<Mutex<PollState>> = LazyLock::new(|| {
    Mutex::new(PollState {
        next_check: Instant::now(),
        last_modified: None,
    })
});

pub fn poll() {
    let Ok(mut state) = POLL.lock() else { return };
    let now = Instant::now();
    if now < state.next_check {
        return;
    }
    state.next_check = now + Duration::from_secs(1);
    let Some(file) = path() else { return };
    let modified = std::fs::metadata(&file).ok().and_then(|meta| meta.modified().ok());
    if modified == state.last_modified {
        return;
    }
    state.last_modified = modified;
    drop(state);
    load();
}

pub fn base() -> Color32 {
    get().base
}

pub fn panel() -> Color32 {
    get().panel
}

pub fn raised() -> Color32 {
    get().raised
}

pub fn floating() -> Color32 {
    get().floating
}

pub fn text() -> Color32 {
    get().text
}

pub fn muted() -> Color32 {
    get().muted
}

pub fn faint() -> Color32 {
    get().faint
}

pub fn accent() -> Color32 {
    get().accent
}

pub fn on_accent() -> Color32 {
    get().on_accent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_hex_colors() {
        assert_eq!(hex("#D0BCFF"), Some(rgb(0xD0BCFF)));
        assert_eq!(hex("d0bcff"), Some(rgb(0xD0BCFF)));
        assert_eq!(hex("nope"), None);
    }

    #[test]
    fn rejects_incomplete_palette() {
        assert!(parse("{\"dark\":{\"surface\":\"#000000\"}}").is_none());
    }

    #[test]
    fn picks_dark_or_light_scheme() {
        let scheme = |surface: &str| {
            format!(
                "{{\"surface\":\"{surface}\",\"surfaceContainerLowest\":\"#000000\",\"surfaceContainerHigh\":\"#000000\",\
                 \"surfaceContainerHighest\":\"#000000\",\"onSurface\":\"#000000\",\"onSurfaceVariant\":\"#000000\",\
                 \"outline\":\"#000000\",\"primary\":\"#000000\",\"onPrimary\":\"#000000\"}}"
            )
        };
        let dark = scheme("#191112");
        let light = scheme("#fff8f7");
        let text = format!("{{\"dark\":{dark},\"light\":{light},\"is_dark\":true}}");
        assert_eq!(parse(&text).map(|palette| palette.base), Some(rgb(0x191112)));
        let text = format!("{{\"dark\":{dark},\"light\":{light},\"is_dark\":false}}");
        assert_eq!(parse(&text).map(|palette| palette.base), Some(rgb(0xfff8f7)));
    }
}
