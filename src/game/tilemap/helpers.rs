use crate::game::CHUNK_SIZE;
use crate::prelude::{ChunkPos, TilePos, WorldData};

pub fn left_of(chunk_pos: &ChunkPos, tile_pos: &TilePos) -> (ChunkPos, TilePos) {
    if tile_pos.x == 0 {
        (
            ChunkPos::new(chunk_pos.x - 1, chunk_pos.y),
            TilePos::new(CHUNK_SIZE as u32 - 1, tile_pos.y),
        )
    } else {
        (chunk_pos.clone(), TilePos::new(tile_pos.x - 1, tile_pos.y))
    }
}

pub fn right_of(chunk_pos: &ChunkPos, tile_pos: &TilePos) -> (ChunkPos, TilePos) {
    if tile_pos.x >= CHUNK_SIZE as u32 - 1 {
        (
            ChunkPos::new(chunk_pos.x + 1, chunk_pos.y),
            TilePos::new(0, tile_pos.y),
        )
    } else {
        (chunk_pos.clone(), TilePos::new(tile_pos.x + 1, tile_pos.y))
    }
}

pub fn below_of(chunk_pos: &ChunkPos, tile_pos: &TilePos) -> (ChunkPos, TilePos) {
    if tile_pos.y == 0 {
        (
            ChunkPos::new(chunk_pos.x, chunk_pos.y - 1),
            TilePos::new(tile_pos.x, CHUNK_SIZE as u32 - 1),
        )
    } else {
        (chunk_pos.clone(), TilePos::new(tile_pos.x, tile_pos.y - 1))
    }
}

pub fn top_of(chunk_pos: &ChunkPos, tile_pos: &TilePos) -> (ChunkPos, TilePos) {
    if tile_pos.y >= CHUNK_SIZE as u32 - 1 {
        (
            ChunkPos::new(chunk_pos.x, chunk_pos.y + 1),
            TilePos::new(tile_pos.x, 0),
        )
    } else {
        (chunk_pos.clone(), TilePos::new(tile_pos.x, tile_pos.y + 1))
    }
}

// 00 01 02 03
// 04 05 06 07
// 08 09 10 11
// 12 13 14 15
pub fn determine_texture_index(
    pos: &TilePos,
    chunk_pos: &ChunkPos,
    world_data: &WorldData,
) -> usize {
    let chunk = world_data.chunks.get(chunk_pos).unwrap();
    let up = if pos.y < CHUNK_SIZE as u32 - 1 {
        chunk.at(pos.x, pos.y + 1).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPos::new(chunk_pos.x, chunk_pos.y + 1));
        if let Some(chunk) = chunk {
            chunk.at(pos.x, 0).is_tilled
        } else {
            false
        }
    };
    let down = if pos.y > 0 {
        chunk.at(pos.x, pos.y - 1).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPos::new(chunk_pos.x, chunk_pos.y - 1));
        if let Some(chunk) = chunk {
            chunk.at(pos.x, CHUNK_SIZE as u32 - 1).is_tilled
        } else {
            false
        }
    };
    let right = if pos.x < CHUNK_SIZE as u32 - 1 {
        chunk.at(pos.x + 1, pos.y).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPos::new(chunk_pos.x + 1, chunk_pos.y));
        if let Some(chunk) = chunk {
            chunk.at(0, pos.y).is_tilled
        } else {
            false
        }
    };
    let left = if pos.x > 0 {
        chunk.at(pos.x - 1, pos.y).is_tilled
    } else {
        let chunk = world_data
            .chunks
            .get(&ChunkPos::new(chunk_pos.x - 1, chunk_pos.y));
        if let Some(chunk) = chunk {
            chunk.at(CHUNK_SIZE as u32 - 1, pos.y).is_tilled
        } else {
            false
        }
    };

    if up {
        if down {
            if left {
                if right {
                    10
                } else {
                    11
                }
            } else if right {
                9
            } else {
                8
            }
        } else if left {
            if right {
                14
            } else {
                15
            }
        } else {
            if right {
                13
            } else {
                12
            }
        }
    } else if down {
        if left {
            if right {
                6
            } else {
                7
            }
        } else if right {
            5
        } else {
            4
        }
    } else if left {
        if right {
            2
        } else {
            3
        }
    } else {
        if right {
            1
        } else {
            0
        }
    }
}
