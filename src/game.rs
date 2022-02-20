use crate::palette::set_draw_color;
use crate::snake::{Point, Snake};
use crate::wasm4;
use fastrand::Rng;

const FRUIT_SPRITE: [u8; 16] = [    
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];
pub struct Game {
    rng: Rng,
    snake: Snake,
    frame_count: u32,
    fruit: Point,
    prev_gamepad: u8,
}

impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        Self {
            snake: Snake::new(),
            frame_count: 0,
            fruit: Point { 
                x: rng.i32(0..20), 
                y: rng.i32(0..20),
            },
            prev_gamepad: 0,
            rng,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;

        self.input();

        if (self.snake.is_dead()) {
            wasm4::trace("SUNK MY BATTLESHIP");
        }

        if self.frame_count % 15 == 0 {
            
            // collision-detection - https://wasm4.org/docs/tutorials/snake/collision-detection
            let dropped_pos = self.snake.update();

            if self.snake.body[0] == self.fruit {
                if let Some(last_pos) = dropped_pos {
                    self.snake.body.push(last_pos);
                }

                self.fruit.x = self.rng.i32(0..20);
                self.fruit.y = self.rng.i32(0..20);
            }

        }

        self.snake.draw();

        set_draw_color(0x4320);
        wasm4::blit(
            &FRUIT_SPRITE,
         self.fruit.x * 8,
         self.fruit.y * 8,
      8,
     8,
      wasm4::BLIT_2BPP,
        );
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad & (gamepad ^ self.prev_gamepad);

        if just_pressed & wasm4::BUTTON_LEFT != 0 {            
            // wasm4::trace("BUTTON_LEFT");
            self.snake.left();
        }        
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            // wasm4::trace("BUTTON_RIGHT");
            self.snake.right();
        }
        if just_pressed & wasm4::BUTTON_UP != 0 {
            // wasm4::trace("BUTTON_UP");
            self.snake.up();
        }        
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            // wasm4::trace("BUTTON_DOWN");
            self.snake.down();
        }
        
        self.prev_gamepad = gamepad;
    }
}