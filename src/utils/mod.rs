/* 
    utils -> Just a module for life saving.
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
use std::io::{self, Read};
use std::fs;
use std::rc::Rc;

use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture};
use sdl2::rect::Rect;

use crate::resources::TextureManager;

pub mod vec2;

pub fn read_to_string(path: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub struct SpriteSheet<'l> {
    pub width: u32,
    pub height: u32,

    texture: Rc<Texture<'l>>
}

impl<'l> SpriteSheet<'l> {
    pub fn new(tm: &mut TextureManager<'l, WindowContext>, source: &str, width: u32, height: u32) -> Result<Self, String> {
        Ok(SpriteSheet { texture: tm.load(source)?, width: width, height: height })
    }

    pub fn draw_index(&self, canvas: &mut Canvas<Window>, ix: i32, iy: i32, x: i32, y: i32) -> Result<(), String> {
        let sprite_rect = Rect::new(ix * self.width as i32, iy * self.height as i32, self.width, self.height);
        let dest_rect = Rect::new(x, y, self.width, self.height);

        canvas.copy(&self.texture, sprite_rect, dest_rect)?;
        Ok(())
    }
}

