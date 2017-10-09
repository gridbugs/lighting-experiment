use cgmath::Vector2;
use entity_store::EntityChange;
use entity_id_allocator::EntityIdAllocator;
use terrain::TerrainMetadata;
use prototype;

pub fn generate(changes: &mut Vec<EntityChange>,
                allocator: &mut EntityIdAllocator) -> TerrainMetadata {

    let mut metadata = TerrainMetadata::default();

    let strings = vec![
        "..................................................",
        "..................................................",
        "..........######o###################..............",
        ".........##,,,,,,%,,,,%m,,,,,,,,,,,#..............",
        "........##,,,l,,,%,,,,%,,,,,,,,,,,,#..............",
        "........#,,,,,,,,%,,,s%%%%%%%%,,,,,#..............",
        "........#,,,,,,,c,,,,@+,,,,,,,,,,,,####o####......",
        "........o,,,,,,,,,,,,s%,,,,,,,,,,,,%,,,,,,,#......",
        "........#,,,,,,,,,,,~~%,,,,,,,,,,,,+,,,,,,,#......",
        "........#,,,,,,,,,,~~~%n,,,,,,,,,,,%,,,,,,,#......",
        "........#######*o############o######%%%%+%%####...",
        "...................................#,,,,,,,,,,#...",
        "...................................o,,,,,,,,,,#...",
        "...................................#,,,,,,,,,,#...",
        "...................................*,,,,,,,,,,*...",
        "...................................#,,,,,,,,,,#...",
        "...................................####o#######...",
        "..................................................",
        "..................................................",
    ];

    metadata.width = strings[0].len() as u32;
    metadata.height = strings.len() as u32;

    let mut y = 0;
    for row in strings.iter() {
        let mut x = 0;
        for ch in row.chars() {
            let coord = Vector2::new(x, y);
            match ch {
                '.' => {
                    prototype::outer_floor(changes, allocator.allocate(), coord);
                }
                ',' => {
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                '~' => {
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                    prototype::inner_water(changes, allocator.allocate(), coord);
                }
                '@' => {
                    let player_id = allocator.allocate();
                    metadata.player_id = Some(player_id);
                    prototype::angler(changes, player_id, coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'c' => {
                    prototype::crab(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'C' => {
                    prototype::crab(changes, allocator.allocate(), coord);
                    prototype::inner_water(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                's' => {
                    prototype::snail(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'S' => {
                    prototype::snail(changes, allocator.allocate(), coord);
                    prototype::inner_water(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'l' => {
                    prototype::light(changes, allocator.allocate(), coord, [1.0, 1.0, 1.0]);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'm' => {
                    prototype::light(changes, allocator.allocate(), coord, [1.0, 0.0, 0.0]);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'n' => {
                    prototype::light(changes, allocator.allocate(), coord, [0.0, 1.0, 0.0]);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                '%' => {
                    prototype::inner_wall(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                '#' => {
                    prototype::outer_wall(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                '+' => {
                    prototype::inner_door(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                'o' => {
                    prototype::outer_wall(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                    prototype::window(changes, allocator.allocate(), coord);
                }
                '*' => {
                    prototype::outer_door(changes, allocator.allocate(), coord);
                    prototype::inner_floor(changes, allocator.allocate(), coord);
                }
                _ => panic!("Unrecognised character"),
            }
            x += 1;
        }
        y += 1;
    }

    metadata
}
