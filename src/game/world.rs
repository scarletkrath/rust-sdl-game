/* 
    game/world.rs -> drawing tilemaps
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
use std::error::Error;
use sdl2::render::{Canvas};
use sdl2::video::{Window, WindowContext};

use crate::utils::{self, SpriteSheet};
use crate::utils::vec2::Vec2f;
use crate::resources::TextureManager;

#[derive(Debug)]
pub enum Tile {
    Air,
    Ground,
    Block
}

pub fn get_tile_id(c: char) -> Tile {
    match c {
        '#' => Tile::Ground,
        '.' => Tile::Air,
        '-' => Tile::Block,

        _ => Tile::Air
    }
}

pub struct Map<'l> {
    pub tiles: SpriteSheet<'l>,
    pub width: u32,
    pub height: u32,

    // TODO: use the spawn pos
    _spawn: Vec2f,
    tokens: Vec<char>
}

impl<'l> Map<'l> {
    pub fn new(fsrc: &'l str, tsrc: &'l str, tm: &mut TextureManager<'l, WindowContext>) -> Result<Self, Box<dyn Error>> {
        let src = utils::read_to_string(fsrc)?;

        let split: Vec<&str> = src.split_whitespace().collect();

        let map_width = split[0].parse::<u32>()?;
        let map_height = split[1].parse::<u32>()?;

        let psx = split[2].parse::<u32>()?;
        let psy = split[3].parse::<u32>()?;

        let map: Vec<&str> = src.split("MAP").collect();
        let tokens: Vec<char> = map[1].trim().replace("\n", "").replace("\r", "").chars().collect();

        // TODO: Use this!
        let _spawn = Vec2f::newi(psx as i32, psy as i32);

        Ok(Map { tiles: SpriteSheet::new(tm, tsrc, 32, 32)?, width: map_width, height: map_height, _spawn: _spawn, tokens: tokens })
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn Error>> {
        for y in 0..self.height {
            for x in 0..self.width {
                let token = get_tile_id(self.tokens[x as usize + y as usize * self.width as usize]);

                let (tx, ty) = match token {
                    Tile::Air => (30, 0),
                    Tile::Ground => (0, 0),
                    Tile::Block => (0, 1)
                };

                self.tiles.draw_index(canvas, tx, ty, (x * self.tiles.width) as i32, (y * self.tiles.height) as i32)?;
            }
        }
        Ok(())
    }
}


