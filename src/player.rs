use wasm_bindgen::prelude::wasm_bindgen;

use crate::{geometry::Rect, utils::console, map::GameMap};

// TODO only in 1 place
extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Player {
    x: i32,
    y: i32, // TODO turn to unsigned
    shape: Rect,
    vertical_velocity: i32,
    universe_shape: Rect
}

impl Player {
    pub fn new(universe_shape: Rect) -> Self {
        Player {
            x: 1,
            y: 1,
            shape: Rect(3, 6), // TODO get from config
            vertical_velocity: 0,
            universe_shape
        }
    }

    pub fn tick(&mut self) {
        //=================================
        // For Debug
        if self.x > self.universe_shape.0 as i32 || self.y > self.universe_shape.1 as i32 {
            console::log!("({},{})", self.x, self.y);
        }
        //=================================
        
        if self.y > 0 {
            console::log!("Vertical velocity is: {}", self.vertical_velocity); // debug
            self.vertical_velocity += 1;
        }

        if self.y > 0 && self.y - self.vertical_velocity >= 0 {
            let player_height = self.shape.1 as i32;
            let universe_height = self.universe_shape.1 as i32;
            if self.y - self.vertical_velocity + player_height <= universe_height {
                self.y -= self.vertical_velocity;
            }
        } else {
            // is constantly called
            self.vertical_velocity = 0;
            self.y = 0;
        }
    }


    pub fn jump(&mut self) {
        crate::utils::set_panic_hook();
        if self.y == 0 {
            log!("jumping from WASM y");
            self.vertical_velocity -= 7; // TODO get jumpheight from a config file
            self.y = 1;
        } else {
            log!("jumping from WASM y != 0");
        }
    }

    pub fn is_right_above_platform(&self) {

    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_vertical_velocity(&self) -> i32 {
        self.vertical_velocity
    }

    pub fn get_shape(&self) -> Rect {
        self.shape
    }

    pub fn set_y(&mut self,y: i32) {
        self.y = y
    }
    
    pub fn set_x(&mut self,x: i32) {
        let player_width = self.shape.0 as i32;
        let universe_width = self.universe_shape.0 as i32;
        if self.x + x > 0 && self.x + x + player_width <= universe_width {
            log!("Setting X. new x: {}", self.x + x);
            self.x += x
        } else {
            log!("Universe too little");
        }
    }
}