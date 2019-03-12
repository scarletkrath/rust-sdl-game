/* 
    game -> the game loop and other stuff
    Copyright (C) 2019 - scarletkrath

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
pub mod player;
pub mod world;

use std::time::Instant;
use std::error::Error;

use sdl2::pixels::Color;
use sdl2::video::{Window};
use sdl2::render::Canvas;
use sdl2::image::InitFlag;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::event::Event;

use crate::resources::TextureManager;
use player::Player;

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub fps_limit: u32,

    // sdl: sdl2::Sdl,
    events: sdl2::EventPump,
    canvas: Canvas<Window>
}

impl Game {
    pub fn new(title: &'static str, width: u32, height: u32, fps_limit: u32) -> Result<Game, String> {
        let sdl = sdl2::init()?;
        let vid_s = sdl.video()?;
        let events = sdl.event_pump()?;

        let window = vid_s.window(title, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let game = Ok(Game {
            width: width,
            height: height,
            fps_limit: fps_limit,

            // sdl: sdl,
            events: events,
            canvas: canvas
        });
        // TODO: Find a way to make Player a member of the Game struct. 

        // Player -> texture
        // texture -> TextureManager
        // TextureManager -> TextureCreator (&canvas.texture_creator())

        game
    }

    pub fn key_pressed(&self, k: Keycode) -> bool {
        let scancode = Scancode::from_keycode(k);
        if let Some(s) = scancode {
            return self.events.keyboard_state().is_scancode_pressed(s);
        } else {
            panic!("Keycode does not exist: {:?}", scancode)
        }
    }

    pub fn fixed_update(&self, player: &mut Player) {
        if self.key_pressed(Keycode::A) { player.vel.x += -1.0 }
        if self.key_pressed(Keycode::D) { player.vel.x +=  1.0 }
        if self.key_pressed(Keycode::W) { player.vel.y += -1.0 }
        if self.key_pressed(Keycode::S) { player.vel.y +=  1.0 }
            
        player.update(0.20);
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();
        
        // let mut t = 0.0;
        let mut current_time = Instant::now();
        let mut dt = 0.0;

        let _img = sdl2::image::init(InitFlag::PNG)?;
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);
        let mut player = Player::new(texture_manager.load("res/sprites/player.png")?, 100.0, 100.0, 16, 16);

        let map = world::Map::new("res/worlds/1.lvl", "res/spritesheets/day.png", &mut texture_manager)?;

        'running: loop {
            let new_time = Instant::now();
            // You have to cast the nanoseconds first and THEN divide them, otherwise integer division truncates the time.
            let frame_time = (new_time.duration_since(current_time).subsec_nanos()) as f32 / 1_000_000_000.0;
            current_time = new_time;
            dt += frame_time;

            while dt >= 0.01 {
                // println!("Step, dt = {}", dt);
                self.fixed_update(&mut player);
                dt -= 0.01;
                // t += 0.1;
            }
            
            if !self.handle_input() { break 'running; }
            
            self.canvas.clear();
            // Start drawing

            map.draw(&mut self.canvas)?;
            player.draw(&mut self.canvas)?;

            // End drawing
            self.canvas.present();
        }

        Ok(())
    }

    pub fn handle_input(&mut self) -> bool {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { return false; },
                _ => {}
            }
        }
        true
    }
}
