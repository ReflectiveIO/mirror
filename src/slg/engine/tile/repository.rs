use super::Tile;

#[derive(Default)]
pub struct TileRepository {
    pub tile_width: u32,
    pub tile_height: u32,
}

impl TileRepository {
    pub fn get_pending_tiles(&self, tiles: &Vec<Tile>) {
        todo!()
    }

    pub fn get_not_converged_tiles(&self, tiles: &Vec<Tile>) {
        todo!()
    }

    pub fn get_converged_tiles(&self, tiles: &Vec<Tile>) {
        todo!()
    }
}
