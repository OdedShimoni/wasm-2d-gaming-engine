use std::collections::HashMap;
use crate::utils::console::log;

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
    pub fn build(&self, components: HashMap<Point, (Rect, Tile)>) -> Self {
        // currently depends on the y but implementation is needed
        let mut new_map = self.clone();

        // fill with components
        for (point, (rect, tile)) in components {
            log!("{:?}", (&point, (&rect, &tile)));
            let component_width = rect.0;
            let component_height = rect.1;

            for y in 0..component_height {
                let y = y + point.1;
                for x in 0..component_width {
                    let x = x + point.0;
                    let idx = get_index(self.width, y - 1, x - 1);
                    new_map.tiles[idx] = Some(tile.clone());
                }
            }
        }

        // let (x, y) = (i / self.width as usize, i / self.height as usize);
        new_map
    }

    pub fn get_tiles(&self) -> &Vec<Option<Tile>> {
        &self.tiles
    }
}