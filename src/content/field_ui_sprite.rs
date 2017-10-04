#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldUiSprite {
    _Num,
}

pub const NUM_FIELD_UI_SPRITES: usize = FieldUiSprite::_Num as usize;
