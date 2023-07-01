/
/ Wolrd generation methods
impl World{

    // Generates world of just water tiles
    fn generate_water_world(size: WorldGenerationSize, _seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
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

    fn random_tile(rng: &mut StdRng) -> Tile{
        match rng.gen_range(1..5) {
            1 => Tile::Water,
            2 => Tile::Grass,
            3 => Tile::Stone,
            4 => Tile::Sand,
            _ =>  unreachable!()
        }
    }

    // Generates either grass or water tile
    fn random_tile_water_grass(rng: &mut StdRng) -> Tile {
        match rng.gen_range(1..=2) {
            1 => Tile::Water,
            2 => Tile::Grass,
            _ => unreachable!(),
        }
    }

    // Generates world of randomized chunks filled with one type of tile
    fn generate_chunk_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                // Randomize tile used for chunk
                let current_chunk_tile = Self::random_tile(&mut rng);

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
    fn generate_tile_mess_world(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = Self::random_tile(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }
        return chunks;
    }

    // Generates world of just grass and water, using cellular automata.
    fn generate_terrain_grass(size: WorldGenerationSize, seed: u32) -> HashMap<ChunkPos, Chunk> {
        let world_size = Self::get_size_from_type(size);
        let normalization_iterations = 10;
        let mut chunks = HashMap::new();

        let seed = seed_to_byte_array(seed);
        let mut rng = StdRng::from_seed(seed);
        
        for chunk_y in 0..world_size {
            for chunk_x in 0..world_size {
                // Generating individual chunks
                let mut current_chunk = Vec::new();
                for _ in 0..16*16 {
                    let current_chunk_tile = Self::random_tile_water_grass(&mut rng);
                    current_chunk.push(current_chunk_tile.clone());
                }
                // Insert chunk into chunks hashmap
                chunks.insert(ChunkPos {x: chunk_x, y: chunk_y}, Chunk { tiles: current_chunk });
            }
        }

        // Normilize tiles by checking neighbors
        let chunks_clone = chunks.clone();

        for _ in 0..normalization_iterations {

        }

        todo!();
    }
}