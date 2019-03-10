/* 
    game.rs -> the game loop and other stuff
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

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::video;
use sdl2::render;
use sdl2::image::InitFlag;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::event::Event;

use crate::resources::TextureManager;
use crate::player::Player;

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub fps_limit: u32,

    // sdl: sdl2::Sdl,
    events: sdl2::EventPump,
    canvas: render::Canvas<video::Window>
}

pub fn key_pressed(k: Keycode, e: &sdl2::EventPump) -> bool {
    let scancode = Scancode::from_keycode(k);
    if let Some(s) = scancode {
        return e.keyboard_state().is_scancode_pressed(s);
    } else {
        panic!("Keycode does not exist: {:?}", scancode)
    }
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

        game
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();
        self.canvas.present();

        let _image_context = sdl2::image::init(InitFlag::PNG)?;
        let texture_creator = self.canvas.texture_creator();
        let mut texture_manager = TextureManager::new(&texture_creator);

        let mut player = Player::new(texture_manager.load("res/sprites/player.png")?, 100.0, 100.0, 16, 16);

        'running: loop {
            // Handle input
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => { break 'running; },
                    _ => {}
                }
            }

            if key_pressed(Keycode::A, &self.events) { player.vel.x += -1.2 }
            if key_pressed(Keycode::D, &self.events) { player.vel.x +=  1.2 }
            if key_pressed(Keycode::W, &self.events) { player.vel.y += -1.2 }
            if key_pressed(Keycode::S, &self.events) { player.vel.y +=  1.2 }
            
            player.update(0.15);

            self.canvas.clear();
            // Drawing code goes here!
            
            player.draw(&mut self.canvas);
            self.canvas.present();

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.fps_limit));
        }

        Ok(())
    }
}
