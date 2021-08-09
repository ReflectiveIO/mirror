use config::Value;

pub struct Tile;

impl From<Tile> for Value {
    fn from(v: Tile) -> Self { todo!() }
}

#[derive(Default, Deserialize, Serialize)]
pub struct TileCoordinate {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl TileCoordinate {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
