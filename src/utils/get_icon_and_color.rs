use dashmap::DashMap;
use once_cell::sync::Lazy;
use ratatui::style::Color;

use crate::data::icons::get_icons;

use super::hex_to_color;

type V = Option<(String, Option<Color>)>;

static CACHE: Lazy<DashMap<String, V>> = Lazy::new(DashMap::new);

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
