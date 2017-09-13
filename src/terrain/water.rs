use cgmath::Vector2;
use entity_store::EntityChange;
use entity_id_allocator::EntityIdAllocator;
use terrain::TerrainMetadata;
use prototype;

const WIDTH: u32 = 40;
const HEIGHT: u32 = 40;

const PLAYER_POSITION: Vector2<f32> = Vector2 { x: (WIDTH / 2) as f32, y: (HEIGHT / 2) as f32 };

pub fn generate(changes: &mut Vec<EntityChange>,
                allocator: &mut EntityIdAllocator) -> TerrainMetadata {

    let mut metadata = TerrainMetadata::default();

    metadata.width = WIDTH;
    metadata.height = HEIGHT;

    let player_id = allocator.allocate();
    metadata.player_id = Some(player_id);
    prototype::angler(changes, player_id, PLAYER_POSITION);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let coord = Vector2::new(j, i).cast();
            prototype::outer_floor(changes, allocator.allocate(), coord);
        }
    }

    metadata
}
