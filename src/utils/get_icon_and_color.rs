use dashmap::DashMap;
use once_cell::sync::Lazy;
use ratatui::style::Color;

use crate::data::icons::get_icons;

type V = Option<(String, Option<Color>)>;

static CACHE: Lazy<DashMap<String, V>> = Lazy::new(DashMap::new);

pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::Rgb(r, g, b))
}

pub fn get_icon_and_color(ext: &str) -> V {
    CACHE
        .entry(ext.to_string())
        .or_insert(
            get_icons().get(ext).and_then(|(icon, hex)| {
                hex_to_color(hex).map(|color| (icon.to_string(), Some(color)))
            }),
        )
        .value()
        .clone()
}
