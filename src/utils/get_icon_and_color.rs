use dashmap::{DashMap, Entry};
use once_cell::sync::Lazy;
use ratatui::style::Color;

use crate::data::icons::get_icons;

use super::hex_to_color;

static CACHE: Lazy<DashMap<&str, Option<(&str, Color)>>> = Lazy::new(DashMap::new);

pub fn get_icon_and_color(extention: &str) -> Option<&(&str, Color)> {
    match CACHE.entry(extention) {
        Entry::Occupied(entry) => entry.get().as_ref(),
        Entry::Vacant(entry) => match get_icons().get(extention) {
            Some((icon, hex_color)) => match hex_to_color(hex_color) {
                Some(color) => (*entry.insert(Some((icon, color)))).as_ref(),
                None => entry.insert(None),
            },
            None => entry.insert(None),
        },
    }
}
