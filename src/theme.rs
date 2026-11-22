use eframe::egui::Color32;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::RwLock;

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
struct PaletteFile {
    base: String,
    panel: String,
    raised: String,
    floating: String,
    text: String,
    muted: String,
    faint: String,
    accent: String,
    on_accent: String,
}

fn hex(value: &str) -> Option<Color32> {
    let digits = value.trim().trim_start_matches('#');
    if digits.len() != 6 {
        return None;
    }
    u32::from_str_radix(digits, 16).ok().map(rgb)
}

fn parse(text: &str) -> Option<Palette> {
    let file: PaletteFile = serde_json::from_str(text).ok()?;
    Some(Palette {
        base: hex(&file.base)?,
        panel: hex(&file.panel)?,
        raised: hex(&file.raised)?,
        floating: hex(&file.floating)?,
        text: hex(&file.text)?,
        muted: hex(&file.muted)?,
        faint: hex(&file.faint)?,
        accent: hex(&file.accent)?,
        on_accent: hex(&file.on_accent)?,
    })
}

fn path() -> Option<PathBuf> {
    let root = match std::env::var_os("XDG_CONFIG_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from(std::env::var_os("HOME")?).join(".config"),
    };
    Some(root.join("materialfast").join("palette.json"))
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
        assert!(parse("{\"base\":\"#000000\"}").is_none());
    }
}
