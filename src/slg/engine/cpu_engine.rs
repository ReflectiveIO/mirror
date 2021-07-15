use downcast_rs::Downcast;

use crate::slg::engine::tile::{Tile, TileRepository};
use crate::slg::engine::Engine;

pub trait CPURenderEngine: Engine + Downcast {}
impl_downcast!(CPURenderEngine);

pub trait CPUNoTileRenderEngine: CPURenderEngine + Downcast {}
impl_downcast!(CPUNoTileRenderEngine);

pub trait CPUTileRenderEngine: CPURenderEngine + Downcast {
    fn tile_repository(&self) -> &TileRepository;

    fn get_pending_tiles(&mut self, tiles: &Vec<Tile>) {
        self.tile_repository().get_pending_tiles(tiles)
    }

    fn get_not_converged_tiles(&mut self, tiles: &Vec<Tile>) {
        self.tile_repository().get_not_converged_tiles(tiles)
    }

    fn get_converged_tiles(&mut self, tiles: &Vec<Tile>) {
        self.tile_repository().get_converged_tiles(tiles)
    }

    fn get_tile_width(&self) -> u32 { self.tile_repository().tile_width }

    fn get_tile_height(&self) -> u32 { self.tile_repository().tile_height }
}
impl_downcast!(CPUTileRenderEngine);
