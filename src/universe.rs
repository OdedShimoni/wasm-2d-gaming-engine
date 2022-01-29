use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::geometry::{Rect, Point};
use crate::player::Player;
use crate::map::GameMap;
extern crate web_sys;

#[derive(Clone, Debug)]
pub enum Tile {
    Player,
    Platform
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    player: Player,
    map: GameMap,
    // camera: Vec<Option<Tile>>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 6;
        let height = 7;

        let mut components = HashMap::new();
        // TODO make sure they don't collide, for now just manually make sure of it
        components.insert(
            Point(2, 2),
            (Rect(10, 1), Tile::Platform)
        );
        components.insert(
            Point(2, 1),
            (Rect(2, 3), Tile::Player)
        );
        Universe {
            width,
            height,
            player: Player::new(),
            map: GameMap::new(width, height)
                .build(components)
        }
    }
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn repr_map(&self) -> *const u8 {
        // converts to a vector represented by u8

        let map_to_return: Vec<u8> = 
            self.map
            .get_tiles()
            .iter()
            .map(|tile| {
                let tile_u8 = match tile {
                    Some(Tile::Player) => 1,
                    Some(Tile::Platform) => 2,
                    _ => 0
                };
                tile_u8
            })
            .collect();

        map_to_return.as_ptr()
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}