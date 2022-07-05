use std::collections::HashMap;

use wasm_bindgen::prelude::wasm_bindgen;
use crate::geometry::{Rect, Point};
use crate::player::Player;
use crate::map::GameMap;
use crate::utils::console::log;
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
        let width = 100;
        let height = 50;

        let mut components = HashMap::new();
        // TODO make sure they don't collide, for now just manually make sure of it
        components.insert(
            Point(2, 2),
            (Rect(1, 1), Tile::Platform)
        );
        let player = Player::new(Rect(width, height));
        Universe {
            width,
            height,
            player,
            map: GameMap::new(width, height)
                .build(
                    components,
                    &player
                )
        }
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

    pub fn dispatch(&mut self, action: String) {
        // TODO action not String
        match action.as_str() {
            "PLAYER_MOVE_LEFT" => self.player.set_x(-3), // TODO get stepsize from config
            "PLAYER_MOVE_RIGHT" => self.player.set_x(3), // TODO get stepsize from config
            "PLAYER_JUMP" => self.player.jump(),
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        // TODO VERY BAD BECAUSE IT SIMPLY CREATES NEW COMPONENTS INSTEAD OF TAKING THE EXISTING ONES
        let mut components = HashMap::new();
        components.insert(
            Point(2, 2),
            (Rect(10, 1), Tile::Platform)
        );

        self.map = GameMap::new(self.width, self.height)
                .build(
                    components,
                    &self.player
                );
        self.player.tick();
    }
}