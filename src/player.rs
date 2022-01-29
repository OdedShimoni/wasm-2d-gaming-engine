use wasm_bindgen::prelude::wasm_bindgen;

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
    vertical_velocity: i32
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Player {
            x: 0, // TODO middle not 0
            y: 10,
            vertical_velocity: 0
        }
    }

    pub fn tick(&mut self) {
        if self.y > 0 {
            self.vertical_velocity += 1;
        }

        if self.y > 0 {
            self.y -= self.vertical_velocity;
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
            self.vertical_velocity -= 15; // TODO get 15 from a config file
            self.y = 1;
        } else {
            log!("jumping from WASM y != 0");
        }
    }

    #[wasm_bindgen(getter)]
    pub fn get_y(&self) -> i32 {
        self.y
    }

    #[wasm_bindgen(getter)]
    pub fn get_x(&self) -> i32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn get_vertical_velocity(&self) -> i32 {
        self.vertical_velocity
    }

    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self,y: i32) {
        self.y = y
    }
    
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self,x: i32) {
        self.x += x
    }
}