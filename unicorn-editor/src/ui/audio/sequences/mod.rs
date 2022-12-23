mod chain_editor;
mod phrase_editor;
mod sfx_editor;
mod song_editor;
mod tracker_edit;
mod tracker_text;

pub(crate) use chain_editor::*;
pub(crate) use phrase_editor::*;
pub(crate) use sfx_editor::*;
pub(crate) use song_editor::*;
pub(crate) use tracker_edit::*;
pub(crate) use tracker_text::*;

use eframe::epaint::Color32;
pub(crate) const DEFAULT_TEXT_COLOR: Color32 = Color32::GRAY;
pub(crate) const SELECTED_BG_COLOR: Color32 = Color32::DARK_BLUE;
pub(crate) const EDITING_BG_COLOR: Color32 = Color32::BLUE;
