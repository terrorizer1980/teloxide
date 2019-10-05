use crate::types::Sticker;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct StickerSet {
    pub name: String,
    pub title: String,
    pub is_animated: bool,
    pub contains_masks: bool,
    pub stickers: Vec<Sticker>,
}