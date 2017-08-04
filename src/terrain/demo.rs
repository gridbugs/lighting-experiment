use cgmath::Vector2;
use entity_store::EntityStoreChange;
use entity_id_allocator::EntityIdAllocator;
use terrain::TerrainMetadata;
use prototype;

pub fn generate(change: &mut EntityStoreChange,
                allocator: &mut EntityIdAllocator) -> TerrainMetadata {

    let mut metadata = TerrainMetadata::default();

    let strings = vec![
        "..................................................",
        "..................................................",
        "........############################..............",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,#..............",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,#..............",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,#..............",
        "........#,,,,,,,,,,,,,,,,,,,,,,,,,,#########......",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,#,,,,,,,#......",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,,,,,,,,,#......",
        "........#,,,,,,,,,,,,,#,,,,,,,,,,,,#,,,,,,,#......",
        "........#######,########################,######...",
        "...................................#,,,,,,,,,,#...",
        "...................................#,,,,,,,,,,#...",
        "...................................#,,,,,,,,,,#...",
        "......................@............,,,,,,,,,,,,...",
        "...................................#,,,,,,,,,,#...",
        "...................................############...",
        "..................................................",
        "..................................................",
    ];

    let mut y = 0;
    for row in strings.iter() {
        let mut x = 0;
        for ch in row.chars() {
            let coord = Vector2::new(x, y);
            match ch {
                '.' => {
                    prototype::outer_floor(change, allocator.allocate(), coord);
                }
                ',' => {
                    prototype::inner_floor(change, allocator.allocate(), coord);
                }
                '@' => {
                    let player_id = allocator.allocate();
                    metadata.player_id = Some(player_id);
                    prototype::angler(change, allocator.allocate(), coord);
                    prototype::outer_floor(change, allocator.allocate(), coord);
                }
                '#' => {
                    prototype::outer_wall(change, allocator.allocate(), coord);
                }
                _ => panic!("Unrecognised character"),
            }
            x += 1;
        }
        y += 1;
    }

    metadata
}
