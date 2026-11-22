#![allow(dead_code)]

use eframe::egui::Color32;
use eframe::egui::ecolor::Hsva;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
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
    pub primary_container: Color32,
    pub on_primary_container: Color32,
    pub secondary: Color32,
    pub on_secondary: Color32,
    pub secondary_container: Color32,
    pub on_secondary_container: Color32,
    pub tertiary: Color32,
    pub on_tertiary: Color32,
    pub tertiary_container: Color32,
    pub on_tertiary_container: Color32,
    pub outline: Color32,
    pub outline_variant: Color32,
    pub surface_variant: Color32,
    pub inverse_surface: Color32,
    pub inverse_on_surface: Color32,
    pub scrim: Color32,
    pub shadow: Color32,
    pub dark: bool,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

fn blend(first: Color32, second: Color32, amount: f32) -> Color32 {
    first.lerp_to_gamma(second, amount.clamp(0.0, 1.0))
}

fn hue_shift(color: Color32, degrees: f32) -> Color32 {
    let mut shifted = Hsva::from(color);
    shifted.h = (shifted.h + degrees / 360.0) % 1.0;
    Color32::from(shifted)
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
        primary_container: rgb(0x6E2E19),
        on_primary_container: rgb(0xFFDBCF),
        secondary: rgb(0xE7B9A5),
        on_secondary: rgb(0x45291E),
        secondary_container: rgb(0x54352A),
        on_secondary_container: rgb(0xFFDBD0),
        tertiary: rgb(0xD9C0E8),
        on_tertiary: rgb(0x3B2D48),
        tertiary_container: rgb(0x4A3A58),
        on_tertiary_container: rgb(0xF5DAFF),
        outline: rgb(0x918F9A),
        outline_variant: rgb(0x46464F),
        surface_variant: rgb(0x46464F),
        inverse_surface: rgb(0xECEDEF),
        inverse_on_surface: rgb(0x1A1B20),
        scrim: rgb(0x000000),
        shadow: rgb(0x000000),
        dark: true,
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
    #[serde(rename = "primaryContainer")]
    primary_container: Option<String>,
    #[serde(rename = "onPrimaryContainer")]
    on_primary_container: Option<String>,
    secondary: Option<String>,
    #[serde(rename = "onSecondary")]
    on_secondary: Option<String>,
    #[serde(rename = "secondaryContainer")]
    secondary_container: Option<String>,
    #[serde(rename = "onSecondaryContainer")]
    on_secondary_container: Option<String>,
    tertiary: Option<String>,
    #[serde(rename = "onTertiary")]
    on_tertiary: Option<String>,
    #[serde(rename = "tertiaryContainer")]
    tertiary_container: Option<String>,
    #[serde(rename = "onTertiaryContainer")]
    on_tertiary_container: Option<String>,
    #[serde(rename = "outlineVariant")]
    outline_variant: Option<String>,
    #[serde(rename = "surfaceVariant")]
    surface_variant: Option<String>,
    #[serde(rename = "inverseSurface")]
    inverse_surface: Option<String>,
    #[serde(rename = "inverseOnSurface")]
    inverse_on_surface: Option<String>,
    scrim: Option<String>,
    shadow: Option<String>,
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

fn role(value: &Option<String>, fallback: Color32) -> Color32 {
    value.as_deref().and_then(hex).unwrap_or(fallback)
}

fn parse(text: &str) -> Option<Palette> {
    let file: HyprYouColors = serde_json::from_str(text).ok()?;
    let scheme = if file.is_dark { &file.dark } else { &file.light };
    let base = hex(&scheme.surface)?;
    let panel = hex(&scheme.surface_container_lowest)?;
    let raised = hex(&scheme.surface_container_high)?;
    let floating = hex(&scheme.surface_container_highest)?;
    let text_color = hex(&scheme.on_surface)?;
    let variant = hex(&scheme.on_surface_variant)?;
    let outline = hex(&scheme.outline)?;
    let primary = hex(&scheme.primary)?;
    let on_primary = hex(&scheme.on_primary)?;
    let primary_container = role(&scheme.primary_container, blend(primary, base, 0.78));
    let secondary = role(&scheme.secondary, blend(primary, variant, 0.45));
    let tertiary = role(&scheme.tertiary, hue_shift(primary, 52.0));
    Some(Palette {
        base,
        panel,
        raised,
        floating,
        text: text_color,
        muted: hex_alpha(&scheme.on_surface_variant, 210)?,
        faint: hex_alpha(&scheme.outline, 150)?,
        accent: primary,
        on_accent: on_primary,
        primary_container,
        on_primary_container: role(&scheme.on_primary_container, blend(primary, text_color, 0.72)),
        secondary,
        on_secondary: role(&scheme.on_secondary, blend(secondary, text_color, 0.72)),
        secondary_container: role(&scheme.secondary_container, blend(secondary, base, 0.78)),
        on_secondary_container: role(&scheme.on_secondary_container, blend(secondary, text_color, 0.72)),
        tertiary,
        on_tertiary: role(&scheme.on_tertiary, blend(tertiary, text_color, 0.72)),
        tertiary_container: role(&scheme.tertiary_container, blend(tertiary, base, 0.78)),
        on_tertiary_container: role(&scheme.on_tertiary_container, blend(tertiary, text_color, 0.72)),
        outline,
        outline_variant: role(&scheme.outline_variant, blend(base, outline, 0.4)),
        surface_variant: role(&scheme.surface_variant, blend(base, variant, 0.28)),
        inverse_surface: role(&scheme.inverse_surface, text_color),
        inverse_on_surface: role(&scheme.inverse_on_surface, base),
        scrim: role(&scheme.scrim, Palette::ORIGINAL.scrim),
        shadow: role(&scheme.shadow, Palette::ORIGINAL.shadow),
        dark: file.is_dark,
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
static GENERATION: AtomicU64 = AtomicU64::new(0);

pub fn poll() -> u64 {
    let now = Instant::now();
    if let Ok(mut state) = POLL.lock()
        && now >= state.next_check
    {
        state.next_check = now + Duration::from_secs(1);
        if let Some(file) = path() {
            let modified = std::fs::metadata(&file).ok().and_then(|meta| meta.modified().ok());
            if modified != state.last_modified {
                state.last_modified = modified;
                load();
                GENERATION.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
    GENERATION.load(Ordering::Relaxed)
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

pub fn primary() -> Color32 {
    get().accent
}

pub fn on_primary() -> Color32 {
    get().on_accent
}

pub fn on_accent() -> Color32 {
    get().on_accent
}

pub fn primary_container() -> Color32 {
    get().primary_container
}

pub fn on_primary_container() -> Color32 {
    get().on_primary_container
}

pub fn secondary() -> Color32 {
    get().secondary
}

pub fn on_secondary() -> Color32 {
    get().on_secondary
}

pub fn secondary_container() -> Color32 {
    get().secondary_container
}

pub fn on_secondary_container() -> Color32 {
    get().on_secondary_container
}

pub fn tertiary() -> Color32 {
    get().tertiary
}

pub fn on_tertiary() -> Color32 {
    get().on_tertiary
}

pub fn tertiary_container() -> Color32 {
    get().tertiary_container
}

pub fn on_tertiary_container() -> Color32 {
    get().on_tertiary_container
}

pub fn outline() -> Color32 {
    get().outline
}

pub fn outline_variant() -> Color32 {
    get().outline_variant
}

pub fn surface_variant() -> Color32 {
    get().surface_variant
}

pub fn inverse_surface() -> Color32 {
    get().inverse_surface
}

pub fn inverse_on_surface() -> Color32 {
    get().inverse_on_surface
}

pub fn scrim() -> Color32 {
    get().scrim
}

pub fn shadow() -> Color32 {
    get().shadow
}

pub fn dark() -> bool {
    get().dark
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

    #[test]
    fn reads_every_expressive_color_role() {
        let source = std::fs::read_to_string(std::env::var("HOME").unwrap() + "/.cache/hypryou/colors/colors.json");
        let Ok(source) = source else { return };
        let Some(palette) = parse(&source) else { return };
        assert_ne!(palette.primary_container, palette.accent);
        assert_ne!(palette.secondary_container, palette.secondary);
        assert_ne!(palette.tertiary, palette.accent);
        assert_ne!(palette.outline_variant, Color32::TRANSPARENT);
    }

    #[test]
    fn derives_missing_roles_from_the_primary_color() {
        let scheme = |surface: &str| {
            format!(
                "{{\"surface\":\"{surface}\",\"surfaceContainerLowest\":\"#000000\",\"surfaceContainerHigh\":\"#000000\",\
                 \"surfaceContainerHighest\":\"#000000\",\"onSurface\":\"#ffffff\",\"onSurfaceVariant\":\"#aaaaaa\",\
                 \"outline\":\"#888888\",\"primary\":\"#bfc1ff\",\"onPrimary\":\"#282a60\"}}"
            )
        };
        let text = format!(
            "{{\"dark\":{},\"light\":{},\"is_dark\":true}}",
            scheme("#131318"),
            scheme("#fbf8ff")
        );
        let palette = parse(&text).unwrap();
        assert_eq!(palette.primary_container, blend(rgb(0xbfc1ff), rgb(0x131318), 0.78));
        assert_eq!(palette.tertiary, hue_shift(rgb(0xbfc1ff), 52.0));
    }
}
