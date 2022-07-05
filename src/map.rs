use std::collections::HashMap;
use crate::player::Player;

use crate::geometry::{Rect, Point, get_index};
use crate::universe::Tile;

#[derive(Clone)]
pub struct GameMap {
    width: u32,
    height: u32,
    tiles: Vec<Option<Tile>>,
}

impl GameMap {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        GameMap {
            width,
            height,
            tiles: vec![None; size],
        }
    }
    pub fn build(&self, components: HashMap<Point, (Rect, Tile)>, player: &Player) -> Self {
        // currently depends on the y but implementation is needed
        let mut new_map = self.clone();

        // fill with components
        for (point, (rect, tile)) in components {
            new_map = new_map.fill(point, rect, tile);
        }

        // insert player
        new_map = new_map.fill(
            Point(player.get_x() as u32, player.get_y() as u32), // temp commentation
            // Point(player.get_x() as u32, 1 as u32), // temp
            player.get_shape(),
            Tile::Player
        );

        // let (x, y) = (i / self.width as usize, i / self.height as usize);
        new_map
    }

    fn fill(&self, point: Point, rect: Rect, tile: Tile) -> GameMap {
        let mut new_map = self.clone();
        let component_width = rect.0;
        let component_height = rect.1;

        for y in 0..component_height {
            let y = y + point.1;
            for x in 0..component_width {
                let x = x + point.0;
                let idx = get_index(self.width, y, x);
                new_map.tiles[idx] = Some(tile.clone());
            }
        }
        new_map
    }

    pub fn get_tiles(&self) -> &Vec<Option<Tile>> {
        &self.tiles
    }
}