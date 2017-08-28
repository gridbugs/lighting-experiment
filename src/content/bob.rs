use content::Sprite;

pub fn bob_sprite(sprite: Sprite) -> Option<Sprite> {
    use self::Sprite::*;
    match sprite {
        Angler => Some(AnglerBob),
        AnglerBob => Some(Angler),
        _ => None,
    }
}
