use std::collections::HashMap;

use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::world::*;
use crate::utils::*;

// Wolrd generation methods
impl World{

    // Generates world of just water tiles
    pub fn generate_water_world(size: WorldGenerationSize, _seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = get_size_from_type(size);
        let mut chunks = HashMap::new();
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    current_chunk.push(Tile::Water);
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of randomized chunks filled with one type of tile
    pub fn generate_chunk_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                // Randomize tile used for chunk
                let current_chunk_tile = random_tile(&mut rng);

                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of randomized tiles
    pub fn generate_tile_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = random_tile(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of just grass and water, using cellular automata.
    pub fn generate_terrain_grass(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = random_tile_water_grass(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }

        todo!();

        // // Normilize tiles by checking neighbors
        // let chunks_clone = chunks.clone();

        // for (chunk_pos, chunk) in chunks {
        //     for (index, tile) in chunk.tiles.iter().enumerate() {
        //         match tile {
        //             &Tile::Grass => (),
        //             _ => continue,
        //         };
        //         let mut neighbors = 0;
                
        //         // Top neighbor
                
        //     }
        // }
    }
}