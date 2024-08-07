use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;

use crate::utils::*;
use crate::world::*;

// Wolrd generation methods
impl World {
    // Generates world of just water tiles
    pub fn generate_water_world(size: WorldGenerationSize, _seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = size as i32;
        let mut chunks = HashMap::new();
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16 * 16 {
                    current_chunk.push(Tile::Water);
                }
                // Insert chunk into chunks hashmap
                chunks.insert(
                    ChunkPos {
                        x: chunk_x,
                        y: chunk_y,
                    },
                    Chunk {
                        tiles: current_chunk,
                    },
                );
            }
        }
        return chunks;
    }

    // Generates world of randomized chunks filled with one type of tile
    pub fn generate_chunk_mess_world(
        size: WorldGenerationSize,
        seed: u32,
    ) -> HashMap<ChunkPos, Chunk> {
        let world_size = size as i32;
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);

        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                // Randomize tile used for chunk
                let current_chunk_tile = random_tile(&mut rng);

                let mut current_chunk = Vec::new();
                for _ in 0..16 * 16 {
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(
                    ChunkPos {
                        x: chunk_x,
                        y: chunk_y,
                    },
                    Chunk {
                        tiles: current_chunk,
                    },
                );
            }
        }
        return chunks;
    }

    // Generates world of randomized tiles
    pub fn generate_tile_mess_world(
        size: WorldGenerationSize,
        seed: u32,
    ) -> HashMap<ChunkPos, Chunk> {
        let world_size = size as i32;
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);

        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16 * 16 {
                    let current_chunk_tile = random_tile(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(
                    ChunkPos {
                        x: chunk_x,
                        y: chunk_y,
                    },
                    Chunk {
                        tiles: current_chunk,
                    },
                );
            }
        }
        return chunks;
    }

    pub fn generate_perlin_noise_world(
        size: WorldGenerationSize,
        island_size: WorldIslandSize,
        seed: u32,
    ) -> HashMap<ChunkPos, Chunk> {
        let fbm = Fbm::<Perlin>::new(seed);
        let size = size as i32;
        let mut map = HashMap::new();

        let plane = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(size as usize * 16, size as usize * 16)
            .set_x_bounds(
                -size as f64 / island_size as i64 as f64,
                size as f64 / island_size as i64 as f64,
            )
            .set_y_bounds(
                -size as f64 / island_size as i64 as f64,
                size as f64 / island_size as i64 as f64,
            )
            .build();

        let terrain_height_offset = 0.4;

        let sand_level = 0.29 + terrain_height_offset;
        let grass_level = 0.8 + terrain_height_offset;
        let stone_level = 1.1 + terrain_height_offset; // Change back to 1.0
        let dark_stone_level = 1.5 + terrain_height_offset;
        let snow_level = 2.0 + terrain_height_offset;

        let shallow_water_level = 0.0 + terrain_height_offset;
        let water_level = -0.50 + shallow_water_level;
        let deep_water_level = -1.55 + terrain_height_offset;

        let max_height = snow_level;
        let height_scale_factor = 1.9;

        for chunk_y in 0..size {
            for chunk_x in 0..size {
                let mut chunk = Chunk { tiles: vec![] };
                for y in 0..16 {
                    for x in 0..16 {
                        let pixel = plane
                            .get_value({ chunk_x * 16 + x } as usize, { chunk_y * 16 + y }
                                as usize);
                        let pixel = { height_scale_factor * pixel };
                        let tile = match pixel {
                            // Land
                            _ if pixel > max_height => Tile::Snow, // Handles for anything above the max height, makes it dark stone
                            _ if pixel >= dark_stone_level && pixel < max_height => Tile::Snow,
                            _ if pixel >= stone_level && pixel < dark_stone_level => {
                                Tile::DarkStone
                            }
                            _ if pixel >= grass_level && pixel < stone_level => Tile::Stone,
                            _ if pixel >= sand_level && pixel < grass_level => Tile::Grass,
                            _ if pixel >= shallow_water_level && pixel < sand_level => Tile::Sand,

                            // Water
                            _ if pixel < shallow_water_level && pixel >= water_level => {
                                Tile::ShallowWater
                            }
                            _ if pixel < water_level && pixel >= deep_water_level => Tile::Water,
                            _ => Tile::DeepWater,
                        };
                        chunk.tiles.push(tile);
                    }
                }
                map.insert(ChunkPos { x: chunk_x, y: chunk_y}, chunk);
            }
        }
        map
    }
}
