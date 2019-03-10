/* 
    player.rs -> input and a square on the screen
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


use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::{Texture, Canvas};
use sdl2::video::Window;

use crate::utils::vec2::Vec2f;

pub struct Player<'l> {
    pub pos: Vec2f,
    pub vel: Vec2f,

    texture: Rc<Texture<'l>>,
    rect: Rect
}

impl<'l> Player<'l> {
    pub fn new(texture: Rc<Texture<'l>>, x: f32, y: f32, width: u32, height: u32) -> Self {
        Player { 
            pos: Vec2f::newf(x, y), 
            vel: Vec2f::newi(0, 0),
            texture: texture,
            rect: Rect::new(x as i32, y as i32, width, height) 
        }
    }

    pub fn update(&mut self, limit: f32) {
        self.vel -= self.vel * limit;
        self.pos += self.vel;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        let result = canvas.copy(&self.texture, None, 
            Rect::new(self.pos.x as i32, self.pos.y as i32, 
                      self.rect.width(), self.rect.height()));

        match result {
            Ok(result) => result,
            Err(e) => panic!("Error copying to canvas: {:?}", e)
        }
    }
}